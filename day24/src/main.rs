use std::collections::HashMap;
use std::collections::HashSet;
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

fn get_neighbors(coordinate: (isize, isize)) -> Vec<(isize, isize)> {
    let x = coordinate.0;
    let y = coordinate.1;

    vec![
        (x + 1, y - 1),
        (x, y - 1),
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x - 1, y + 1),
    ]
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

    // Simulate a day
    for _ in (0..100) {
        // Get all possible candidates
        let mut candidates: Vec<(isize, isize)> = vec![];
        for (position, _) in &is_black {
            candidates.push(*position);
            for neighbor in get_neighbors(*position) {
                candidates.push(neighbor);
            }
        }

        // Deduplicate
        let set: HashSet<_> = candidates.drain(..).collect(); 
        candidates.extend(set.into_iter());

        // What to flip?
        let mut flip: Vec<(isize, isize)> = vec![];
        for position in candidates {
            let black = match is_black.get(&position) {
                Some(value) => *value,
                None => false,
            };

            let black_neighbor_count = get_neighbors(position)
                .iter()
                .map(|neighbor| match is_black.get(neighbor) {
                    Some(value) => *value,
                    None => false,
                })
                .filter(|black| *black)
                .count();

            if black && (black_neighbor_count == 0 || black_neighbor_count > 2) {
                flip.push(position);
            } 

            if !black && black_neighbor_count == 2 {
                flip.push(position);
            }
        }

        // Now flip
        for position in flip {
            let turn_black;
            if is_black.contains_key(&position) {
                turn_black = !is_black.get(&position).unwrap();
            } else {
                turn_black = true;
            }

            is_black.insert(position, turn_black);
        }

        // Report on how many blacks there are
        let mut black_face_up = 0;
        for (_, is_black) in &is_black {
            if *is_black {
                black_face_up += 1;
            }
        }

        println!("{} have face turned up", black_face_up);
    }
}
