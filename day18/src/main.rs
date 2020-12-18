use std::fs;

#[derive(Debug)]
enum Expression {
    Number(isize),
    Plus(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
}

fn parse(input: &str) -> Expression {
    if let Ok(value) = input.parse::<isize>() {
        return Expression::Number(value);
    }

    let mut first_multiply_operator_position = None;
    let mut first_addition_operator_position = None;
    let mut opened_braces = 0;
    for (position, character) in input.chars().rev().enumerate() {
        let position = input.len() - position - 1;
        if character == ')' {
            opened_braces += 1;
        } else if character == '(' {
            opened_braces -= 1;
        } else if opened_braces == 0 && character == '*' {
            first_multiply_operator_position = match first_multiply_operator_position {
                None => Some(position),
                Some(value) => Some(value),
            };
        } else if opened_braces == 0 && character == '+' {
            first_addition_operator_position = match first_addition_operator_position {
                None => Some(position),
                Some(value) => Some(value),
            };
        }
    }

    match first_multiply_operator_position {
        Some(pos) => parse_sub(&input[pos .. pos + 1], &input[0..pos - 1], &input[pos + 2..]),
        None => match first_addition_operator_position {
            Some(pos) => parse_sub(&input[pos .. pos + 1], &input[0..pos - 1], &input[pos + 2..]),
            None => parse(&input[1..input.len() - 1]) 
        }
    }
}


fn parse_sub(operator: &str, lhs: &str, rhs: &str) -> Expression {
    let lhs = parse(lhs);
    let rhs = parse(rhs);
    return match operator {
        "*" => Expression::Multiply(Box::new(lhs), Box::new(rhs)),
        "+" => Expression::Plus(Box::new(lhs), Box::new(rhs)),
        _ => panic!("Unknown operator"),
    }
}

fn evaluate(input: &Expression) -> isize {
    return match input {
        Expression::Number(value) => *value,
        Expression::Plus(a, b) => evaluate(a) + evaluate(b),
        Expression::Multiply(a, b) => evaluate(a) * evaluate(b),
    }
}

fn main() {
    let input = fs::read_to_string("assets/day18.in").unwrap();

    let sum: isize = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| evaluate(&parse(line)))
        .sum();

    println!("Sum of all expressions is {}", sum);
}
