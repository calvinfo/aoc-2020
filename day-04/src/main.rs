use regex::Regex;
use std::io::Read;
use std::{collections::HashMap, fs::File};

fn main() {
    let inp = load(String::from("./input"));
    let passports = load_passports(inp);

    let sol1 = part1(&passports);
    println!("Part 1: {}", sol1);

    let sol2 = part2(&passports);
    println!("Part 2: {}", sol2);
}

pub fn part1(v: &Vec<Passport>) -> usize {
    v.iter().filter(|x| x.is_valid()).count()
}

pub fn part2(v: &Vec<Passport>) -> usize {
    v.iter().filter(|x| x.is_valid_2().unwrap()).count()
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
            let k = s.next().unwrap().to_string();
            let v = s.next().unwrap().to_string();
            items.insert(k, v);
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

    fn is_valid_2(&self) -> Option<bool>  {
        if !self.is_valid() {
            return Some(false);
        }

        let m = &self.map;
        let byr = m.get("byr")?;
        let iyr = m.get("iyr")?;
        let eyr = m.get("eyr")?;
        let hgt = m.get("hgt")?;
        let pid = m.get("pid")?;
        let hcl = m.get("hcl")?;
        let ecl = m.get("ecl")?;

        Some(valid_range(byr, "1920", "2002")
            && valid_range(iyr, "2010", "2020")
            && valid_range(eyr, "2020", "2030")
            && valid_height(hgt)
            && valid_id(pid)
            && valid_hair(hcl)
            && valid_eye(ecl))
    }
}

pub fn valid_range(s: &String, lo: &str, hi: &str) -> bool {
    s.len() == hi.len() && s >= &lo.to_string() && s <= &hi.to_string()
}

pub fn valid_hair(s: &String) -> bool {
    let r = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    r.is_match(s)
}

pub fn valid_id(s: &String) -> bool {
    let r = Regex::new(r"^[0-9]{9}$").unwrap();
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
