#[derive(Debug)]
#[derive(PartialEq)]
pub enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug)]
pub struct SyntaxError {}

pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn from_string(input: &str) -> Result<Program, SyntaxError> {
        let mut instructions: Vec<Instruction> = vec![];

        for line in input.split("\n") {
            let parts: Vec<&str> = line.split(' ').collect();
            instructions.push(match parts[0] {
                "nop" => Instruction::Nop(parts[1].parse::<i32>().unwrap()),
                "acc" => Instruction::Acc(parts[1].parse::<i32>().unwrap()),
                "jmp" => Instruction::Jmp(parts[1].parse::<i32>().unwrap()),
                _ => return Err(SyntaxError {})
            });
        }

        Ok(Program { instructions })
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_will_work() {
        let program = Program::from_string(
"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
).unwrap();

        assert_eq!(9, program.instructions().len());

        // TODO Use pattern matching instead, so we don't need PartialEq?
        assert_eq!(Instruction::Nop(0), *program.instructions().get(0).unwrap());
        assert_eq!(Instruction::Acc(1), *program.instructions().get(1).unwrap());
        assert_eq!(Instruction::Jmp(4), *program.instructions().get(2).unwrap());
        assert_eq!(Instruction::Acc(3), *program.instructions().get(3).unwrap());
        assert_eq!(Instruction::Jmp(-3), *program.instructions().get(4).unwrap());
        assert_eq!(Instruction::Acc(-99), *program.instructions().get(5).unwrap());
        assert_eq!(Instruction::Acc(1), *program.instructions().get(6).unwrap());
        assert_eq!(Instruction::Jmp(-4), *program.instructions().get(7).unwrap());
        assert_eq!(Instruction::Acc(6), *program.instructions().get(8).unwrap());
    }
}