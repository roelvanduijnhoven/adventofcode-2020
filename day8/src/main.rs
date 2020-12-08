mod program;

use std::fs;
use program::Program;
use program::Instruction;

fn test_mutated_program(program: &Program, intruction: Option<i32>) -> Option<i32> {
    let mut visited: Vec<i32> = vec![];

    let mut program_counter: i32 = 0;
    let mut accumulator: i32 = 0;

    loop {
        if program_counter == program.instructions().len() as i32 {
            break;
        }

        if visited.contains(&program_counter) {
            return None;
        }

        visited.push(program_counter);

        match intruction {
            Some(i) if program_counter == i => match program.instructions().get(program_counter as usize).unwrap() {
                Instruction::Nop(value) => program_counter += value,
                Instruction::Acc(value) => {
                    accumulator += value;
                    program_counter += 1;
                },
                Instruction::Jmp(_) => program_counter += 1,
            },
            _ => match program.instructions().get(program_counter as usize).unwrap() {
                Instruction::Nop(_) => program_counter += 1,
                Instruction::Acc(value) => {
                    accumulator += value;
                    program_counter += 1;
                },
                Instruction::Jmp(value) => program_counter += value
            }
        }
    }

    return Some(accumulator);
}

fn main() {
    let content = fs::read_to_string("assets/day8.in").expect("Something went wrong reading the file");
    let program = Program::from_string(&content).unwrap();

    for i in 0..(program.instructions().len() as i32) {
        match test_mutated_program(&program, Some(i)) {
            None => (),
            Some(value) => println!("{}", value),
        }
    }
}
