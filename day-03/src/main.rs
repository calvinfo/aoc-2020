use std::fs::File;
use std::io::Read;
use std::fmt::Display;

fn main() {
    let mut inp = load(String::from("./input"));
    let m = Map::from_string(inp.as_mut());

    let sol1 = part1(&m);
    println!("Part 1: {}", sol1);

    let sol2 = part2(&m);
    println!("Part 2: {}", sol2);
}

pub fn part1(m: &Map) -> i32 {
    let mut count = 0;
    for i in 1..m.len() {
        match m.get(i, i*3) {
            Item::Tree => count += 1,
            _ => {},
        }
    }
    count
}

pub fn part2(m: &Map) -> u64 {
    let slopes: [(usize, usize); 5] = [
        (1, 1),
        (1, 3),
        (1, 5),
        (1, 7),
        (2, 1)
    ];

    let mut product: u64 = 1;

    for (x, y) in slopes.iter() {
        product *= count_trees(&m, x, y) as u64;
    }

    product
}

pub fn count_trees(m: &Map, slope_r: &usize, slope_c: &usize) -> i32 {
    let mut count = 0;
    let mut row: usize = 0;
    let mut col: usize  = 0;
    while row < m.len() {
        match m.get(row, col) {
            Item::Tree => count += 1,
            _ => {},
        }
        row += slope_r;
        col += slope_c;
    }
    count
}

/**
 * Definitions
 */

pub enum Item {
    Tree,
    Open
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Tree => f.write_str("#"),
            Item::Open => f.write_str("."),
        }
    }
}

pub struct Map {
    rows: Vec<Vec<u8>>,
}

impl Item {
    fn from_char(s: &u8) -> Item {
        if *s == b'#' {
            return Item::Tree;
        } else {
            return Item::Open;
        }
    }
}

impl Map {
    pub fn from_string(s: &str) -> Map {
        let rows = s.lines()
            .map(|x| x.as_bytes().to_vec() ).collect::<Vec<Vec<u8>>>();

        return Map{rows};
    }

    pub fn get(&self, row: usize, col: usize) -> Item {
        let curr_row = self.rows.get(row).unwrap();
        let index = col % curr_row.len();
        let val = curr_row.get(index).unwrap();
        return Item::from_char(val);
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

/**
 * Load boilerplate
 */

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