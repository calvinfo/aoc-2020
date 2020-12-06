use std::{fs::File, io::Read};

fn main() {
    let inp = load(String::from("./input"));
    let seats = load_seats(inp);

    let sol1 = part1(&seats);
    println!("Part 1: {}", sol1);

    let sol2 = part2(&seats);
    println!("Part 2: {}", sol2);
}

pub fn part1(v: &Vec<Seat>) -> i32 {
    v.iter().map(|x| x.id()).max().unwrap()
}

pub fn part2(v: &Vec<Seat>) -> i32 {
    let mut ids = v.iter().map(|x| x.id()).collect::<Vec<i32>>();
    ids.sort();

    let mut it = ids.iter();
    let mut curr = it.nth(0).unwrap();

    for id in it {
        if *id > curr + 1 {
            return curr + 1;
        }
        curr = id;
    }
    0
}

/**
 * Definitions
 */

pub struct Seat {
    row: u8,
    col: u8,
}

impl Seat {
    fn from_str(s: &str) -> Seat {
        let row = Seat::get_row(&s[0..7]);
        let col = Seat::get_col(&s[7..10]);
        return Seat { row, col };
    }

    fn get_col(s: &str) -> u8 {
        let mut col: u8 = 0x00;
        for c in s.chars() {
            col = col << 1;
            let val: u8 = match c {
                'L' => 0,
                'R' => 1,
                _ => unreachable!(),
            };
            col = col | val;
        }
        col
    }

    fn get_row(s: &str) -> u8 {
        let mut row: u8 = 0x00;
        for c in s.chars() {
            row = row << 1;
            let val: u8 = match c {
                'F' => 0,
                'B' => 1,
                _ => unreachable!(),
            };
            row = row | val;
        }
        row
    }

    fn id(&self) -> i32 {
        let r = self.row as i32;
        let c = self.col as i32;
        r * 8 + c
    }
}

/**
 * Load boilerplate
 */

pub fn load_seats(inp: String) -> Vec<Seat> {
    inp.lines().map(|x| Seat::from_str(x)).collect()
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
