use std::io::{self, Read};

fn check_pair(pairs: &Vec<&str>, s: &str) -> bool {
    let mut ok = false;
    for p in pairs {
        ok |= p.find(s).is_some();
    }
    ok
}

fn check_passport(vpairs: &Vec<&str>) -> bool {
    let mut ok = check_pair(&vpairs, "hcl");
    ok &= check_pair(&vpairs, "iyr");
    ok &= check_pair(&vpairs, "eyr");
    ok &= check_pair(&vpairs, "ecl");
    ok &= check_pair(&vpairs, "pid");
    ok &= check_pair(&vpairs, "byr");
    ok &= check_pair(&vpairs, "hgt");
    return ok;
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");

    let mut rows: Vec<&str> = buffer.split("\n\n").collect();
    rows.retain(|&s| s != "");
    let mut i = 0;
    for r in rows {
        let san = r.replace("\n", " ");
        let mut vpairs: Vec<&str> = san.split(" ").collect();
        vpairs.retain(|&s| s != "");
        if check_passport(&vpairs) {
            i += 1;
        }
    }
    println!("OK passports: {}", i)
}
