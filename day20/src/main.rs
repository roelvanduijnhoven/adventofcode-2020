use std::fs;
use regex::Regex;

const PIXELS: usize = 10;
const PER_ROW: usize = 12;

#[derive(Debug, Clone)]
struct SatelliteTile {
    id: usize,
    pixels: Vec<bool>,
    orientation: String,
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

        SatelliteTile { id, pixels, orientation: String::from("original") }
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
    orientations.push(SatelliteTile { orientation: String::from("90CW"), id: tile.id, pixels: rotate(&tile.pixels, PIXELS) });
    orientations.push(SatelliteTile { orientation: String::from("180CW"), id: tile.id, pixels: rotate(&rotate(&tile.pixels, PIXELS), PIXELS) });
    orientations.push(SatelliteTile { orientation: String::from("270CW"), id: tile.id, pixels: rotate(&rotate(&rotate(&tile.pixels, PIXELS), PIXELS), PIXELS) });

    // Flip it
    orientations.push(SatelliteTile { orientation: String::from("flipped"), id: tile.id, pixels: flip(&tile.pixels, PIXELS) });

    // And now rotate again
    orientations.push(SatelliteTile { orientation: String::from("flipped + 90CW"), id: tile.id, pixels: rotate(&flip(&tile.pixels, PIXELS), PIXELS) });
    orientations.push(SatelliteTile { orientation: String::from("flipped + 180CW"), id: tile.id, pixels: rotate(&rotate(&flip(&tile.pixels, PIXELS), PIXELS), PIXELS) });
    orientations.push(SatelliteTile { orientation: String::from("flipped + 270CW"), id: tile.id, pixels: rotate(&rotate(&rotate(&flip(&tile.pixels, PIXELS), PIXELS), PIXELS), PIXELS) });    

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

    let per_row = PER_ROW;
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

    for y in 0..n {
        for x in 0..n {
            let new_y = n - 1 - y;
            ret[new_y * n + x] = matrix[y * n + x];
        }
    }

    ret
}

fn find_monster(image: &Vec<bool>, dimension: usize) {
    println!("Start!");

    let mut output = String::new();
    for y in 0..dimension {
        for x in 0..dimension {
            let c = if image[y * dimension + x] { '#' } else { '.' };
            output.push(c);
        }
        output.push_str("\n");
    }

    let first_line  = Regex::new(r"..................#.").unwrap();
    let second_line = Regex::new(r"#....##....##....###").unwrap();
    let third_line  = Regex::new(r".#..#..#..#..#..#...").unwrap();
    for second_line_match in second_line.find_iter(&output) {
        if second_line_match.start() < dimension {
            continue;
        }

        match third_line.find_at(&output, second_line_match.start() + dimension) {
            Some(third_line_match) => {
                match first_line.find_at(&output, (second_line_match.start() - dimension - 1) as usize) {
                    Some(first_line_match) => {
                        if first_line_match.start() != second_line_match.start() - dimension - 1 {
                            continue;
                        }

                        if third_line_match.start() != second_line_match.start() + dimension + 1 {
                            continue;
                        }
                        
                        println!("We found a sea monster!");
                    },
                    None => (),
                };
            },
            None => (),
        };
    }
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

    let winner = puzzle_total(&orientations).unwrap();

    // Solution to part 1
    let f1 = winner[0].id;
    let f2 = winner[PER_ROW - 1].id;
    let f3 = winner[PER_ROW * PER_ROW - 1].id;
    let f4 = winner[PER_ROW * PER_ROW - PER_ROW].id;

    // Now construct the image
    let mut image = vec![];
    for tile_y in 0..PER_ROW {
        for y in 1..PIXELS-1 {
            for tile_index in tile_y * PER_ROW..tile_y * PER_ROW + PER_ROW {
                for x in 1..PIXELS-1 {
                    image.push(winner[tile_index].pixels[y * PIXELS + x])
                }
            }
        }              
    }

    let mut count = 0;
    for i in &image {
        if *i {
            count += 1;
        }
    }

    println!("There are {} # tiles.", count);

    // Find sea monsters in every rotation
    let dimesion: usize = 12 * 8;
    find_monster(&image, dimesion);
    find_monster(&rotate(&image.clone(), dimesion), dimesion);
    find_monster(&rotate(&rotate(&image.clone(), dimesion), dimesion), dimesion);
    find_monster(&rotate(&rotate(&rotate(&image.clone(), dimesion), dimesion), dimesion), dimesion);
    find_monster(&flip(&image.clone(), dimesion), dimesion);
    find_monster(&rotate(&flip(&image.clone(), dimesion), dimesion), dimesion);
    find_monster(&rotate(&rotate(&flip(&image.clone(), dimesion), dimesion), dimesion), dimesion);
    find_monster(&rotate(&rotate(&rotate(&flip(&image.clone(), dimesion), dimesion), dimesion), dimesion), dimesion);
}
