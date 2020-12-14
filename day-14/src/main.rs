use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut test = parse(load(String::from("./inp-test")));
    let mut input = parse(load(String::from("./input")));

    // Part 1
    let test_sol = part1(&mut test);
    println!("Part 1 test: {}", test_sol);
    let sol1 = part1(&mut input);
    println!("Part 1: {}", sol1);

    // Part 2
    let mut test_2 = parse_2(load(String::from("./inp-test-2")));
    let test_sol2 = part2(&mut test_2);
    println!("Part 2 test: {}", test_sol2);

    let mut input2 = parse_2(load(String::from("./input")));
    let sol2 = part2(&mut input2);
    println!("Part 2: {}", sol2);
}

/**
 * Solution
 */

fn part2(p: &mut FloatingProgram) -> u64 {
    for block in p.blocks.iter() {
        for command in block.commands.iter() {
            let b = block.bitmask.clone();
            let values = b.apply(command.addr);
            for v in values {
                p.memory.insert(v, command.val);
            }
        }
    }
    p.memory.values().sum()
}

fn part1(p: &mut Program) -> u64 {
    for block in p.blocks.iter() {
        let b = block.bitmask;
        for command in block.commands.iter() {
            let val = b.and & (b.or | command.val);
            p.memory.insert(command.addr, val);
        }
    }
    p.memory.values().sum()
}

/**
 * Definition
 */

#[derive(Copy, Clone)]
struct Command {
    addr: u64,
    val: u64,
}

#[derive(Copy, Clone)]
struct Bitmask {
    and: u64,
    or: u64,
}

#[derive(Clone)]
struct FloatingBitmask {
    bitmask: u64,
    floaters: Vec<usize>,
}

impl FloatingBitmask {
    fn apply(self, inp: u64) -> Vec<u64> {
        let mut res = inp | self.bitmask;
        for f in &self.floaters {
            res = res & !(1 << f);
        }

        let mut outputs = Vec::new();
        for i in 0..self.floaters.len()+1 {
            for combination in self.floaters.iter().combinations(i) {
                let mut target = res.clone();
                for num in combination {
                    target = target | (1 << num);
                }
                outputs.push(target);
            }
        }
        outputs
    }
}


struct FloatingBlock {
    bitmask: FloatingBitmask,
    commands: Vec<Command>,
}

impl FloatingBlock {
    fn from_str(s: &str) -> FloatingBlock {
        let bitmask = FloatingBitmask::from_str(s);
        let commands = Vec::new();
        FloatingBlock { bitmask, commands }
    }
}

#[derive(Clone)]
struct Block {
    bitmask: Bitmask,
    commands: Vec<Command>,
}

impl Block {
    fn from_str(s: &str) -> Block {
        let bitmask = Bitmask::from_str(s);
        let commands = Vec::new();
        Block { bitmask, commands }
    }
}

struct Program {
    memory: HashMap<u64, u64>,
    blocks: Vec<Block>,
}

struct FloatingProgram {
    memory: HashMap<u64, u64>,
    blocks: Vec<FloatingBlock>,
}

impl Bitmask {
    fn from_str(s: &str) -> Bitmask {
        let mask = &s["mask = ".len()..s.len()];
        let mut and = 0xFFFFFFFFFFFFFFF;
        let mut or = 0;
        let mut offset = 0;

        for c in mask.chars() {
            or = or << 1;
            match c {
                '1' => or += 1,
                '0' => and = and ^ (1u64 << 35 - offset),
                'X' => {}
                _ => unreachable!(),
            }
            offset += 1;
        }
        Bitmask { and, or }
    }
}

impl FloatingBitmask {
    fn from_str(s: &str) -> FloatingBitmask {
        let mask = &s["mask = ".len()..s.len()];
        let mut bitmask = 0u64;
        let mut floaters = Vec::new();

        for (i, c) in mask.chars().enumerate() {
            bitmask = bitmask << 1;
            match c {
                '1' => bitmask += 1,
                '0' => {},
                'X' => floaters.push(35 - i),
                _ => unreachable!(),
            }
        }
        FloatingBitmask { bitmask, floaters }
    }
}

impl Command {
    fn from_str(s: &str) -> Command {
        let re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        let captures = re.captures(s).unwrap();
        let addr = &captures[1].parse().unwrap();
        let val = &captures[2].parse().unwrap();
        Command {
            addr: *addr,
            val: *val,
        }
    }
}

/**
 * Load boilerplate
 */

fn parse_2(s: String) -> FloatingProgram {
    let mut lines = s.lines();
    let mut blocks = Vec::new();
    let mut block = FloatingBlock::from_str(lines.next().unwrap());

    for line in lines {
        if line.starts_with("mask ") {
            blocks.push(block);
            block = FloatingBlock::from_str(line);
        } else {
            block.commands.push(Command::from_str(line));
        }
    }
    blocks.push(block);
    let memory = HashMap::new();
    FloatingProgram { blocks, memory }
}

fn parse(s: String) -> Program {
    let mut lines = s.lines();
    let mut blocks = Vec::new();
    let mut block = Block::from_str(lines.next().unwrap());

    for line in lines {
        if line.starts_with("mask ") {
            blocks.push(block);
            block = Block::from_str(line);
        } else {
            block.commands.push(Command::from_str(line));
        }
    }
    blocks.push(block);
    let memory = HashMap::new();
    Program { blocks, memory }
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
