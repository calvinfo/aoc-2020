use std::{fs::File, io::Read};

fn main() {
    let test = load(String::from("./inp-test"));
    let test_v = load_nums(test);

    let test_sol = part1(&test_v);
    println!("Part 1 test: {}", test_sol);

    let test_sol2 = part2(&test_v);
    println!("Part 2 test: {}", test_sol2);

    let inp = load(String::from("./input"));
    let v = load_nums(inp);

    let sol1 = part1(&v);
    println!("Part 1: {}", sol1);

    let sol2 = part2(&v);
    println!("Part 2: {}", sol2);
}

fn part1(v: &Vec<u64>) -> u64 {
    let mut jump_1 = 0;
    let mut jump_3 = 0;
    let mut current = 0;
    for i in v {
        match i - current {
            3 => jump_3 += 1,
            1 => jump_1 += 1,
            _ => unreachable!(),
        }
        current = *i;
    }
    jump_1 * (jump_3 + 1) as u64
}

fn fib(i: i32) -> i32 {
    match i {
        2 => 2,
        1 => 1,
        _ => fib(i - 1) + fib(i - 2)
    }
}

fn fib_sum(i: i32) -> i32 {
    (1..i).map(|x| fib(x)).sum()
}

fn part2(v: &Vec<u64>) -> u64 {
    let mut count: u64 = 1;
    let mut cons_jump_one: i32 = 0;
    let mut current = 0;
    for i in v {
        match i - current {
            3 => { count *= (fib_sum(cons_jump_one) + 1) as u64; cons_jump_one = 0; },
            1 => cons_jump_one += 1,
            _ => unreachable!(),
        }
        current = *i;
    }
    count *= (fib_sum(cons_jump_one) + 1) as u64;
    count
}

/**
 * Load boilerplate
 */

fn load_nums(s: String) -> Vec<u64> {
    let mut v: Vec<u64> = s.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    v.sort();
    v
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
