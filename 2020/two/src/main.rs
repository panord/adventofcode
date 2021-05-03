use std::io::{self, Read};

fn is_valid_row2(row: &str) -> bool {
    let r: Vec<&str> = row.split(" ").collect();
    if r.len() < 3 {
        println!("Row too small");
    }

    let policy_range = r[0];
    let rangestr: Vec<&str> = policy_range.split("-").collect();
    let (start, end) = (
        rangestr[0].parse::<usize>().expect("fail parse"),
        rangestr[1].parse::<usize>().expect("fail parse"),
    );
    let mc = r[1].chars().nth(0).unwrap();
    let first = r[2].chars().nth(start - 1).unwrap() == mc;
    let second = r[2].chars().nth(end - 1).unwrap() == mc;
    return (first || second) && !(first && second)t;
}

fn is_valid_row(row: &str) -> bool {
    let r: Vec<&str> = row.split(" ").collect();
    if r.len() < 3 {
        println!("Row too small");
    }

    let policy_range = r[0];
    let rangestr: Vec<&str> = policy_range.split("-").collect();
    let (start, end) = (
        rangestr[0].parse::<usize>().expect("fail parse"),
        rangestr[1].parse::<usize>().expect("fail parse"),
    );
    let c = r[2].matches(r[1].chars().nth(0).unwrap()).count();
    return c <= end && c >= start;
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");

    let mut rows: Vec<&str> = buffer.split("\n").collect();
    rows.retain(|&s| s != "");
    let mut i = 0;
    for r in rows {
        if is_valid_row2(r) {
            println!("{}", r);
            i += 1
        }
    }
    println!("{}", i);
}
