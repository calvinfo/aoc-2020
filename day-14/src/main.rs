use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let test = parse(load(String::from("./inp-test")));
    let input = parse(load(String::from("./input")));

    // Part 1
    let test_sol = part1(&test);
    println!("Part 1 test: {}", test_sol);
    let sol1 = part1(&input);
    println!("Part 1: {}", sol1);

    // Part 2
    let test_sol2 = part2(&test);
    println!("Part 2 test: {}", test_sol2);
    let sol2 = part2(&input);
    println!("Part 2: {}", sol2);
}

/**
 * Solution
 */

fn part1(p: &Program) -> u32 {
    let b = p.bitmask;
    for command in p.commands.iter() {
        let val = (b.and & command.val) | (b.or | command.val);
        p.memory.insert(command.addr, val);
    }
    p.memory.values().sum()
}

fn part2(p: &Program) -> u64 {
    0
}

/**
 * Definition
 */

struct Command {
    addr: u32,
    val: u64,
}

struct Bitmask {
    and: u64,
    or: u64,
}



struct Program {
    bitmask: Bitmask,
    memory: HashMap<u64, u64>,
    commands: Vec<Command>
}

/**
 * Load boilerplate
 */

fn parse(s: String) -> Program {
    let mut lines = s.lines();
    let time = lines.next().unwrap().parse::<u32>().unwrap();
    let bus_routes = lines
        .next()
        .unwrap()
        .replace("x", "1")
        .split(",")
        .filter(|x| x != &"x")
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();
    Schedule { time, bus_routes }
}

fn load(filename: String) -> String {
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
