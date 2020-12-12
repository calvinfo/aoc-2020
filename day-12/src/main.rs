use std::ops::Add;
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

fn part1(v: &Vec<Instruction>) -> u64 {
    let mut s = Ship::new();
    for i in v {
        s.apply(*i);
    }
    s.point.x.abs() as u64 + s.point.y.abs() as u64
}

fn part2(v: &Vec<Instruction>) -> u64 {
    let mut s = Ship::new();
    s.direction = Point { x: 10, y: 1 };
    for i in v {
        s.apply_2(*i);
    }
    s.point.x.abs() as u64 + s.point.y.abs() as u64
}

/**
 * Definition
 */

#[derive(Debug, Copy, Clone, PartialEq)]
enum Command {
    Forward,
    Left,
    Right,
    North,
    East,
    South,
    West,
}

impl Command {
    fn from_str(s: &str) -> Command {
        match s {
            "F" => Command::Forward,
            "L" => Command::Left,
            "R" => Command::Right,
            "N" => Command::North,
            "E" => Command::East,
            "S" => Command::South,
            "W" => Command::West,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

const NORTH: Point = Point { x: 0, y: 1 };
const SOUTH: Point = Point { x: 0, y: -1 };
const EAST: Point = Point { x: 1, y: 0 };
const WEST: Point = Point { x: -1, y: 0 };

impl Point {
    fn mul(self, i: i32) -> Point {
        return Point {
            x: self.x * i,
            y: self.y * i,
        };
    }

    fn rotate(self, i: Instruction) -> Point {
        let modifier = match i.cmd {
            Command::Left => -1,
            Command::Right => 1,
            _ => unreachable!(),
        };

        let amount = (360 + (modifier * i.amount)) % 360;
        match amount {
            90 => Point {
                x: self.y,
                y: -self.x,
            },
            180 => Point {
                x: -self.x,
                y: -self.y,
            },
            270 => Point {
                x: -self.y,
                y: self.x,
            },
            0 | 360 => Point {
                x: self.x,
                y: self.y,
            },
            _ => {
                println!("uh oh: {}", amount);
                unreachable!()
            }
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

struct Ship {
    point: Point,
    direction: Point,
}

impl Ship {
    fn new() -> Ship {
        return Ship {
            point: Point { x: 0, y: 0 },
            direction: EAST,
        };
    }

    fn apply(&mut self, inst: Instruction) {
        match inst.cmd {
            Command::Forward => self.point = self.point + self.direction.mul(inst.amount),
            Command::Left => self.direction = self.direction.rotate(inst),
            Command::Right => self.direction = self.direction.rotate(inst),
            Command::North => self.point = self.point + NORTH.mul(inst.amount),
            Command::East => self.point = self.point + EAST.mul(inst.amount),
            Command::South => self.point = self.point + SOUTH.mul(inst.amount),
            Command::West => self.point = self.point + WEST.mul(inst.amount),
        }
    }

    fn apply_2(&mut self, inst: Instruction) {
        match inst.cmd {
            Command::Forward => self.point = self.point + self.direction.mul(inst.amount),
            Command::Left => self.direction = self.direction.rotate(inst),
            Command::Right => self.direction = self.direction.rotate(inst),
            Command::North => self.direction = self.direction + NORTH.mul(inst.amount),
            Command::East => self.direction = self.direction + EAST.mul(inst.amount),
            Command::South => self.direction = self.direction + SOUTH.mul(inst.amount),
            Command::West => self.direction = self.direction + WEST.mul(inst.amount),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Instruction {
    cmd: Command,
    amount: i32,
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        let cmd = Command::from_str(&s[0..1]);
        let amount = &s[1..s.len()].parse::<i32>().unwrap();
        Instruction {
            cmd,
            amount: *amount,
        }
    }
}

/**
 * Load boilerplate
 */

fn parse(s: String) -> Vec<Instruction> {
    s.lines().map(|x| Instruction::from_str(x)).collect()
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
