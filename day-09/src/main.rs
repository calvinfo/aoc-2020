use std::{fs::File, io::Read};

fn main() {
    let test = load(String::from("./inp-test"));
    let test_v = load_xmas(test);

    let test_sol = part1(&test_v, 5);
    println!("Part 1 test: {}", test_sol);

    let test_sol2 = part2(&test_v, 127);
    println!("Part 2 test: {}", test_sol2);

    let inp = load(String::from("./input"));
    let v = load_xmas(inp);

    let sol1 = part1(&v, 25);
    println!("Part 1: {}", sol1);

    let sol2 = part2(&v, sol1);
    println!("Part 2: {}", sol2);
}

fn part1(v: &Vec<u64>, i: i32) -> u64 {
    let mut offset = 0;
    let output = v.iter().skip(i as usize).find(|val| {
        for x in 0..i {
            for y in 0..i {
                if x != y && &v[(x + offset) as usize] + &v[(y + offset) as usize] == **val {
                    offset += 1;
                    return false;
                }
            }
        }
        offset += 1;
        true
    });

    *output.unwrap()
}

fn part2(v: &Vec<u64>, target: u64) -> u64 {
    let mut i = 0;
    let mut j = 1;

    while true {
        let guess: u64 = v.iter().skip(i).take(j - i).sum();
        if guess == target {
            break;
        }
        if guess < target {
            j += 1;
        }

        if guess > target {
            i += 1;
        }
    }

    v.iter().skip(i).take(j - i).min().unwrap() + v.iter().skip(i).take(j - i).max().unwrap()
}

/**
 * Load boilerplate
 */

fn load_xmas(s: String) -> Vec<u64> {
    s.lines().map(|x| x.parse::<u64>().unwrap()).collect()
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
