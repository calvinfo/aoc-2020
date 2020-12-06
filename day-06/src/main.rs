use std::{fs::File, io::Read};
use itertools::Itertools;

fn main() {
    let inp = load(String::from("./input"));
    let groups = load_groups(inp);

    let sol1 = part1(&groups);
    println!("Part 1: {}", sol1);

    let sol2 = part2(&groups);
    println!("Part 2: {}", sol2);
}

pub fn part1(v: &Vec<Group>) -> i32 {
    v.iter().map(|g| g.num()).sum()
}

pub fn part2(v: &Vec<Group>) -> i32 {
    let count: usize = v.iter().map(|g| g.all()).sum();
    count as i32
}

/**
 * Definitions
 */

pub struct Group {
    passengers: Vec<Passenger>
}

pub struct Passenger {
    responses: Vec<char>
}

impl Group {
    fn from_str(s: &str) -> Group {
        let mut passengers = Vec::new();
        for s in s.lines() {
            let p = Passenger{responses: s.chars().collect_vec()};
            passengers.push(p)
        }
        Group{passengers}
    }

    // Count the number of passengers who said yes to any question
    fn num(&self) -> i32 {
        self.passengers.iter().map(|x| &x.responses).flatten().unique().count() as i32
    }

    // Count the number of passengers who all said yes to a question
    fn all(&self) -> usize {
        let mut counts: [usize; 26] = [0; 26];
        let zero = 'a' as usize;

        for p in self.passengers.iter() {
            for r in &p.responses {
                let idx = *r as usize - zero;
                counts[idx] += 1
            }
        }

        counts.iter().filter(|x| *x == &self.passengers.len()).count()
    }
}

/**
 * Load boilerplate
 */

pub fn load_groups(inp: String) -> Vec<Group> {
    inp.split("\n\n").map(|s| Group::from_str(s)).collect()
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
