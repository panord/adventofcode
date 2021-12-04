use anyhow::Context;
use clap::{value_t, App, Arg, ArgMatches};
use either::{Left, Right};
use std::fs::File;
use std::io::{stdin, stdout};
use String;

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

    aoc::do_solve(a, input, &mut output);
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
