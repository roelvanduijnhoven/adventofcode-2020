use std::fs;

#[derive(Debug)]
struct Instruction(char, isize);

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .split("\n")
        .map(|line| Instruction(line.chars().next().unwrap(), line[1..].parse::<isize>().unwrap()))
        .collect()
}

fn get_end_coordinate(instructions: Vec<Instruction>) -> (isize, isize) {
    let mut waypoint = (10, 1);
    let mut ship = (0, 0);

    for Instruction(action, value) in instructions {
        match action {
            'N' => waypoint.1 += value,
            'E' => waypoint.0 += value,
            'S' => waypoint.1 -= value,
            'W' => waypoint.0 -= value,

            'L' => waypoint = match value {
                0 => waypoint,
                90 => (-waypoint.1, waypoint.0),                
                180 => (-waypoint.0, -waypoint.1),
                270 => (waypoint.1, -waypoint.0),
                _ => panic!("Error"),
            },            
            'R' => waypoint = match value {
                0 => waypoint,
                90 => (waypoint.1, -waypoint.0),
                180 => (-waypoint.0, -waypoint.1),
                270 => (-waypoint.1, waypoint.0),
                _ => panic!("Error"),
            },

            'F' => {
                ship.0 += waypoint.0 * value;
                ship.1 += waypoint.1 * value;
            },
            _ => panic!("Error"),
        }
    }

    ship
}

fn main() {
    let input = fs::read_to_string("assets/day12.in").expect("Something went wrong reading the file");
    println!("({:?})", get_end_coordinate(parse_instructions(&input)));
}