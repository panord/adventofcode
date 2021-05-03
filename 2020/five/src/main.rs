use std::io::{self, Read};

fn calc_row(s: &str, col: &mut usize, row: &mut usize) {
    let mut lower = 0;
    let mut upper = 127;
    for c in s[0..7].chars() {
        match &c {
            'B' => lower += (upper - lower) / 2 + 1,
            'F' => upper -= (upper - lower) / 2 + 1,
            _ => println!("UNRECOGNIZED CHAR"),
        }
    }
    *row = lower;
    lower = 0;
    upper = 7;
    for c in s[7..].chars() {
        match &c {
            'R' => lower += (upper - lower) / 2 + 1,
            'L' => upper -= (upper - lower) / 2 + 1,
            _ => println!("UNRECOGNIZED CHAR"),
        }
    }
    *col = lower;
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");

    let mut rows: Vec<&str> = buffer.split("\n").collect();
    let mut max = 0;
    let mut col = 0;
    let mut row = 0;
    rows.retain(|&s| s != "");
    let mut seats: Vec<usize> = Vec::new();
    for r in rows {
        calc_row(r, &mut col, &mut row);
        let id = row * 8 + col;
        seats.push(id);
        if id > max {
            max = id;
        }
    }
    seats.sort_by(|a, b| a.cmp(b));

    let mut i = 0;
    while seats[i] + 1 == seats[i + 1] {
        i += 1;
    }
    println!("Max: {}", max);
    println!("My Seat: {}", seats[i] + 1);
}
