use std::collections::HashMap;
use std::io::{self, Read};

fn add_group(
    qnaire: &mut HashMap<char, usize>,
    people: &Vec<&str>,
    tot: &mut usize,
    tot2: &mut usize,
) {
    for p in people {
        for c in p.chars() {
            let q = qnaire.entry(c).or_insert(0);
            *q += 1;
        }
    }
    for q in "abcdefghijklmnopqrstuvwxyz".chars() {
        let c = qnaire.get(&q).unwrap();
        if *c > 0 {
            *tot += 1;
        }
        if *c == people.len() {
            *tot2 += 1;
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");

    let mut groups: Vec<&str> = buffer.split("\n\n").collect();
    groups.retain(|&s| s != "");

    let mut tot = 0;
    let mut tot2 = 0;
    for g in &groups {
        let mut qnaire = HashMap::new();
        let alpha = "abcdefghijklmnopqrstuvwxyz";
        for q in alpha.chars() {
            qnaire.insert(q, 0);
        }
        let mut people: Vec<&str> = g.split("\n").collect();
        people.retain(|&s| s != "");
        add_group(&mut qnaire, &people, &mut tot, &mut tot2);
    }
    println!("Total yes answers: {}", tot);
    println!("Total yes answers: {}", tot2);
}
