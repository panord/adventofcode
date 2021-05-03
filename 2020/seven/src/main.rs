use std::collections::HashMap;
use std::io::{self, Read};
extern crate regex;
use regex::Regex;

fn parse_contains_rule(tree: &mut HashMap<String, Vec<String>>, s: &str) {
    let re = Regex::new(r"(\w*\s\w*)\sbag").unwrap();
    let re2 = Regex::new(r"(\w*\s\w*\s\w*)\sbag").unwrap();
    let mut chain: Vec<&str> = s.split("contain").collect();
    let key = re.captures(chain[0]).unwrap()[1].to_string();
    for cap in re2.captures_iter(chain[1]) {
        let entry = tree.entry(key.to_string()).or_insert(Vec::new());
        tree.get_mut(&key).unwrap().push(cap[1].to_string());
    }
}

fn parse_containee_rule(tree: &mut HashMap<String, Vec<String>>, s: &str) {
    let re = Regex::new(r"(\w*\s\w*)\sbag").unwrap();
    let mut chain: Vec<&str> = s.split("contain").collect();
    let val = re.captures(chain[0]).unwrap()[1].to_string();
    for cap in re.captures_iter(chain[1]) {
        let entry = tree.entry(cap[1].to_string()).or_insert(Vec::new());
        tree.get_mut(&cap[1]).unwrap().push(val.clone());
    }
}

fn contains(tree: &HashMap<String, Vec<String>>, s: &str) -> usize {
    let re2 = Regex::new(r"(\w*)\s(\w*\s\w*)").unwrap();
    let res = tree.get(s);
    if res.is_none() {
        // println!("Couldn't find '{}'", s);
        return 0;
    }
    let contained_by = res.unwrap();
    let mut subres = 0;
    for b in tree.get(s).unwrap() {
        println!("{:?}", b);
        let cap = re2.captures(b).unwrap();
        let i = cap[1].parse::<usize>();
        if i.is_ok() {
            subres += i.unwrap() * (contains(tree, &cap[2]) + 1);
        }
    }
    return subres;
}

fn may_contain(tree: &mut HashMap<String, Vec<String>>, s: &str, n: &mut HashMap<String, usize>) {
    let res = tree.get(s);
    if res.is_none() {
        // println!("Couldn't find '{}'", s);
        return;
    }
    let contained_by = res.unwrap();
    println!("{:?}", contained_by);
    for b in contained_by.clone() {
        *n.entry(b.to_string()).or_insert(1);
        may_contain(tree, &b, n);
    }
}

fn main() {
    let mut tree = HashMap::new();
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed reading stdin");

    let mut rules: Vec<&str> = buffer.split("\n").collect();
    rules.retain(|&s| s != "");
    for r in &rules {
        parse_contains_rule(&mut tree, r);
        // parse_containee_rule(&mut tree, r);
    }
    let mut res = HashMap::new();
    may_contain(&mut tree, "shiny gold", &mut res);
    let mut n = 0;
    for p in &res {
        if *p.1 > 0 {
            n += 1;
        }
    }
    println!("{:?}", tree);
    println!("Count: {:?}", n);
    for p in &res {
        if *p.1 > 0 {
            n += *p.1;
        }
    }

    n = 0;
    println!("contains: {:?}", contains(&mut tree, "shiny gold"));
}
