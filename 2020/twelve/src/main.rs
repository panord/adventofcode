use std::error::Error;
use std::fmt;
use std::io::{self, Read};
use std::iter::FromIterator;
use std::vec::Vec;

extern crate regex;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct ParseError {}

impl Error for ParseError {
    fn description(&self) -> &str {
        "Failed to parse input"
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse input")
    }
}

#[derive(Debug)]
enum Action {
    NORTH,
    SOUTH,
    EAST,
    WEST,

    RIGHT,
    LEFT,

    FORWARD,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: i64,
}

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Action::NORTH),
            "S" => Ok(Action::SOUTH),
            "E" => Ok(Action::EAST),
            "W" => Ok(Action::WEST),
            "R" => Ok(Action::RIGHT),
            "L" => Ok(Action::LEFT),
            "F" => Ok(Action::FORWARD),
            _ => Err(ParseError {}),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\w)(\w*)").unwrap();
        let captres = re.captures(s).unwrap();

        let act = captres[1].parse::<Action>();
        let val = captres[2].parse::<i64>();

        if act.is_err() || val.is_err() {
            return Err(ParseError {});
        }

        return Ok(Instruction {
            action: act.unwrap(),
            value: val.unwrap(),
        });
    }
}

#[derive(Debug)]
struct Ship {
    x: i64,
    y: i64,
    dir: f64,
}

#[derive(Debug)]
struct WayPoint {
    dx: i64,
    dy: i64,
}

#[derive(Debug)]
struct Voyage {
    x: i64,
    y: i64,
    wp: WayPoint,
}

impl Voyage {
    fn do_instr(&mut self, i: &Instruction) {
        match &i.action {
            Action::NORTH => self.wp.dy += i.value,
            Action::SOUTH => self.wp.dy -= i.value,
            Action::EAST => self.wp.dx += i.value,
            Action::WEST => self.wp.dx -= i.value,

            Action::RIGHT => {
                let d = -i.value as f64;
                let r = d.to_radians();
                let cosd = r.cos() as i64;
                let sind = r.sin() as i64;
                let nx = self.wp.dx * cosd - self.wp.dy * sind;
                let ny = self.wp.dx * sind + self.wp.dy * cosd;
                self.wp.dx = nx;
                self.wp.dy = ny;
            }
            Action::LEFT => {
                let d = i.value as f64;
                let r = d.to_radians();
                let cosd = r.cos() as i64;
                let sind = r.sin() as i64;
                let nx = self.wp.dx * cosd - self.wp.dy * sind;
                let ny = self.wp.dx * sind + self.wp.dy * cosd;
                self.wp.dx = nx;
                self.wp.dy = ny;
            }
            Action::FORWARD => {
                for _ in 0..i.value {
                    self.x += self.wp.dx;
                    self.y += self.wp.dy;
                }
            }
        }
    }
}

impl Ship {
    fn do_instr(&mut self, i: &Instruction) {
        match &i.action {
            Action::NORTH => self.y += i.value,
            Action::SOUTH => self.y -= i.value,
            Action::EAST => self.x += i.value,
            Action::WEST => self.x -= i.value,

            Action::RIGHT => self.dir -= i.value as f64,
            Action::LEFT => self.dir += i.value as f64,
            Action::FORWARD => {
                self.x += i.value * self.dir.to_radians().cos() as i64;
                self.y += i.value * self.dir.to_radians().sin() as i64;
            }
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");

    let inst: Vec<Instruction> = buffer
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|istr| istr.parse::<Instruction>().unwrap())
        .collect();

    let mut s = Ship {
        x: 0,
        y: 0,
        dir: 0.0,
    };

    let mut v = Voyage {
        x: 0,
        y: 0,
        wp: WayPoint { dx: 10, dy: 1 },
    };

    for i in inst {
        s.do_instr(&i);
        v.do_instr(&i);
    }
    println!("Manhattan {}", s.x.abs() + s.y.abs());
    println!("Manhattan {}", v.x.abs() + v.y.abs());
}
