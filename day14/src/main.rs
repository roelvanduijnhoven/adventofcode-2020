use std::fs;
use std::collections::HashMap;
use regex::Regex;
use combinations::Combinations;

#[derive(Debug)]
enum Instruction {
    UpdateMask(String),
    Write(u64, u64),
}

fn apply_mask(value: u64, mask: &str) -> String {
    let string_value = format!("{:036b}", value);
    let mut masked = String::new();
    for (position, character) in mask.chars().enumerate() {
        match character {
            '0' => masked.push_str(&string_value[position..position + 1]),
            value => masked.push(value),
        }
    }
    masked
}

fn find_all(input: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut output = vec![];

    output.push(vec![]);
    output.push(input.clone());

    for i in 1..input.len() {
        let combinations: Vec<_> = Combinations::new(input.clone(), i).collect();
        for combination in combinations {
            output.push(combination);
        }
    }

    output
}

fn main() {
    let program = load_program_from_string(&"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1");

    let contents = fs::read_to_string("assets/day14.in").expect("Something went wrong reading the file");
    let program = load_program_from_string(&contents);

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask: String = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

    for instruction in program {
        match instruction {
            Instruction::UpdateMask(new_mask) => mask = new_mask,
            Instruction::Write(memory_address, value) => { 
                let decoder = apply_mask(memory_address, &mask);
                let x_positions: Vec<usize> = decoder
                    .chars().rev().enumerate()
                    .filter(|(_, x)| *x == 'X')
                    .map(|(pos, _)| pos)
                    .collect();

                let base_value = u64::from_str_radix(&decoder.replace("X", "0"), 2).unwrap();

                let combinations = find_all(&x_positions);
                for combination in combinations {
                    let mut memory_location = base_value;
                    for position in combination {
                        memory_location = memory_location | ((2 as u64).pow(position as u32) as u64);
                    }

                    memory.insert(memory_location, value);
                }
            },
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
