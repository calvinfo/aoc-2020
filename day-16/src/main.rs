use std::{fs::File, io::Read};

fn main() {
    let test_state = parse(load(String::from("./inp-test")));
    let input = parse(load(String::from("./input")));

    // // Part 1
    let test_sol = part1(&test_state);
    println!("Part 1 test: {}", test_sol);
    let sol1 = part1(&input);
    println!("Part 1: {}", sol1);

    // // Part 2
    let test_sol2 = part2(&test_state, "class".to_string());
    println!("Part 2 test: {}", test_sol2);
    let sol2 = part2(&input, "departure".to_string());
    println!("Part 2: {}", sol2);
}

/**
 * Solution
 */

fn part1(s: &State) -> i32 {
    s.nearby.iter().map(|t| t.invalid_sum(&s.rules)).sum()
}

fn dfs(
    prefix: Vec<RuleIndex>,
    current: Vec<RuleIndex>,
    remaining: Vec<Vec<RuleIndex>>,
) -> Option<Vec<RuleIndex>> {
    for val in current {
        if prefix.iter().any(|x| x.index == val.index) {
            continue;
        }

        let mut new_prefix = prefix.clone();
        new_prefix.push(val);
        if remaining.len() == 0 {
            return Some(new_prefix);
        }

        let mut new_remaining = remaining.clone();
        let new_current = new_remaining.remove(0);

        let found = dfs(new_prefix, new_current, new_remaining);
        match found {
            None => continue,
            Some(path) => return Some(path),
        }
    }
    None
}

fn part2(s: &State, ticket_prefix: String) -> u64 {
    let valid_tickets: Vec<&Ticket> = s
        .nearby
        .iter()
        .filter(|t| t.invalid_count(&s.rules) == 0)
        .collect::<Vec<&Ticket>>();
    let mut mul = 1u64;
    let num_rules = s.rules.len();

    // first initialize search space
    let mut positions: Vec<Vec<RuleIndex>> = Vec::new();
    for rule in s.rules.iter() {
        let mut v = Vec::new();
        for ticket_idx in 0..num_rules {
            // traverse each ticket, if it's invalid, it's invalid
            let mut valid = true;
            for ticket in valid_tickets.iter() {
                if rule.clone().eval(&ticket.numbers[ticket_idx]) == false {
                    valid = false;
                    break;
                }
            }
            // if it's valid, add to our vector
            if valid {
                v.push(RuleIndex {
                    name: rule.name.clone(),
                    index: ticket_idx.clone(),
                });
            }
        }
        positions.push(v);
    }

    // next, sort by most limited positions first
    positions.sort_by(|a, b| a.len().cmp(&b.len()));

    // now, DFS
    let prefix = Vec::new();
    let current = positions.remove(0);
    let result = dfs(prefix, current, positions).unwrap();

    for rule_idx in result.iter() {
        if rule_idx.name.starts_with(&ticket_prefix) {
            mul *= s.ticket.numbers[rule_idx.index] as u64;
        }
    }
    mul
}
/**
 * Definition
 */

#[derive(Debug)]
struct State {
    rules: Vec<Rule>,
    ticket: Ticket,
    nearby: Vec<Ticket>,
}

#[derive(Debug, Clone)]
struct RuleIndex {
    name: String,
    index: usize,
}

#[derive(Debug)]
struct Ticket {
    numbers: Vec<i32>,
}

impl Ticket {
    fn from_str(s: &str) -> Ticket {
        let numbers = s.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        Ticket { numbers }
    }

    fn invalid_count(&self, v: &Vec<Rule>) -> usize {
        self.numbers
            .iter()
            .filter(|i| !v.iter().any(|rule| rule.clone().eval(&i)))
            .count()
    }

    fn invalid_sum(&self, v: &Vec<Rule>) -> i32 {
        self.numbers
            .iter()
            .filter(|i| !v.iter().any(|rule| rule.clone().eval(&i)))
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    ranges: (Range, Range),
}

impl Rule {
    fn from_str(s: &str) -> Rule {
        let mut it = s.split(": ");
        let name = it.next().unwrap();
        let rest = it.next().unwrap();

        it = rest.split(" or ");
        let lo = it.next().unwrap();
        let hi = it.next().unwrap();
        Rule {
            name: name.to_string(),
            ranges: (Range::from_str(lo), Range::from_str(hi)),
        }
    }

    fn eval(self, i: &i32) -> bool {
        self.ranges.0.eval(*i) || self.ranges.1.eval(*i)
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    lo: i32,
    hi: i32,
}

impl Range {
    fn from_str(s: &str) -> Range {
        let mut items = s.split("-").map(|x| x.parse::<i32>().unwrap());
        let lo = items.next().unwrap();
        let hi = items.next().unwrap();
        Range { lo, hi }
    }

    fn eval(self, i: i32) -> bool {
        i >= self.lo && i <= self.hi
    }
}

/**
 * Load boilerplate
 */

fn parse(s: String) -> State {
    let mut it = s.split("your ticket:");

    let first = it.next().unwrap();
    let rest = it.next().unwrap();

    it = rest.split("nearby tickets:");
    let second = it.next().unwrap();
    let third = it.next().unwrap();

    let rules = first
        .lines()
        .filter(|x| x.len() > 0)
        .map(|x| Rule::from_str(x))
        .collect();
    let ticket = second
        .lines()
        .filter(|x| x.len() > 0)
        .map(|x| Ticket::from_str(x))
        .next()
        .unwrap();
    let nearby = third
        .lines()
        .filter(|x| x.len() > 0)
        .map(|x| Ticket::from_str(x))
        .collect();

    State {
        rules,
        ticket,
        nearby,
    }
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
