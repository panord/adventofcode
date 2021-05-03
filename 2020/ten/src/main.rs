use std::collections::HashMap;
use std::io::{self, Read};

fn combs(adap: &Vec<i64>) -> i64 {
    let mut combs = HashMap::new();
    combs.insert(0, 1);
    for a in adap {
        let mut e = combs.entry(*a).or_insert(0);
        for i in *a - 3..*a {
            if combs.iter().find(|p| *p.0 == i).is_some() {
                let v = combs[&i].clone();
                let mut e = combs.get_mut(a).unwrap();
                println!("HEEEJ a:{}, e:{}, v:{}", a, e, v);
                *e += v;
            }
        }
    }
    println!("{:?}", combs);
    combs[adap.iter().max().unwrap()]
}

fn adapters(adap: &Vec<i64>, jolts: &mut [u64], acc: &mut u64) {
    for i in 0..adap.len() - 1 {
        let diff = adap[i + 1] - adap[i];
        jolts[diff as usize] += 1;
    }
}

fn main() {
    let mut jolts = vec![];
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");
    let mut adap: Vec<i64> = buffer
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    jolts.resize_with(*adap.iter().max().unwrap() as usize, || 0);
    adap.sort_by(|a, b| a.partial_cmp(b).unwrap());
    adap.push(adap[adap.len() - 1] + 3);
    adap.insert(0, 0);
    let mut acc = 1;
    adapters(&adap, &mut jolts, &mut acc);

    println!("Adapters {:?}", adap);
    println!("{} * {}  = {}", jolts[1], jolts[3], jolts[1] * jolts[3]);
    println!("Combs: {}", combs(&adap));
}
