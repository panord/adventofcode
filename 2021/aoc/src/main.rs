use anyhow::{Context, Result};
use clap::{value_t, App, Arg, ArgMatches};
use either::{Left, Right};
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader};
use String;

fn sonar_sweep<X, Y>(input: X, out: &mut Y)
where
    X: std::io::Read,
    Y: std::io::Write,
{
    let reader = BufReader::new(input);
    let mut last: i64 = std::i64::MAX;
    let mut curr: i64;
    let mut cnt: u64 = 0;
    for line in reader.lines().into_iter() {
        let l = line.unwrap();
        if l.is_empty() {
            break;
        }
        curr = l.parse::<i64>().unwrap();
        if curr > last {
            cnt += 1;
        }
        last = curr;
    }

    out.write(format!("Growing: {}\n", cnt).as_ref())
        .expect("Failed to write result");
}

fn solve(args: &ArgMatches) -> Result<()> {
    let a =
        value_t!(args.value_of("assignment"), u32).context("Could not parse assignment number")?;

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

    match a {
        1 => sonar_sweep(input, &mut output),
        _ => println!("Unsupported assignment {}", a),
    };
    Ok(())
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

    solve(&matches).unwrap();
}
