use regex::Regex;
use std::io::Read;
use std::{collections::HashMap, fs::File};

fn main() {
    let mut inp = load(String::from("./input"));
    let mut passports = load_passports(inp);

    let sol1 = part1(&passports);
    println!("Part 1: {}", sol1);

    let sol2 = part2(&passports);
    println!("Part 2: {}", sol2);
}

pub fn part1(v: &Vec<Passport>) -> usize {
    v.iter().filter(|x| x.is_valid()).count()
}

pub fn part2(v: &Vec<Passport>) -> usize {
    v.iter().filter(|x| x.is_valid_2()).count()
}

/**
 * Definitions
 */

pub struct Passport {
    map: HashMap<String, String>,
}

impl Passport {
    fn from_str(str: String) -> Passport {
        let line = str.replace("\n", " ");
        let mut items = HashMap::new();
        for mut s in line.split(" ").map(|x| x.split(":")) {
            let k = s.next().unwrap();
            let v = s.next().unwrap();
            items.insert(String::from(k), String::from(v));
        }

        Passport { map: items }
    }

    fn is_valid(&self) -> bool {
        let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        required
            .iter()
            .filter(|x| self.map.contains_key(**x))
            .count()
            == required.len()
    }

    fn is_valid_2(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        let m = &self.map;
        let byr = m.get("byr").unwrap();
        let iyr = m.get("iyr").unwrap();
        let eyr = m.get("eyr").unwrap();
        let hgt = m.get("hgt").unwrap();
        let pid = m.get("pid").unwrap();
        let hcl = m.get("hcl").unwrap();
        let ecl = m.get("ecl").unwrap();

        if byr.len() != 4 || !valid_range(byr, "1920", "2002") {
            return false;
        }

        if iyr.len() != 4 || !valid_range(iyr, "2010", "2020") {
            return false;
        }

        if eyr.len() != 4 || !valid_range(eyr, "2020", "2030") {
            return false;
        }

        if !valid_height(hgt) {
            return false;
        }

        if !valid_id(pid) {
            return false;
        }

        if !valid_hair(hcl) {
            return false;
        }

        if !valid_eye(ecl) {
            return false;
        }

        true
    }
}

pub fn valid_range(s: &String, lo: &str, hi: &str) -> bool {
    s >= &String::from(lo) && s <= &String::from(hi)
}

pub fn valid_hair(s: &String) -> bool {
    if s.len() != 7 {
        return false;
    }
    let r = Regex::new(r"#[a-f0-9]{6}").unwrap();
    r.is_match(s)
}

pub fn valid_id(s: &String) -> bool {
    if s.len() != 9 {
        return false;
    }
    let r = Regex::new(r"[0-9]{9}").unwrap();
    r.is_match(s)
}

pub fn valid_eye(s: &String) -> bool {
    let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    for color in colors {
        if color == s {
            return true;
        }
    }
    false
}

pub fn valid_height(s: &String) -> bool {
    if s.ends_with("cm") {
        if s.len() != 5 {
            return false;
        }
        return valid_range(&String::from(&s[0..3]), "150", "193");
    } else if s.ends_with("in") {
        if s.len() != 4 {
            return false;
        }
        return valid_range(&String::from(&s[0..2]), "59", "76");
    }
    false
}

/**
 * Load boilerplate
 */

pub fn load_passports(inp: String) -> Vec<Passport> {
    inp.split("\n\n")
        .map(|x| Passport::from_str(String::from(x)))
        .collect()
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
