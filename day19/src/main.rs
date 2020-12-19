#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Sequence(Vec<Rule>),
    Or(Box<Rule>, Box<Rule>),
}

fn read_input(input: &str) -> (Rule, Vec<String>) {
    let contents = std::fs::read_to_string(input).unwrap();
    let parts: Vec<&str> = contents.split("\n\n").collect();

    let rule = read_rule_tree(parts[0]);

    let messages: Vec<String> = parts[1]
        .split("\n")
        .map(|item| String::from(item))
        .collect();

    (rule, messages)
}

fn read_rule_tree(input: &str) -> Rule {
    let raw_rules: Vec<String> = input
        .split("\n")
        .map(|item| String::from(&item[item.find(':').unwrap() + 2..]))
        .collect();

    recursive_parse(&raw_rules, &raw_rules[0])
}

// Uses more space than necessary because it will duplicate identical rules in the tree.
fn recursive_parse(raw_rules: &Vec<String>, rule: &str) -> Rule {
    if &rule[0..1] == "\"" {
        return Rule::Char(rule[1..2].parse::<char>().unwrap());
    } else if let Some(position) = rule.find("|") {
        return Rule::Or(
            Box::new(recursive_parse(&raw_rules, &rule[0..position - 1])),
            Box::new(recursive_parse(&raw_rules, &rule[position + 2..]))
        );
    } else {
        return Rule::Sequence(
            rule.split(" ")
                .map(|value| value.parse::<usize>().unwrap())
                .map(|rule_number| recursive_parse(&raw_rules, &raw_rules[rule_number]))
                .collect::<Vec<Rule>>()
        );
    }
}

fn valid_message(message: &str, rule: &Rule, start: usize) -> Option<usize> {
    if start >= message.len() {
        return None;
    }

    let result = match rule {
        Rule::Char(x) => {
            if message[start..start + 1].parse::<char>().unwrap() == *x { 
                Some(start + 1)
            } else {
                None
            }
        },
        Rule::Sequence(rules) => {
            let mut position = start;
            let mut iter = rules.iter();
            loop {
                match iter.next() {
                    None => break,
                    Some(local_rule) => {
                        match valid_message(&message, &local_rule, position) {
                            None => return None,
                            Some(new_position) => position = new_position,
                        }
                    }
                }
            }

            Some(position)
        },
        Rule::Or(a, b) => {
            let a = valid_message(message, &a, start);
            let b = valid_message(message, &b, start);

            match a {
                None => match b {
                    None => None,
                    value => value,
                },
                value => value,
            }
        }
    };

    result
}

fn main() {
    let (rule, messages) = read_input("assets/simple.in");
    let (rule, messages) = read_input("assets/example.in");
    let (rule, messages) = read_input("assets/day19.in");

    let mut count = 0;
    for message in messages {
        let result = valid_message(&message, &rule, 0);
        match result {
            Some(position) if position == message.len() => count += 1,
            _ => continue,
        }
    }

    println!("{} valid messages", count);
        
}
