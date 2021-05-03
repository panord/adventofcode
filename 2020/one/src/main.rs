use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");

    let vec: Vec<&str> = buffer.split("\n").collect();
    // print!("{:?}", vec);
    for jstr in &vec {
        for kstr in &vec {
            for lstr in &vec {
                let jp = jstr.parse::<i32>();
                let kp = kstr.parse::<i32>();
                let lp = lstr.parse::<i32>();
                if jp.is_ok() && kp.is_ok() && lp.is_ok() {
                    let j = jp.unwrap();
                    let k = kp.unwrap();
                    let l = lp.unwrap();
                    if j + k + l == 2020 {
                        println!("{} * {} * {} == {}", j, k, l, j * k * l);
                    }
                }
            }
        }
    }
}
