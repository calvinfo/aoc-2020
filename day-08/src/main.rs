use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    let inp = load(String::from("./input"));
    let mut program = Program::from_str(&inp);

    let sol1 = part1(&program);
    println!("Part 1: {}", sol1);

    let sol2 = part2(&mut program);
    println!("Part 2: {}", sol2);
}

fn part1(program: &Program) -> i32 {
    program.execute_loop()
}

fn part2(program: &mut Program) -> i32 {
    let mut i = 0;
    while i < program.instructions.len() as i32 {
        program.swap(i);
        if !program.is_loop() {
            return program.execute_loop();
        } else {
            program.swap(i);
        }
        i += 1
    }
    0
}

/**
 * Definitions
 */

struct Program {
    instructions: Vec<Instruction>,
}

struct Instruction {
    command: String,
    modifier: i32,
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        let mut split = s.split(" ");
        let command = split.next().unwrap();
        let modifier = split.next().unwrap().parse::<i32>().unwrap();
        Instruction {
            command: String::from(command),
            modifier,
        }
    }
}

impl Program {
    fn from_str(s: &str) -> Program {
        let instructions = s.lines().map(|s| Instruction::from_str(s)).collect();
        Program { instructions }
    }

    fn execute_loop(&self) -> i32 {
        let instructions = self.instructions.as_slice();
        let mut counter: i32 = 0;
        let mut acc = 0;
        let mut processed = HashSet::new();

        while !processed.contains(&counter) && counter < instructions.len() as i32 {
            processed.insert(counter);
            let i = &instructions[counter as usize];

            match i.command.as_str() {
                "nop" => {}
                "acc" => acc += i.modifier,
                "jmp" => counter += i.modifier - 1,
                _ => {}
            }
            counter += 1;
        }
        acc
    }

    fn is_loop(&self) -> bool {
        let instructions = self.instructions.as_slice();
        let mut counter: i32 = 0;
        let mut processed = HashSet::new();

        while !processed.contains(&counter) {
            if counter >= instructions.len() as i32 {
                return false;
            }

            processed.insert(counter);
            let i = &instructions[counter as usize];

            match i.command.as_str() {
                "nop" => {}
                "acc" => {}
                "jmp" => counter += i.modifier - 1,
                _ => {}
            }
            counter += 1;
        }
        true
    }

    fn swap(&mut self, i: i32) {
        let instructions = self.instructions.as_slice();
        let inst = &instructions[i as usize];
        let new_cmd = match inst.command.as_str() {
            "jmp" => String::from("nop"),
            "nop" => String::from("jmp"),
            "acc" => String::from("acc"),
            _ => String::from(""),
        };
        let new_inst = Instruction {
            command: new_cmd,
            modifier: inst.modifier.clone(),
        };

        self.instructions.remove(i as usize);
        self.instructions.insert(i as usize, new_inst);
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
