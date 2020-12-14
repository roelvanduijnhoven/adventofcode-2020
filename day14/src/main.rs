use std::fs;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    UpdateMask(String),
    Write(u64, u64),
}

fn apply_mask(value: u64, mask: &str) -> u64 {
    let mut masked = value;
    for (position, character) in mask.chars().rev().enumerate() {
        masked = match character {
            '1' => masked | ((2 as usize).pow(position as u32) as u64),
            '0' => masked & !((2 as usize).pow(position as u32) as u64),
            _ => masked,
        }
    }
    masked
}

fn main() {
    let program = load_program_from_string(&"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0");

    let contents = fs::read_to_string("assets/day14.in").expect("Something went wrong reading the file");
    let program = load_program_from_string(&contents);

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask: String = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

    for instruction in program {
        match instruction {
            Instruction::UpdateMask(new_mask) => mask = new_mask,
            Instruction::Write(memory_address, value) => { memory.insert(memory_address, apply_mask(value, &mask)); },
        }
    }

    let mut sum = 0;
    for memory_location in memory.keys() {
        sum += memory.get(memory_location).unwrap();
    }

    println!("Sum of memory is {}", sum);
}

fn load_program_from_string(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    let mask_regex = Regex::new(r"^mask = ([X10]{36})$").unwrap();
    let write_regex = Regex::new(r"^mem\[([0-9]+)\] = ([0-9]+)$").unwrap();

    for line in input.split("\n") {
        if let Some(captures) = mask_regex.captures(line) {
            instructions.push(Instruction::UpdateMask(captures.get(1).unwrap().as_str().to_owned()));
        } else if let Some(captures) = write_regex.captures(line) {
            instructions.push(Instruction::Write(
                captures.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<u64>().unwrap()
            ));
        }
    }

    instructions
}
