use std::io::{BufRead, BufReader};

pub fn from_lines<X, Y, Z>(input: X) -> Z
where
    X: std::io::Read,
    Y: std::str::FromStr,
    <Y as std::str::FromStr>::Err: std::fmt::Debug,
    Z: std::iter::FromIterator<Y>,
{
    BufReader::new(input)
        .lines()
        .into_iter()
        .map(|r| r.and_then(|r| Ok(r.parse::<Y>().unwrap())).unwrap())
        .collect()
}

pub fn do_sonar_sweep(vals: &mut Vec<i64>, n: usize) -> usize {
    let mut last: i64 = std::i64::MAX;
    let mut cnt: usize = 0;

    for i in 0..vals.len() - (n - 1) {
        for j in 1..n {
            vals[i] += vals[i + j];
        }
    }

    for v in 0..vals.len() - (n - 1) {
        if vals[v] > last {
            cnt += 1;
        }
        last = vals[v];
    }
    return cnt;
}

pub fn sonar_sweep<X, Y>(input: X, out: &mut Y, n: usize)
where
    X: std::io::Read,
    Y: std::io::Write,
{
    let mut vals: Vec<i64> = from_lines(input);
    let cnt = do_sonar_sweep(&mut vals, n);

    out.write(format!("Growing: {}\n", cnt).as_ref())
        .expect("Failed to write result");
}

pub fn do_solve<X, Y>(assign: u8, input: X, output: &mut Y)
where
    X: std::io::Read,
    Y: std::io::Write,
{
    match assign {
        1 => sonar_sweep(input, output, 1),
        2 => sonar_sweep(input, output, 3),
        _ => println!("Unsupported assignment {}", assign),
    };
}
