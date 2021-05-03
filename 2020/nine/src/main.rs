use std::io::{self, Read};

fn is_sum_of_2(n: &u64, nums: &[u64]) -> bool {
    for i in nums {
        for j in nums {
            if i + j == *n {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");
    let ciph: Vec<u64> = buffer
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    find_err(&ciph, 5);
    println!("Sum of 5: {}", contiguous(&ciph, find_err(&ciph, 5)));
    println!("Sum of 25: {}", contiguous(&ciph, find_err(&ciph, 25)));
}

fn contiguous(ciph: &Vec<u64>, target: u64) -> u64 {
    let mut sumlen = 2;
    while sumlen < ciph.len() {
        let mut i = 0;
        while i + sumlen < ciph.len() {
            let c = &ciph[i..i + sumlen + 1];
            let sum: u64 = c.iter().sum();
            if target == c.iter().sum() {
                return c.iter().min().unwrap() + c.iter().max().unwrap();
            }
            i += 1;
        }
        sumlen += 1;
    }
    return 0;
}

fn find_err(ciph: &Vec<u64>, off: usize) -> u64 {
    let mut i = off;
    while i < ciph.len() {
        if !is_sum_of_2(&ciph[i], &ciph[i - off..i]) {
            println!("Fails rule: {}", &ciph[i]);
            return ciph[i];
        }
        i += 1;
    }
    return 0;
}
