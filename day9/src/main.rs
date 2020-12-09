mod encoding;

use std::fs;
use encoding::Encryption;

fn does_sum_to(input: &[usize], start: usize, sums_to: usize) -> Option<(usize, usize)> {
    let mut sum = 0;

    let mut low: Option<usize> = None;
    let mut high: Option<usize> = None;

    for (position, value) in input[start..].iter().enumerate() {
        if sum == sums_to && position > 1 {
            return Some((low.unwrap(), high.unwrap()));
        }

        sum += value;
        
        low = match low {
            Some(now) if now < *value => Some(now),
            _ => Some(*value),
        };
        
        high = match high {
            Some(now) if now > *value => Some(now),
            _ => Some(*value),
        };
    }

    return None;
}

fn main() {
    let content = fs::read_to_string("assets/day9.in").expect("Something went wrong reading the file");
    let input: Vec<usize> = content
        .split("\n")
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let mut stream = Encryption::new(&input[0..25]);

    let unparsable_numbers: Vec<&usize> = input[25..]
        .iter()
        .filter(|number| stream.parse(**number).is_err())
        .collect();

    if unparsable_numbers.len() == 0 {
        panic!("No unparsable numbers found");
    }

    let first_unparsable = unparsable_numbers[0];
    println!("First unparsable number is {}", first_unparsable);

    for start in 0..input.len() {
        match does_sum_to(&input, start, *first_unparsable) {
            Some((low, end)) => println!("Encryption weakness is {}", low + end),
            None => (),
        }
    }
}