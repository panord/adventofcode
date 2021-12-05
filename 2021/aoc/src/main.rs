use anyhow::Context;
use clap::{value_t, App, Arg, ArgMatches};
use either::{Left, Right};
use std::fs::File;
use std::io::{stdin, stdout};
use String;

fn binary_diagnostic<X, Y>(input: X, out: &mut Y)
where
    X: std::io::Read,
    Y: std::io::Write,
{
    let vals: Vec<String> = aoc::from_lines(input);
    let (gamma, epsilon) = aoc::binary_diagnostic(&vals.as_slice());

    out.write(format!("Gamma: {}\nEpsilon: {}", gamma, epsilon).as_ref())
        .expect("Failed to write result");
}

fn dive<X, Y>(input: X, out: &mut Y, aim: bool)
where
    X: std::io::Read,
    Y: std::io::Write,
{
    let vals: Vec<aoc::Move> = aoc::from_lines(input);
    let orig = aoc::Pos { x: 0, y: 0, aim: 0 };
    let dest = if aim {
        aoc::dive_aim(orig, vals)
    } else {
        aoc::dive(orig, vals)
    };

    out.write(format!("Position: {:?}\n", dest).as_ref())
        .expect("Failed to write result");

    out.write(format!("x * y: {}\n", dest.x * dest.y).as_ref())
        .expect("Failed to write result");
}

fn sonar_sweep<X, Y>(input: X, out: &mut Y, n: usize)
where
    X: std::io::Read,
    Y: std::io::Write,
{
    let mut vals: Vec<i64> = aoc::from_lines(input);
    let cnt = aoc::sonar_sweep(&mut vals, n);

    out.write(format!("Growing: {}\n", cnt).as_ref())
        .expect("Failed to write result");
}

fn do_solve<X, Y>(assign: u8, input: X, output: &mut Y)
where
    X: std::io::Read,
    Y: std::io::Write,
{
    match assign {
        1 => sonar_sweep(input, output, 1),
        2 => sonar_sweep(input, output, 3),
        3 => dive(input, output, false),
        4 => dive(input, output, true),
        5 => binary_diagnostic(input, output),
        _ => println!("Unsupported assignment {}", assign),
    };
}

pub fn solve(args: &ArgMatches) {
    let a = value_t!(args.value_of("assignment"), u8)
        .context("Could not parse assignment number")
        .unwrap();

    // crate for fileopen or stdout/stdin
    let input = value_t!(args.value_of("file"), String)
        .context("Failed to parse file name")
        .and_then(|fname| Ok(File::open(fname).expect("failed to open file")))
        .and_then(|f| Ok(Right(std::io::BufReader::new(f))))
        .unwrap_or(Left(stdin()));

    let mut output = value_t!(args.value_of("out"), String)
        .context("Failed to parse file name")
        .and_then(|fname| Ok(File::open(fname).expect("failed to open file")))
        .and_then(|f| Ok(Right(std::io::BufWriter::new(f))))
        .unwrap_or(Left(stdout()));

    do_solve(a, input, &mut output);
}

fn main() {
    let app = App::new("Advent of code solver")
        .version("latest")
        .author("Patrik Lundgren")
        .arg(
            Arg::with_name("assignment")
                .short("a")
                .long("assignment")
                .help("Assignment to solve")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Set input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .help("Set output file")
                .takes_value(true),
        );

    let matches = app
        .clone()
        .get_matches_safe()
        .context("Failed to parse args")
        .unwrap();

    solve(&matches);
}
