use std::io::{self, Read};

struct Point {
    x: usize,
    y: usize,
}

struct Toboggan {
    pos: Point,
}

impl Toboggan {
    fn step(&mut self, x: usize, y: usize) {
        self.pos.x += x;
        self.pos.y += y;
    }

    fn wrap_boundary(&mut self, max: &Point) {
        self.pos.x = self.pos.x % max.x;
        self.pos.y = self.pos.y % max.y;
    }
}

fn on_tree(map: &Vec<&str>, point: &Point) -> bool {
    let row: &str = map[point.y];
    let c = row.chars().nth(point.x).unwrap();
    return c == '#';
}

fn calc_slope(rows: &Vec<&str>, slope: Point) -> i64 {
    let mut t = Toboggan {
        pos: Point { x: 0, y: 0 },
    };
    let boundary = Point {
        x: rows[0].len(),
        y: rows.len(),
    };
    let mut i = 0;
    loop {
        if &t.pos.y >= &boundary.y {
            break;
        }
        t.wrap_boundary(&boundary);
        if on_tree(&rows, &t.pos) {
            i += 1;
        }
        t.step(slope.x, slope.y);
    }
    println!("trees hit for {} {}: {}", slope.x, slope.y, i);
    return i;
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");

    let mut rows: Vec<&str> = buffer.split("\n").collect();
    rows.retain(|&s| s != "");
    let mut ans = 1;
    ans *= calc_slope(&rows, Point { x: 1, y: 1 });
    ans *= calc_slope(&rows, Point { x: 3, y: 1 });
    ans *= calc_slope(&rows, Point { x: 5, y: 1 });
    ans *= calc_slope(&rows, Point { x: 7, y: 1 });
    ans *= calc_slope(&rows, Point { x: 1, y: 2 });
    println!("Answer: {}", ans);
}
