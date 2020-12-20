use std::collections::HashMap;
use std::fs;

const PIXELS: usize = 10;

#[derive(Debug)]
struct SatelliteTile {
    id: usize,
    pixels: [bool; PIXELS * PIXELS],
}

impl SatelliteTile {
    fn from_string(input: &str) -> SatelliteTile {
        let lines: Vec<&str> = input.split("\n").collect();

        let id_line = lines[0];
        let id = id_line[5..id_line.len() - 1].parse::<usize>().unwrap();

        let mut pixels = [false; PIXELS * PIXELS];
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

#[derive(Debug)]
struct TileOrientation {
    tile_id: usize,
    ccw_nord: usize,
    ccw_east: usize,
    ccw_south: usize,
    ccw_west: usize,

    cw_nord: usize,
    cw_east: usize,
    cw_south: usize,
    cw_west: usize,
}

impl TileOrientation {
    fn new(tile: &SatelliteTile, ccw_nord: usize, ccw_east: usize, ccw_south: usize, ccw_west: usize, cw_nord: usize, cw_east: usize, cw_south: usize, cw_west: usize ) -> TileOrientation {
        TileOrientation { tile_id: tile.id, ccw_nord, ccw_east, ccw_south, ccw_west, cw_nord, cw_east, cw_south, cw_west }
    }

    fn from(tile: &SatelliteTile) -> Vec<TileOrientation> {
        vec![
            // Original
            TileOrientation::new(
                tile,
                tile.ccw_nord(), tile.ccw_east(), tile.ccw_south(), tile.ccw_west(),
                tile.cw_nord(), tile.cw_east(), tile.cw_south(), tile.cw_west()
            ),

            // Rotation 
            TileOrientation::new(
                tile, 
                tile.ccw_west(), tile.ccw_nord(), tile.ccw_east(), tile.ccw_south(),
                tile.cw_west(), tile.cw_nord(), tile.cw_east(), tile.cw_south()
            ),
            TileOrientation::new(
                tile,
                tile.ccw_south(), tile.ccw_west(), tile.ccw_nord(), tile.ccw_east(),
                tile.cw_south(), tile.cw_west(), tile.cw_nord(), tile.cw_east()
            ),
            TileOrientation::new(
                tile,
                tile.ccw_east(), tile.ccw_south(), tile.ccw_west(), tile.ccw_nord(),
                tile.cw_east(), tile.cw_south(), tile.cw_west(), tile.cw_nord()
            ),

            // Flipped
            TileOrientation::new(
                tile,
                tile.cw_south(), tile.cw_east(), tile.cw_nord(), tile.cw_west(),
                tile.ccw_south(), tile.ccw_east(), tile.ccw_nord(), tile.ccw_west()
            ),

            // With its rotations
            TileOrientation::new(
                tile, 
                tile.cw_west(), tile.cw_south(), tile.cw_east(), tile.cw_nord(),
                tile.ccw_west(), tile.ccw_south(), tile.ccw_east(), tile.ccw_nord()
            ),
            TileOrientation::new(
                tile,
                tile.cw_nord(), tile.cw_west(), tile.cw_south(), tile.cw_east(),
                tile.ccw_nord(), tile.ccw_west(), tile.ccw_south(), tile.ccw_east()
            ),
            TileOrientation::new(
                tile,
                tile.cw_east(), tile.cw_nord(), tile.cw_west(), tile.cw_south(),
                tile.ccw_east(), tile.ccw_nord(), tile.ccw_west(), tile.ccw_south()
            ),

            //
        ]
    }
}

fn puzzle(orientations: &Vec<TileOrientation>, start: &TileOrientation) {
    let mut used = vec![start.tile_id];
    let mut pieces = vec![start];

    println!("===");
    println!("Start searching with tile {}", start.tile_id);

    let per_row = 12;
    for position in 1..(per_row * per_row) {
        let above = if position < per_row {
            None
        } else {
            Some(pieces[position - per_row])
        };

        let previous = if (position % per_row) == 0 {
            None
        } else {
            Some(pieces[position - 1])
        };
        
        println!("Previous is {:?}, above is {:?}", previous, above);

        let mut winner = None;
        for orientation in orientations {
            if used.contains(&orientation.tile_id) {
                continue;
            }

            let matches_previous = match previous {
                None => true,
                Some(piece) => piece.ccw_east == orientation.cw_west,
            };

            let matches_above = match above {
                None => true,
                Some(piece) => piece.ccw_south == orientation.cw_nord,
            };

            if matches_above && matches_previous {
                winner = Some(orientation);
            }
        }

        match winner {
            None => return,
            Some(orientation) => {
                used.push(orientation.tile_id);
                pieces.push(orientation);
                println!("Found matching piece {:?}!", orientation);
            }
        }
    }

    println!("{}", pieces[0].tile_id *  pieces[per_row - 1].tile_id *  pieces[per_row * per_row - per_row].tile_id *  pieces[per_row * per_row - 1].tile_id);
    panic!("sdf");
}

fn main() {
    let contents = fs::read_to_string("assets/day20.in").unwrap();

    let tiles: Vec<SatelliteTile> = contents
        .split("\n\n")
        .map(|part| SatelliteTile::from_string(part))
        .collect();

    // Look at all possible ways we can position our tiles, so that we don't need to consider rotating them.
    let mut orientations: Vec<TileOrientation> = vec![];
    for tile in &tiles {
        for orientation in TileOrientation::from(&tile) {
            orientations.push(orientation);
        }
    }

    // println!("{:#?}", tiles[8].cw_west());
    // for other in TileOrientation::from(&tiles[0]) {
    //     println!("{}", other.ccw_east);
    // }

    // println!("{:#?}", tiles[tiles.len() - 1]);
    // println!("Orientations {:#?}", &orientations[64..]);

    // let piece = &orientations[64];
    // println!("{:?}", piece);
    // for other in &orientations {
    //     if piece.tile_id != other.tile_id && piece.cw_west == other.cw_south {
    //         println!("{} fits {}", piece.tile_id, other.tile_id);
    //     }
    // }

    for orientation in &orientations {
        puzzle(&orientations, &orientation);
    }
}
