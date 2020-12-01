use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let v = load(String::from("./input"));
    let sol1 = part1(v);
    println!("Part 1: {}", sol1);

    let v2 = load(String::from("./input"));
    let sol2 = part2(v2);
    println!("Part 2: {}", sol2);
}

pub fn load(filename: String) -> Vec<i32> {
    let mut input = match File::open(filename) {
        Ok(f) => f,
        Err(err) => panic!(err),
    };

    let mut content = String::new();
    match input.read_to_string(&mut content) {
        Err(err) => panic!(err),
        Ok(_) => {}
    }

    content
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn part1(input: Vec<i32>) -> i32 {
    let compare = 2020i32;
    let mut numbers = HashSet::new();
    for i in input.into_iter() {
        // if we've already found our other number, return it!
        let candidate = compare - i;
        if numbers.contains(&candidate) {
            return candidate * i;
        } else {
            numbers.insert(i);
        }
    }
    return -1;
}

pub fn part2(input: Vec<i32>) -> i32 {
    // permutations can give us all sets of three numbers
    let mut candidates = input.into_iter().permutations(3);
    let solution = candidates.find(|x| x[0] + x[1] + x[2] == 2020);
    return solution.unwrap().into_iter().product();
}
