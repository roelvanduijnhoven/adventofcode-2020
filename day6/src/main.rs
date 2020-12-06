use std::fs;

fn get_positive_answered_by_anyone(lines: &str) -> String {
    let mut all_positive_answers: Vec<char> = lines
        .clone()
        .chars()
        .filter(|a_char| a_char.is_alphabetic())
        .collect();

    all_positive_answers.sort();
    all_positive_answers.dedup();

    all_positive_answers.into_iter().collect()
}

fn get_positive_answered_by_everybody(lines: &str) -> String {
    let mut lines_iter = lines.split("\n");
    
    let mut remainder: Vec<char> = match lines_iter.next() {
        None => return String::from(""),
        Some(value) => value.chars().collect()
    };

    loop {
        let line = match lines_iter.next() {
            None => break,
            Some(value) => value,
        };  

        remainder.retain(|local_char| line.find(*local_char).is_some());
    }

    remainder.sort();
    remainder.into_iter().collect()
}

fn main() {
    let content = fs::read_to_string("assets/day6.in").expect("Something went wrong reading the file");

    {
        let sum: usize = content
            .split("\n\n")
            .map(|group| get_positive_answered_by_anyone(group).len())
            .sum();

        println!("Sum where anyone yes-answered is {}", sum);
    }

    {   
        let sum: usize = content
            .split("\n\n")
            .map(|group| get_positive_answered_by_everybody(group).len())
            .sum();

        println!("Sum where everybody yes-answered is {}", sum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_will_count_correct_for_anyone() {
        assert_eq!("abc", get_positive_answered_by_anyone("abc"));
        assert_eq!("abc", get_positive_answered_by_anyone("cba"));
        assert_eq!("abc", get_positive_answered_by_anyone("a\nb\nc"));
        assert_eq!("abc", get_positive_answered_by_anyone("ab\nac"));
        assert_eq!("a", get_positive_answered_by_anyone("a\na\na"));
        assert_eq!("b", get_positive_answered_by_anyone("b"));
    }

    #[test]
    fn it_will_count_correct_for_everybody() {
        assert_eq!("abc", get_positive_answered_by_everybody("abc"));
        assert_eq!("abc", get_positive_answered_by_everybody("cba"));
        assert_eq!("", get_positive_answered_by_everybody("a\nb\nc"));
        assert_eq!("a", get_positive_answered_by_everybody("ab\nac"));
        assert_eq!("a", get_positive_answered_by_everybody("a\na\na"));
        assert_eq!("b", get_positive_answered_by_everybody("b"));
    }
}