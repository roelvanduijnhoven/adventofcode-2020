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

fn main() {
    let (rules, my_ticket, nearby_ticket) = read_input("assets/example.in").unwrap();

    println!("{:?}", rules);
    println!("{:?}", my_ticket);
    println!("{:?}", nearby_ticket);
}
