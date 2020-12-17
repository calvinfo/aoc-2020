use std::collections::HashMap;
use std::{fs::File, io::Read};

fn main() {
    let mut test = parse(load(String::from("./inp-test")));
    let mut input = parse(load(String::from("./input")));

    // Part 1
    let test_sol = part1(&mut test);
    println!("Part 1 test: {}", test_sol);
    let sol1 = part1(&mut input);
    println!("Part 1: {}", sol1);

    // Part 2
    let mut test2 = parse(load(String::from("./inp-test")));
    let mut input2 = parse(load(String::from("./input")));
    let test_sol2 = part2(&mut test2);
    println!("Part 2 test: {}", test_sol2);
    let sol2 = part2(&mut input2);
    println!("Part 2: {}", sol2);
}

/**
 * Solution
 */

fn part1(g: &mut Game) -> i64 {
    while g.step != 2020 {
        g.next();
    }
    g.last
}

fn part2(g: &mut Game) -> i64 {
    while g.step != 30000000 {
        g.next();
    }
    g.last
}
/**
 * Definition
 */

struct Game {
    seen: HashMap<i64, (i64, i64)>,
    step: i64,
    last: i64,
}

impl Game {
    fn from_str(s: &str) -> Game {
        let mut seen: HashMap<i64, (i64, i64)> = HashMap::new();
        let mut step = 0;
        let mut last = 0;

        let start: Vec<i64> = s.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
        for item in start {
            step += 1;
            let val = seen.get(&item);
            match val {
                Some(val) => seen.insert(item, (step, val.0)),
                None => seen.insert(item, (step, -1)),
            };
            last = item;
        }

        Game { seen, step, last }
    }

    fn next(&mut self) -> i64 {
        self.step += 1;
        let mut seen = &self.seen;
        let sequence = seen.get(&self.last);
        // First we lookup the last time the last number was seen
        let num = match sequence {
            None => 0,             // hasn't been seen before: 0
            Some((x, -1)) => 0,    // has been seen once: 0
            Some((x, y)) => x - y, // has been seen twice: diff
        };

        // update the num
        let seq = seen.get(&num);
        match seq {
            Some((x, y)) => self.seen.insert(num, (self.step, *x)),
            None => self.seen.insert(num, (self.step, -1)),
        };

        self.last = num;
        num
    }

    fn log(&self) {
        println!("Turn [{}]: {}", self.step, self.last);
    }
}

/**
 * Load boilerplate
 */

fn parse(s: String) -> Game {
    Game::from_str(&s)
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
