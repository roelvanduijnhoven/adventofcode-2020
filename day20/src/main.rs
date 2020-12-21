use std::collections::HashMap;
use std::fs;

const PIXELS: usize = 10;

#[derive(Debug, Clone)]
struct SatelliteTile {
    id: usize,
    pixels: Vec<bool>,
}

impl SatelliteTile {
    fn from_string(input: &str) -> SatelliteTile {
        let lines: Vec<&str> = input.split("\n").collect();

        let id_line = lines[0];
        let id = id_line[5..id_line.len() - 1].parse::<usize>().unwrap();

        let mut pixels = vec![false; PIXELS * PIXELS];
        for (y, line) in lines.iter().skip(1).enumerate() {
            for (x, character) in line.chars().enumerate() {
                if character == '#' {
                    pixels[y * PIXELS + x] = true;
                }
            }
        }

        SatelliteTile { id, pixels }
    }

    fn cw_nord(&self) -> usize {
        self.get_ccw_border(0, 1)
    }

    fn cw_east(&self) -> usize {
        self.get_ccw_border(PIXELS - 1, PIXELS as isize)
    }

    fn cw_south(&self) -> usize {
        self.get_ccw_border(PIXELS * PIXELS - 1, -1)
    }

    fn cw_west(&self) -> usize {
        self.get_ccw_border(PIXELS * PIXELS - PIXELS, -1 * PIXELS as isize)
    }

    fn ccw_nord(&self) -> usize {
        self.get_ccw_border(PIXELS - 1, -1)
    }

    fn ccw_east(&self) -> usize {
        self.get_ccw_border(PIXELS * PIXELS - 1, -1 * PIXELS as isize)        
    }

    fn ccw_south(&self) -> usize {
        self.get_ccw_border(PIXELS * PIXELS - PIXELS, 1)
    }

    fn ccw_west(&self) -> usize {
        self.get_ccw_border(0, PIXELS as isize)
    }

    fn get_ccw_border(&self, start: usize, delta: isize) -> usize {
        let mut sum = 0;

        let mut position = start as isize;
        for iteration in 0..PIXELS {
            let value = self.pixels[position as usize];
            if value {
                let value = 1 << iteration;
                sum += value;
            }

            position += delta;
        }

        sum
    }
}

fn orientations_of(tile: &SatelliteTile) -> Vec<SatelliteTile> {
    let mut orientations: Vec<SatelliteTile> = vec![];

    orientations.push(tile.clone());

    // Rotations
    orientations.push(SatelliteTile { id: tile.id, pixels: rotate(&tile.pixels, PIXELS) });
    orientations.push(SatelliteTile { id: tile.id, pixels: rotate(&rotate(&tile.pixels, PIXELS), PIXELS) });
    orientations.push(SatelliteTile { id: tile.id, pixels: rotate(&rotate(&rotate(&tile.pixels, PIXELS), PIXELS), PIXELS) });

    // Flip it
    orientations.push(SatelliteTile { id: tile.id, pixels: flip(&tile.pixels, PIXELS) });

    // And now rotate again
    orientations.push(SatelliteTile { id: tile.id, pixels: rotate(&flip(&tile.pixels, PIXELS), PIXELS) });
    orientations.push(SatelliteTile { id: tile.id, pixels: rotate(&rotate(&flip(&tile.pixels, PIXELS), PIXELS), PIXELS) });
    orientations.push(SatelliteTile { id: tile.id, pixels: rotate(&rotate(&rotate(&flip(&tile.pixels, PIXELS), PIXELS), PIXELS), PIXELS) });    

    return orientations;
}

fn puzzle_total(orientations: &Vec<SatelliteTile>) -> Option<Vec<SatelliteTile>> {
    for orientation in orientations {
        match puzzle(orientations, orientation) {
            None => continue,
            value => return value,
        }
    }

    None
}

fn puzzle(orientations: &Vec<SatelliteTile>, start: &SatelliteTile) -> Option<Vec<SatelliteTile>> {
    let mut used = vec![start.id];
    let mut pieces = vec![start.clone()];

    // println!("===");
    // println!("Start searching with tile {}", start.id);

    let per_row = 12;
    for position in 1..(per_row * per_row) {
        let above = if position < per_row {
            None
        } else {
            Some(&pieces[position - per_row])
        };

        let previous = if (position % per_row) == 0 {
            None
        } else {
            Some(&pieces[position - 1])
        };
        
        // println!("Previous is {:?}, above is {:?}", previous, above);

        let mut winner = None;
        for orientation in orientations {
            if used.contains(&orientation.id) {
                continue;
            }

            let matches_previous = match previous {
                None => true,
                Some(piece) => piece.ccw_east() == orientation.cw_west(),
            };

            let matches_above = match above {
                None => true,
                Some(piece) => piece.ccw_south() == orientation.cw_nord(),
            };

            if matches_above && matches_previous {
                winner = Some(orientation);
            }
        }

        match winner {
            None => return None,
            Some(orientation) => {
                used.push(orientation.id);
                pieces.push(orientation.clone());
                // println!("Found matching piece {:?}!", orientation);
            }
        }
    }

    return Some(pieces);
}

fn rotate(matrix: &Vec<bool>, n: usize) -> Vec<bool> {
    let mut ret = vec![false; n * n];

    for i in 0..n {
        for j in 0..n {
            ret[j * n + i] = matrix[(n - i - 1) * n + j];
        }
    }

    return ret;
}

fn flip(matrix: &Vec<bool>, n: usize) -> Vec<bool> {
    let mut ret = vec![false; n * n];

    let half = (n as f32 / 2.0).floor() as usize;

    for y in 0..n {
        for x in 0..n {
            let new_y = n - 1 - y;
            ret[new_y * n + x] = matrix[y * n + x];
        }
    }

    ret
}

fn main() {
    // let matrix = vec![true, true, false, false, false, true, false, true, false];
    // println!("Rotated is {:?}", rotate(&matrix, 3));
    // println!("Rotated is {:?}", flip(&matrix, 3));
    // panic!("Done!");

    let contents = fs::read_to_string("assets/day20.in").unwrap();

    let tiles: Vec<SatelliteTile> = contents
        .split("\n\n")
        .map(|part| SatelliteTile::from_string(part))
        .collect();

    // Look at all possible ways we can position our tiles, so that we don't need to consider rotating them.
    let mut orientations: Vec<SatelliteTile> = vec![];
    for tile in &tiles {
        for orientation in orientations_of(&tile) {
            orientations.push(orientation.clone());
        }
    }

    let winner = puzzle_total(&orientations);
    println!("{:?}", winner);
}
