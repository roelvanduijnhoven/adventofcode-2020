use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use combinations::Combinations;

fn main() {
    let input = read_integers_from_file("assets/day1.input").unwrap();
    let pairs = get_tuples_summing_to(input, 3, 2020); 
    let first = &pairs[0];
    println!("{:?}", first[0] * first[1] * first[2]);
}

fn read_integers_from_file(input_path: &str) -> Result<Vec<i32>, Error> {
    let reader = BufReader::new(File::open(input_path)?);
    reader
        .lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))    
        .collect()  
}

fn get_tuples_summing_to(list: Vec<i32>, tuple_length: usize, sum: i32) -> Vec<Vec<i32>> {
    let mut pairs: Vec<Vec<i32>> = vec![];

    if list.len() < tuple_length {
        return pairs;
    }

    let combinations: Vec<_> = Combinations::new(list, tuple_length).collect();
    for combination in combinations {
        let mut local_sum = 0;
        for number in &combination {
            local_sum += number;
        }
        if local_sum == sum {
            pairs.push(combination.clone());
        }
    }

    return pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_will_not_sum_itself() {
        let list = vec![100];
        assert_eq!(0, get_tuples_summing_to(list, 2, 200).len());
    }

    #[test]
    fn it_will_work_with_negative_numbers() {
        let list = vec![-100, 300];
        assert_eq!(vec![vec![-100, 300]], get_tuples_summing_to(list, 2, 200));
    }

    #[test]
    fn it_will_yield_multiple_pairs() {
        let list = vec![-100, 300, 500, -300];
        assert_eq!(vec![vec![-300, 500], vec![-100, 300]], get_tuples_summing_to(list, 2, 200));
    }    

    #[test]
    fn it_finds_result_in_small_example() {
        let list = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(vec![vec![299, 1721]], get_tuples_summing_to(list, 2, 2020));
    }
}