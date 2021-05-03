use regex::Regex;
use std::io::{self, Read};
extern crate regex;

struct ConsoleState {
    acc: i64,
    pc: i64,
    bl: Vec<bool>,
}

fn run_prog(prog: &Vec<&str>, state: &mut ConsoleState) -> u8 {
    let re = Regex::new(r"(\w*)\s(\S*)").unwrap();
    while (state.pc as usize) < prog.len() {
        if state.bl[state.pc as usize] {
            return 2;
        }
        let line = prog[state.pc as usize];
        let instr = re.captures(line).unwrap();
        let op = &instr[1];
        let arg1 = &instr[2];
        match op {
            "nop" => {}
            "acc" => state.acc += arg1.parse::<i64>().unwrap(),
            "jmp" => {
                state.pc += arg1.parse::<i64>().unwrap();
                continue;
            }
            "exit" => return arg1.parse::<u8>().unwrap(),
            _ => println!("Unrecognized operation"),
        }
        state.bl[state.pc as usize] = true;
        state.pc += 1;
    }
    return 255;
}

fn exec(prog: &Vec<&str>) -> u8 {
    let mut state = ConsoleState {
        acc: 0,
        pc: 0,
        bl: vec![],
    };
    state.bl.resize_with(prog.len(), || false);
    let ret = run_prog(&prog, &mut state);
    println!("exit({}) Accumulator {}", ret, state.acc);
    return ret;
}

fn find_err(prog: &Vec<&str>) {
    let re = Regex::new(r"(\w*)\s(\S*)").unwrap();
    for (i, v) in prog.iter().enumerate() {
        let instr = re.captures(v).unwrap();
        let op = &instr[1];
        let arg1 = &instr[2];
        let mdfy: String;
        let mut mod_prog = prog.clone();
        match op {
            "nop" => {
                mdfy = format!("{} {}", "jmp", arg1);
            }
            "jmp" => {
                mdfy = format!("{} {}", "nop", arg1);
            }
            _ => continue,
        };
        mod_prog[i] = &mdfy;
        if exec(&mod_prog) == 0 {
            return;
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");
    let mut prog: Vec<&str> = buffer.split("\n").collect();
    prog.retain(|&s| s != "");
    exec(&prog);
    println!("{:?}", &prog);
    find_err(&prog);
}
