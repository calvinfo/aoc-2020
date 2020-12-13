use std::{fs::File, io::Read};

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

fn part1(s: &Schedule) -> u32 {
    let mut time = s.time;
    loop {
        let found: Option<&u32> = s.bus_routes.iter().find(|x| *x != &1u32 && time % *x == 0);
        match found {
            Some(bus) => return (time - s.time) * bus,
            None => time += 1,
        }
    }
}

fn part2(s: &Schedule) -> u64 {
    let mut number = 0;
    let mut step = 0;

    // First step, get out all of our modulo bases, and sort them
    // from largest to smallest
    let mut bases: Vec<&u32> = s
        .bus_routes
        .iter()
        .filter(|x| *x != &1u32)
        .collect::<Vec<&u32>>();
    bases.sort();
    bases.reverse();

    for base in bases {
        let (mut target_remainder, modulo) = s
            .bus_routes
            .iter()
            .enumerate()
            .find(|(_, val)| val == &base)
            .unwrap();

        // println!("number: {}, step: {}, num = {} (mod {})", number, step, target_remainder, modulo);

        // Special case the first, where we start out with the first
        // target, and then the step being the first number.
        if number == 0 && step == 0 {
            number = target_remainder as u64;
            step = *modulo as u64;
            continue;
        }

        target_remainder = target_remainder % (*modulo as usize);
        while (number % *modulo as u64) != (target_remainder as u64) {
            number += step;
        }

        step *= *modulo as u64;
    }

    step - number
}

/**
 * Definition
 */

struct Schedule {
    time: u32,
    bus_routes: Vec<u32>,
}

/**
 * Load boilerplate
 */

fn parse(s: String) -> Schedule {
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
