use std::error::Error;
use std::fmt;
use std::io::{self, Read};
use std::iter::FromIterator;
use std::vec::Vec;

extern crate regex;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Bus {
    id: u64,
    active: bool,
}

#[derive(Debug)]
struct ParseError {}

impl FromStr for Bus {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Bus {
                id: u64::MAX,
                active: false,
            }),
            _ => Ok(Bus {
                id: s.parse::<u64>().unwrap(),
                active: true,
            }),
        }
    }
}

fn earliest(busses: &Vec<Bus>, depart: u64) {
    let mut eid = u64::MAX;
    let mut min = u64::MAX;
    for b in busses {
        let k = depart / b.id + 1;
        let wait = k * b.id - depart;
        if k > 0 && wait < min {
            eid = b.id;
            min = wait;
        }
    }
    println!("{} * {} =  {}", eid, min, eid * min);
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inverse(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some(x % n)
    } else {
        None
    }
}

fn chinese_remainder(a: &Vec<i64>, n: &Vec<i64>) -> i64 {
    /* chinese remainder
     *      x = for i in = i..k
     *          ai * Mi * Ni
     *  Where
     *      x  := solution
     *      ai := Offsets from t (index of bus)
     *      Mi := Some integer?
     *      Ni := Ni = N / ni where { ni := busid (modulus) and N || i..k nk
     *      https://rosettacode.org/wiki/Chinese_remainder_theorem#C
     */
    let N = n.iter().product::<i64>();
    let mut sum = 0;

    for (&ai, &ni) in a.iter().zip(n) {
        let ndiv = N / ni;
        sum += ai * mod_inverse(ndiv, ni).unwrap() * ndiv;
    }
    sum % N
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");

    let plan: Vec<&str> = buffer.split("\n").filter(|s| !s.is_empty()).collect();
    let depart: u64 = plan[0].parse::<u64>().unwrap();
    let busses: Vec<Bus> = plan[1]
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|bs| bs.parse::<Bus>().unwrap())
        .collect();

    let mut A = Vec::new();
    let mut N = Vec::new();
    for (i, b) in busses.iter().enumerate() {
        if b.active {
            N.push(b.id as i64);
            A.push(i as i64)
        }
    }
    earliest(&busses, depart);
    println!("Chinese remainder: {}", chinese_remainder(&A, &N).abs());
}
