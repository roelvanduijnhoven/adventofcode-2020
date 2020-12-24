use std::collections::HashMap;
use std::fs;

fn coordinate(input: &str) -> (isize, isize) {
    let mut x: isize = 0;
    let mut y: isize = 0;

    let mut position = 0;
    loop {
        if position >= input.len() {
            break;
        }

        match &input[position..position + 1] {
            "n" => match &input[position + 1..position + 2] {
                "e" => { position += 2; x += 1; y -= 1; },
                "w" => { position += 2; y -= 1; },
                _ => panic!("Not expected!"),
            },
            "e" => { position += 1; x += 1; },
            "s" => match &input[position + 1..position + 2] {
                "e" => { position += 2; y += 1; }
                "w" => { position += 2; x -= 1; y += 1; },
                _ => panic!("Not expected!"),
            },
            "w" => { position += 1; x -= 1 },
            value => panic!("Not expected {}", value),
        }
    }

    (x, y)
}

fn main() {
    let mut is_black: HashMap<(isize, isize), bool> = HashMap::new();

    let contents = fs::read_to_string("assets/day24.in").unwrap();
    for line in contents.split("\n") {
        let coordinate = coordinate(&line);

        let turn_black;
        if is_black.contains_key(&coordinate) {
            turn_black = !is_black.get(&coordinate).unwrap();
        } else {
            turn_black = true;
        }

        is_black.insert(coordinate, turn_black);
    }

    let mut black_face_up = 0;
    for (_, is_black) in is_black {
        if is_black {
            black_face_up += 1;
        }
    }

    println!("{} have face turned up", black_face_up);
}
