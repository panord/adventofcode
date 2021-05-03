use std::io::{self, Read};
use std::mem;

fn line_occ(rows: &Vec<Vec<char>>, x: i64, y: i64, dx: i64, dy: i64) -> u64 {
    let mut xx = x + dx;
    let mut yy = y + dy;
    let height = rows.len();
    let width = rows[0].len();
    while (yy as usize) < height && (xx as usize) < width {
        let c = rows[yy as usize][xx as usize];
        match c {
            '#' => {
                // println!("HIT {} {}", xx, yy);
                return 1;
            }
            'L' => return 0,
            _ => {
                yy += dy;
                xx += dx;
            }
        };
    }
    return 0;
}

fn line_adj(rows: &Vec<Vec<char>>, x: i64, y: i64) -> u64 {
    let mut res = 0;
    res += line_occ(&rows, x, y, 1, 0); // Right
    res += line_occ(&rows, x, y, -1, 0); // Left
    res += line_occ(&rows, x, y, 0, -1); // Up
    res += line_occ(&rows, x, y, 0, 1); // Down
    res += line_occ(&rows, x, y, -1, -1); // Left Up
    res += line_occ(&rows, x, y, -1, 1); // Left Down
    res += line_occ(&rows, x, y, 1, -1); // Right Up
    res += line_occ(&rows, x, y, 1, 1); // Right Down
    return res;
}

fn adj(rows: &Vec<Vec<char>>, c: char, i: i64, j: i64) -> u64 {
    let mut res = 0;
    for l in i - 1..i + 2 {
        for m in j - 1..j + 2 {
            if !(l == i && m == j) {
                let r = rows.get(l as usize);
                if r.is_some() {
                    let c = r.unwrap().get(m as usize);
                    if c.is_some() {
                        if *c.unwrap() == '#' {
                            res += 1;
                        }
                    }
                }
            }
        }
    }
    return res;
}

fn apply_rule1(rows: &Vec<Vec<char>>, c: char, y: i64, x: i64, thresh: u64) -> char {
    if c == '#' && adj(&rows, c, x, y) >= thresh {
        return 'L';
    }
    if c == 'L' && adj(&rows, c, x, y) == 0 {
        return '#';
    }
    return c;
}

fn apply_rule2(rows: &Vec<Vec<char>>, c: char, y: i64, x: i64, thresh: u64) -> char {
    if c == '#' && line_adj(&rows, x, y) >= thresh {
        return 'L';
    }
    if c == 'L' && line_adj(&rows, x, y) == 0 {
        return '#';
    }
    return c;
}

fn step_rows(rows: &Vec<Vec<char>>, thresh: u64, part_one: bool) -> Vec<Vec<char>> {
    let mut i = 0;
    let mut new_rows = vec![];
    for r in rows.iter() {
        let mut j = 0;
        let mut nr = vec![];
        for c in r.iter() {
            if part_one {
                nr.push(apply_rule1(&rows, *c, i, j, thresh));
            } else {
                nr.push(apply_rule2(&rows, *c, i, j, thresh));
            }
            j += 1;
        }
        new_rows.push(nr);
        i += 1;
    }
    return new_rows;
}

fn shuffle(mut rows: Vec<Vec<char>>, thresh: u64, print: bool, part_one: bool) -> u64 {
    let mut occ = 0;
    let mut last_occ = 0;
    loop {
        occ = 0;
        rows = step_rows(&rows, thresh, part_one);
        if print {
            print_grid(&rows);
            print_lineadj(&rows);
        }

        for r in &rows {
            for c in r {
                if *c == '#' {
                    occ += 1;
                }
            }
        }

        if occ == last_occ {
            break;
        }
        last_occ = occ;
    }
    return last_occ;
}

fn print_lineadj(rows: &Vec<Vec<char>>) {
    for (i, r) in rows.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            print!("{}", line_adj(&rows, i as i64, j as i64));
        }
        println!("");
    }
    println!("");
}

fn print_grid(rows: &Vec<Vec<char>>) {
    for r in rows {
        for c in r {
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");
    let mut rows: Vec<Vec<char>> = buffer
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|x| x.to_string().chars().collect())
        .collect();

    println!("occupied {}", shuffle(rows.clone(), 4, false, true));
    println!("occupied {}", shuffle(rows, 5, false, false));
}
