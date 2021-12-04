use simple_error::SimpleError;
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

#[derive(Clone)]
pub enum Direction {
    FORWARD,
    BACKWARD,
    DOWN,
    UP,
}

#[derive(Clone)]
pub struct Move {
    pub dir: Direction,
    pub dist: usize,
}

impl std::str::FromStr for Direction {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Direction, Self::Err> {
        match s {
            "forward" => Ok(Direction::FORWARD),
            "backward" => Ok(Direction::BACKWARD),
            "up" => Ok(Direction::UP),
            "down" => Ok(Direction::DOWN),
            _ => Err(Self::Err::new("Failed to parse direction")),
        }
    }
}

impl std::str::FromStr for Move {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Move, Self::Err> {
        let sv: Vec<&str> = s.split(" ").collect();
        let dir = sv[0].parse::<Direction>()?;
        let dst = sv[1].parse::<usize>()?;

        Ok(Move {
            dist: dst,
            dir: dir,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
    pub aim: usize,
}

pub fn do_dive_aim(mut pos: Pos, moves: Vec<Move>) -> Pos {
    moves.into_iter().for_each(|m| match m.dir {
        Direction::FORWARD => {
            pos.x += m.dist;
            pos.y += pos.aim * m.dist
        }
        Direction::BACKWARD => pos.x -= m.dist,
        Direction::UP => pos.aim -= m.dist,
        Direction::DOWN => pos.aim += m.dist,
    });
    return pos;
}

pub fn do_dive(mut pos: Pos, moves: Vec<Move>) -> Pos {
    moves.into_iter().for_each(|m| match m.dir {
        Direction::FORWARD => pos.x += m.dist,
        Direction::BACKWARD => pos.x -= m.dist,
        Direction::UP => pos.y -= m.dist,
        Direction::DOWN => pos.y += m.dist,
    });
    return pos;
}

pub fn dive<X, Y>(input: X, out: &mut Y, aim: bool)
where
    X: std::io::Read,
    Y: std::io::Write,
{
    let vals: Vec<Move> = from_lines(input);
    let orig = Pos { x: 0, y: 0, aim: 0 };
    let dest = if aim {
        do_dive_aim(orig, vals)
    } else {
        do_dive(orig, vals)
    };

    out.write(format!("Position: {:?}\n", dest).as_ref())
        .expect("Failed to write result");

    out.write(format!("x * y: {}\n", dest.x * dest.y).as_ref())
        .expect("Failed to write result");
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
        3 => dive(input, output, false),
        4 => dive(input, output, true),
        _ => println!("Unsupported assignment {}", assign),
    };
}
