use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
struct PasswordRequirement {
    character: char,
    minimum_occurrence: usize,
    maximum_occurrence: usize
}

#[derive(Debug)]
struct PositionalPasswordRequirement {
    character: char,
    should_occur_at: Vec<usize>,
}

impl PasswordRequirement {
    pub fn new(character: char, minimum_occurrence: usize, maximum_occurrence: usize) -> PasswordRequirement {
        PasswordRequirement {
            character: character,
            minimum_occurrence: minimum_occurrence,
            maximum_occurrence: maximum_occurrence
        }
    }
}

impl PositionalPasswordRequirement {
    pub fn new(character: char, should_occur_at: Vec<usize>) -> PositionalPasswordRequirement {
        PositionalPasswordRequirement {
            character: character,
            should_occur_at: should_occur_at,
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn read_input_from_file(file_path: &str) -> Result<Vec<(String, PasswordRequirement)>, Error> {
    let lines = match read_lines(file_path) {
        Err(error) => return Err(error),
        Ok(lines) => lines,
    };

    let mut output = vec![];
    for line in lines {
        if let Ok(input_line) = line {
            let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
            let result = re.captures(&input_line).unwrap();
            output.push((
                result[4].to_owned(),
                PasswordRequirement::new(
                    result[3].parse::<char>().unwrap(),
                    result[1].parse::<usize>().unwrap(),
                    result[2].parse::<usize>().unwrap()
                )
            ))
        }
    }

    return Ok(output);
}

fn read_second_puzzle_input_from_file(file_path: &str) -> Result<Vec<(String, PositionalPasswordRequirement)>, Error> {
    let lines = match read_lines(file_path) {
        Err(error) => return Err(error),
        Ok(lines) => lines,
    };

    let mut output = vec![];
    for line in lines {
        if let Ok(input_line) = line {
            let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
            let result = re.captures(&input_line).unwrap();
            output.push((
                result[4].to_owned(),
                PositionalPasswordRequirement::new(
                    result[3].parse::<char>().unwrap(),
                    vec![result[1].parse::<usize>().unwrap() - 1, result[2].parse::<usize>().unwrap() - 1]
                )
            ))
        }
    }

    return Ok(output);
}

fn validates_policy(password: &str, policy: PasswordRequirement) -> bool {
    let mut character_occurence = 0;

    for character in password.chars() {
        if policy.character == character {
            character_occurence += 1;
        }
    }
    
    return policy.minimum_occurrence <= character_occurence && character_occurence <= policy.maximum_occurrence;
}

fn validates_positional_password_policy(password: &str, policy: PositionalPasswordRequirement) -> bool {
    let mut character_occurence = 0;
    for (index, character) in password.chars().enumerate() {
        for position in &policy.should_occur_at {
            if policy.character == character && *position == index {
                character_occurence += 1;
            }
        }
    }
    
    return character_occurence == 1;
}

fn main() {
    let input = read_second_puzzle_input_from_file("assets/day2.input").unwrap();
    println!("{:?}", input[0]);

    let mut match_policy = 0;
    for (password, policy) in input {
        if validates_positional_password_policy(&password, policy) {
            match_policy += 1;
        }
    }

    println!("{} passwords validated policy.", match_policy);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_will_validate_password() {
        assert!(validates_policy("abcde", PasswordRequirement::new('a', 1, 3)));
        assert_eq!(false, validates_policy("cdefg", PasswordRequirement::new('b', 1, 3)));
        assert!(validates_policy("ccccccccc", PasswordRequirement::new('c', 2, 9)));
        assert!(validates_policy("a", PasswordRequirement::new('a', 1, 1)));
        assert!(validates_policy("a", PasswordRequirement::new('a', 0, 1)));
        assert!(validates_policy("", PasswordRequirement::new('a', 0, 1)));
        assert_eq!(false, validates_policy("a", PasswordRequirement::new('a', 2, 2)));
    }

    #[test]
    fn it_will_validate_positional_password() {
        assert!(validates_positional_password_policy("abcde", PositionalPasswordRequirement::new('a', vec![0, 2])));
        assert_eq!(false, validates_positional_password_policy("cdefg", PositionalPasswordRequirement::new('b', vec![1, 3])));
        assert!(validates_positional_password_policy("caccccccc", PositionalPasswordRequirement::new('c', vec![2, 9])));
        assert!(validates_positional_password_policy("a", PositionalPasswordRequirement::new('a', vec![0, 1])));
        assert!(validates_positional_password_policy("a", PositionalPasswordRequirement::new('a', vec![0, 2])));
        assert_eq!(false, validates_positional_password_policy("", PositionalPasswordRequirement::new('a', vec![0, 1])));
    }
}