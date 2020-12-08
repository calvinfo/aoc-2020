use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let inp = load(String::from("./input"));
    let bags = load_bags(inp);

    let sol1 = part1(&bags);
    println!("Part 1: {}", sol1);

    let sol2 = part2(&bags);
    println!("Part 2: {}", sol2);
}

pub fn part1(v: &Vec<Bag>) -> usize {
    let mut results = Vec::new();
    let mut ancestors = find_ancestors(v, "shiny gold");

    while !ancestors.is_empty() {
        let ancestor = ancestors.pop().unwrap();
        let new_ancestors = find_ancestors(v, &String::from(&ancestor));

        let ancestor_copy = String::from(&ancestor);
        results.push(ancestor_copy);

        for considered in new_ancestors.iter() {
            let candidate = String::from(considered);
            if !results.contains(&candidate) && !ancestors.contains(&candidate) {
                ancestors.push(candidate);
            }
        }
    }
    results.len()
}

fn find_ancestors(v: &Vec<Bag>, name: &str) -> Vec<String> {
    let mut ancestors = Vec::new();
    for bag in v {
        if bag.bags.contains_key(name) {
            ancestors.push(String::from(&bag.name));
        }
    }
    ancestors
}

fn part2(v: &Vec<Bag>) -> u64 {
    part2_helper(v, "shiny gold".to_string()) - 1
}

fn part2_helper(v: &Vec<Bag>, name: String) -> u64 {
    let found = v.iter().find(|b| b.name == name).unwrap();
    let mut count = 1;
    let children = &found.bags;
    for (child_name, child_count) in children {
        count += part2_helper(v, String::from(child_name)) * *child_count as u64
    }
    count
}

/**
 * Definitions
 */

pub struct Bag {
    name: String,
    bags: HashMap<String, i32>,
}

impl Bag {
    fn from_str(s: &str) -> Bag {
        let bag_matcher = Regex::new(&r"^(?P<name>[a-z ]*) bags contain").unwrap();
        let bag_subgroups = Regex::new(&r"(\d+ [a-z ]+ bags?)").unwrap();

        let cap1 = bag_matcher.captures(s).unwrap();
        let name = cap1.name("name").unwrap().as_str();

        let mut bags = HashMap::new();
        for mat in bag_subgroups.find_iter(s) {
            let mut s = mat.as_str().replace(" bags", "").replace(" bag", "");
            let num = s[0..1].parse::<i32>().unwrap();
            s.replace_range(0..2, "");
            bags.insert(s, num);
        }

        Bag {
            name: name.to_string(),
            bags: bags,
        }
    }
}

/**
 * Load boilerplate
 */

pub fn load_bags(inp: String) -> Vec<Bag> {
    inp.lines().map(|s| Bag::from_str(s)).collect()
}

pub fn load(filename: String) -> String {
    let mut input = match File::open(filename) {
        Ok(f) => f,
        Err(err) => panic!(err),
    };

    let mut content = String::new();
    match input.read_to_string(&mut content) {
        Err(err) => panic!(err),
        Ok(_) => {}
    }

    return content;
}
