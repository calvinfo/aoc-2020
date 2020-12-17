use std::collections::HashMap;
use std::collections::HashSet;
use std::{fs::File, io::Read};

fn main() {
    let test_state = parse(load(String::from("./inp-test")));
    let input = parse(load(String::from("./input")));

    // // // Part 1
    let test_sol = part1(test_state);
    println!("Part 1 test: {}", test_sol);
    let sol1 = part1(input);
    println!("Part 1: {}", sol1);

    // // // Part 2
    let test_state = parse(load(String::from("./inp-test")));
    let input = parse(load(String::from("./input")));

    let test_sol2 = part2(test_state);
    println!("Part 2 test: {}", test_sol2);
    let sol2 = part2(input);
    println!("Part 2: {}", sol2);
}

/**
 * Solution
 */

fn part1(cubes: HashSet<Cube>) -> i32 {
    let mut w = World { cubes };
    for _ in 0..6 {
        w = w.next(false);
    }
    w.cubes.len() as i32
}

fn part2(cubes: HashSet<Cube>) -> i32 {
    let mut w = World { cubes };
    for _ in 0..6 {
        w = w.next(true);
    }
    w.cubes.len() as i32
}

/**
 * Definition
 */

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
struct Cube {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

struct World {
    cubes: HashSet<Cube>,
}

impl World {
    fn next(self, four_d: bool) -> World {
        let mut neighbors: HashMap<Cube, u32> = HashMap::new();
        for cube in self.cubes.iter() {
            for x in -1..2 {
                for y in -1..2 {
                    for z in -1..2 {
                        for w in -1..2 {
                            if !four_d && w != 0 {
                                continue;
                            }

                            if x == 0 && y == 0 && z == 0 && w == 0 {
                                continue;
                            }

                            let neighbor = Cube {
                                w: cube.w + w,
                                x: cube.x + x,
                                y: cube.y + y,
                                z: cube.z + z,
                            };
                            let val = match neighbors.get(&neighbor) {
                                None => 1u32,
                                Some(x) => x + 1,
                            };
                            neighbors.insert(neighbor, val);
                        }
                    }
                }
            }
        }

        let mut new_cubes = HashSet::new();
        for (cube, count) in neighbors {
            let active = self.cubes.contains(&cube);
            if active && (count == 2 || count == 3) {
                new_cubes.insert(cube);
            }
            if !active && count == 3 {
                new_cubes.insert(cube);
            }
        }
        World { cubes: new_cubes }
    }
}

/**
 * Load boilerplate
 */

fn parse(s: String) -> HashSet<Cube> {
    let mut v = HashSet::new();
    for (x, line) in s.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                v.insert(Cube {
                    w: 0,
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                });
            }
        }
    }
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
