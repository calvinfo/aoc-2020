use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let v = load(String::from("./input"));
    let sol1 = part1(v);
    println!("Part 1: {}", sol1);

    let v2 = load(String::from("./input"));
    let sol2 = part2(v2);
    println!("Part 2: {}", sol2);
}

pub struct Password {
    lo: usize,
    hi: usize,
    character: char,
    password: String,
}

pub fn parse(line: String) -> Password {
    let mut parts = line.split(" ");
    let (lo, hi) = get_range(parts.next().unwrap().to_string());
    let character = get_char(parts.next().unwrap().to_string());
    let password = parts.next().unwrap().to_string();

    return Password{lo, hi, character, password};
}

pub fn get_char(char_str: String) -> char {
    char_str.chars().next().unwrap()
}

pub fn get_range(range: String) -> (usize, usize) {
    let mut values = range.split("-");
    let low = values.next();
    let high = values.next();
    return (low.unwrap().parse::<usize>().unwrap(), high.unwrap().parse::<usize>().unwrap());
}


pub fn load(filename: String) -> Vec<Password> {
    let mut input = match File::open(filename) {
        Ok(f) => f,
        Err(err) => panic!(err),
    };

    let mut content = String::new();
    match input.read_to_string(&mut content) {
        Err(err) => panic!(err),
        Ok(_) => {}
    }

    content
        .lines()
        .map(|x| parse(x.to_string()))
        .collect::<Vec<Password>>()
}

pub fn is_valid(p: &Password) -> bool {
    let count =  p.password.matches(p.character).count();
    count <= p.hi && count >= p.lo
}

pub fn is_valid_2(p: &Password) -> bool {
    let mut chars = p.password.chars();
    let lo_check = chars.nth(p.lo-1).unwrap() == p.character;
    let hi_check = chars.nth(p.hi-p.lo-1).unwrap() == p.character;
    (lo_check || hi_check) && !(lo_check && hi_check)
}

pub fn part1(passwords: Vec<Password>) -> usize {
    passwords.into_iter().filter(|p| is_valid(p)).count()
}

pub fn part2(passwords: Vec<Password>) -> usize {
    passwords.into_iter().filter(|p| is_valid_2(p)).count()
}

#[cfg(tests)]
mod tests {
    use super::get_range;

    #[test]
    fn test_get_range(){
        let (lo, hi) = get_range("1-10");
        assert_eq!(lo, 1);
        assert_eq!(hi, 10);
    }
}