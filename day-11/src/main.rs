use std::{fs::File, io::Read};

fn main() {
    let test = load(String::from("./inp-test"));
    let  m = Map::from_str(&test);

    let test_sol = part1(&m);
    println!("Part 1 test: {}", test_sol);

    let test_sol2 = part2(&m);
    println!("Part 2 test: {}", test_sol2);

    let inp = load(String::from("./input"));
    let map = Map::from_str(&inp);

    let sol1 = part1(&map);
    println!("Part 1: {}", sol1);

    let sol2 = part2(&map);
    println!("Part 2: {}", sol2);
}

fn part1(m: &Map) -> u64 {
    let mut map = m.clone();
    let mut prev_occupied: usize = 1000000;
    let mut curr_occupied = map.num_occupied();

    while prev_occupied != curr_occupied {
        map = map.iter().clone();
        prev_occupied = curr_occupied;
        curr_occupied = map.num_occupied();
    }
    
    curr_occupied as u64
}

fn part2(m: &Map) -> u64 {
    let mut map = m.clone();
    let mut prev_occupied: usize = 1000000;
    let mut curr_occupied = map.num_occupied();

    while prev_occupied != curr_occupied {
        map = map.iter_2().clone();
        prev_occupied = curr_occupied;
        curr_occupied = map.num_occupied();
    }
    
    curr_occupied as u64
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile {
    Empty,
    Occupied,
    Floor,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            'L' => Tile::Empty,
            '#' => Tile::Occupied,
            '.' => Tile::Floor,
            _ => unreachable!(),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Tile::Empty => "L".to_string(),
            Tile::Occupied => "#".to_string(),
            Tile::Floor => ".".to_string(),
        }
    }
}

#[derive(Clone, PartialEq)]
struct Map {
    graph: Vec<Tile>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn from_str(s: &str) -> Map {
        let graph = s
            .lines()
            .map(|x| x.chars())
            .flatten()
            .map(|x| Tile::from_char(x))
            .collect();

        let rows = s.lines().count();
        let cols = s.lines().nth(0).unwrap().len();
        Map { graph, rows, cols }
    }

    // counts how many neighbors of a type are occupied
    fn neighbors(&self, i: i32, j: i32) -> i32 {
        let mut count = 0;
        for d_i in 0..3 {
            for d_j in 0..3 {
                if d_i == 1 && d_j == 1 {
                    continue
                }
                match self.get(i + d_i - 1, j + d_j - 1) {
                    Some(Tile::Occupied) => count += 1,
                    _ => {},
                }
            }
        }
        count
    }

    fn sight_neighbors(&self, i: i32, j: i32) -> i32 {
        let mut count = 0;
        let rows = self.rows as i32;
        for slope_i in -1..2 {
            for slope_j in -1..2 {
                if slope_i == 0 && slope_j == 0 {
                    continue
                }

                for modifier in 1..rows {
                    match self.get(i + slope_i * modifier, j + slope_j * modifier) {
                        Some(Tile::Occupied) => { count += 1; break },
                        Some(Tile::Empty) => break,
                        _ => {},
                    }
                }
            }
        }

        count
    }

    fn get(&self, i: i32, j: i32) -> Option<&Tile> {
        if i < 0 || j < 0 || i as usize >= self.rows || j as usize >= self.cols {
            return None;
        }

        let v = &self.graph;
        v.iter().nth(i as usize * self.cols + j as usize)
    }

    fn iter(&self) -> Map {
        let rows = self.rows;
        let cols = self.cols;

        let mut new_graph = Vec::new();
        for i in 0..rows as i32 {
            for j in 0..cols as i32 {
                let tile = self.get(i, j).unwrap();
                let count = self.neighbors(i, j);
                let new_tile = match tile {
                    Tile::Empty if count == 0 => Tile::Occupied,
                    Tile::Occupied if count >= 4 => Tile::Empty,
                    _ => *tile,
                };
                new_graph.push(new_tile);
            }
        }

        Map{graph: new_graph, rows, cols }
    }

    fn iter_2(&self) -> Map {
        let rows = self.rows;
        let cols = self.cols;

        let mut new_graph = Vec::new();
        for i in 0..rows as i32 {
            for j in 0..cols as i32 {
                let tile = self.get(i, j).unwrap();
                let count = self.sight_neighbors(i, j);
                let new_tile = match tile {
                    Tile::Empty if count == 0 => Tile::Occupied,
                    Tile::Occupied if count >= 5 => Tile::Empty,
                    _ => *tile,
                };

                new_graph.push(new_tile);
            }
        }

        Map{graph: new_graph, rows, cols }
    }

    fn num_occupied(&self) -> usize {
        self.graph.iter().filter(|x| x == &&Tile::Occupied).count()
    }

    fn print(&self) {
        for (i, t) in self.graph.iter().enumerate() {
            if i % self.rows == 0 {
                println!();
            }
            print!("{}", t.to_str());
        }
        println!();
    }
}

/**
 * Load boilerplate
 */

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
