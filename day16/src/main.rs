use std::fs;
use std::io;
use regex::Regex;

#[derive(Debug)]
struct Ticket {
    fields: Vec<usize>,
}

#[derive(Debug)]
struct Rule {
    name: String,
    terms: Vec<(usize, usize)>,
}

#[derive(Debug)]
enum ImportError {
    IoError(io::Error),
    ParseError(),
}

impl From<io::Error> for ImportError {
    fn from(error: io::Error) -> Self {
        ImportError::IoError(error)
    }
}

fn read_input(filename: &str) -> Result<(Vec<Rule>, Ticket, Vec<Ticket>), ImportError> {
    let contents = fs::read_to_string(filename)?;
    let parts: Vec<&str> = contents.split("\n\n").collect();

    // TODO Find out how to better map Option to my custom Error type, without duplicating the ok_or()'s and
    // remove the .unwraps().
    
    // Part 1
    let mut rules = vec![];
    for line in parts.get(0).ok_or(ImportError::ParseError())?.split("\n") {
        let regex = Regex::new(r"^([^:]+): (.*)$").unwrap();
        let matches = regex.captures(line).ok_or(ImportError::ParseError())?;
        rules.push(Rule {
            name: matches.get(1).ok_or(ImportError::ParseError())?.as_str().to_owned(),
            terms: matches.get(2).ok_or(ImportError::ParseError())?.as_str()
                .split(" or ")
                .map(|item| {
                    let parts: Vec<&str>= item.split("-").collect();
                    (
                        parts.get(0).unwrap().parse::<usize>().unwrap(),
                        parts.get(1).unwrap().parse::<usize>().unwrap()
                    )
                })
                .collect()
        })
    }

    // Part 2
    let lines: Vec<&str> = parts.get(1).ok_or(ImportError::ParseError())?.split("\n").collect();
    let ticket = parse_ticket(lines.get(1).ok_or(ImportError::ParseError())?).ok_or(ImportError::ParseError())?;

    // Part 3
    let nearby_ticket = parts.get(2).ok_or(ImportError::ParseError())?
        .split("\n")
        .skip(1)
        .map(|line| parse_ticket(&line).unwrap())
        .collect::<Vec<Ticket>>();

    Ok((rules, ticket, nearby_ticket))
}

fn parse_ticket(line: &str) -> Option<Ticket> {
    let fields = line
        .split(",")
        .map(|field| field.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    Some(Ticket { fields })
}

fn is_valid_ticket(ticket: &Ticket, rules: &Vec<Rule>) -> bool {
    for field in &ticket.fields {
        if !is_valid_field(*field, &rules) {
            return false;
        }
    }
    
    true
}

fn is_valid_field(field: usize, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if validates_rule(field, &rule)    {
            return true;
        }
    }
    false
}

fn validates_rule(field: usize, rule: &Rule) -> bool {
    for term in &rule.terms {
        if field >= term.0 && field <= term.1 {
            return true;
        }
    }
    false
}

fn learn_rule(rule: &Rule, field_positions: Vec<usize>, tickets: &Vec<&Ticket>) -> Option<usize> {
    let mut possible_positions = vec![];
    for field_position in field_positions {
        let mut is_possible = true;
        for ticket in tickets {
            if !validates_rule(ticket.fields[field_position], &rule) {
                is_possible = false;
            }
        }

        if is_possible {
            possible_positions.push(field_position);
        }
    }

    if possible_positions.len() == 1 {
        return Some(possible_positions[0]);
    }

    None
}

fn main() {
    let (rules, my_ticket, nearby_tickets) = read_input("assets/day16.in").unwrap();
    // let (rules, my_ticket, nearby_tickets) = read_input("assets/example.in").unwrap();

    println!("{:?}", rules);
    println!("{:?}", my_ticket);
    println!("{:?}", nearby_tickets);

    // Problem 1
    let mut sum_of_invalid_fields = 0;
    for ticket in &nearby_tickets {
        for field in &ticket.fields {
            if !is_valid_field(*field, &rules) {
                sum_of_invalid_fields += field;
            }
        }
    }

    println!("Sum of troubled fields is {}", sum_of_invalid_fields);

    // Problem 2
    let mut valid_nearby_tickets = vec![];
    for ticket in &nearby_tickets {
        if is_valid_ticket(&ticket, &rules) {
            valid_nearby_tickets.push(ticket);
        }
    }

    let mut remaining_fields: Vec<usize> = (0..rules.len()).collect();

    loop {
        if remaining_fields.len() == 0 {
            break
        }

        for rule in &rules {
            match learn_rule(&rule, remaining_fields.clone(), &valid_nearby_tickets) {
                Some(value) => {
                    println!("Rule {} maps to field {}", rule.name, value);
                    remaining_fields.retain(|position| *position != value);
                },
                None => (),
            }
        }
    }

    // Rule departure time maps to field 2
    // Rule departure platform maps to field 4
    // Rule departure station maps to field 8
    // Rule departure location maps to field 12
    // Rule departure track maps to field 14
    // Rule departure date maps to field 19

    // TODO automate the last part.
    
    println!("{}", my_ticket.fields[2] * my_ticket.fields[4] * my_ticket.fields[8] * my_ticket.fields[12] * my_ticket.fields[14] * my_ticket.fields[19]);

}