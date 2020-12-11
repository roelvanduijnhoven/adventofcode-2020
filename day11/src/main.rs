use std::fs;
use std::fmt;

#[derive(Debug, Clone, Copy)]
enum AreaState {
    Floor,
    OccupiedSeat,
    FreeSeat
}

#[derive(Debug)]
struct Area {
    width: usize,
    height: usize,
    state: Vec<AreaState>,
}

#[derive(Debug)]
struct ParseError;

impl Area {
    fn from_string(input: &str) -> Result<Area, ParseError> {
        let mut width = 0;
        let mut height = 0;
        let mut state: Vec<AreaState> = vec![];

        for line in input.split("\n") {
            let mut width_of_line = 0;
            for character in line.chars() {
                state.push(match character {
                    '.' => AreaState::Floor,
                    'L' => AreaState::FreeSeat,
                    '#' => AreaState::OccupiedSeat,
                    _ => return Err(ParseError{}),
                });
                width_of_line += 1;
            }

            width = width_of_line;
            height += 1;
        }

        Ok(Area { width, height, state})
    }

    fn simulate(&self) -> Option<Area> {
        let mut changed = false;
        let mut copied_state: Vec<AreaState> = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let occupied_neighbors = self.number_of_occupied_neighbors(x, y);
                copied_state.push(match self.state[y * self.width + x] {
                    AreaState::FreeSeat if occupied_neighbors == 0 => {
                        changed = true;
                        AreaState::OccupiedSeat
                    },
                    AreaState::OccupiedSeat if occupied_neighbors >= 5 => {
                        changed = true;
                        AreaState::FreeSeat
                    },
                    value => value
                });
            }
        }

        match changed {
            true => Some(Area { width: self.width, height: self.height, state: copied_state }),
            false => None,
        }
    }

    fn number_of_occupied(&self) -> usize {
        let mut occupied = 0;
        for i in &self.state {
            if let AreaState::OccupiedSeat = *i {
                occupied += 1;
            }
        }
        occupied
    }

    fn number_of_occupied_neighbors(&self, x: usize, y: usize) -> usize {
        let directions = [
            (-1, -1), ( 0, -1), (1, -1),
            (-1,  0),           (1,  0),
            (-1,  1), ( 0,  1), (1,  1)
        ];

        let mut occupied = 0;
        for (x_step, y_step) in directions.iter() {
            let mut current_x = x as isize;
            let mut current_y = y as isize;
            loop {
                current_x += x_step;
                current_y += y_step;

                if current_x < 0 || current_x >= self.width as isize {
                    break
                }
                if current_y < 0 || current_y >= self.height as isize {
                    break
                }
                
                match self.state[current_y as usize * self.width + current_x as usize] {
                    AreaState::FreeSeat => break,
                    AreaState::Floor => continue,
                    AreaState::OccupiedSeat => {
                        occupied += 1;
                        break;
                    }
                }
            }
        }
        
        occupied
    }

    fn number_of_occupied_direct_neighbors(&self, x: usize, y: usize) -> usize {
        let x = x as isize;
        let y = y as isize;
        
        let mut occupied = 0;
        for i in x-1..x+2 {
            for j in y-1..y+2 {
                if i == x && j == y {
                    continue
                }
                if i < 0 || i >= self.width as isize {
                    continue
                }
                if j < 0 || j >= self.height as isize {
                    continue
                }

                let state = self.state[j as usize * self.width + i as usize];
                if let AreaState::OccupiedSeat = state {
                    occupied += 1
                }
            }
        }

        occupied
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", match self.state[y * self.width + x] {
                    AreaState::Floor => '.',
                    AreaState::FreeSeat => 'L',
                    AreaState::OccupiedSeat => '#',
                });
            }
            write!(f, "\n");
        }

        write!(f, "")
    }
}

// impl Clone for Area {
//     fn clone(&self) -> Area {
//         let mut copied_state: Vec<AreaState> = vec![];

//         for i in &self.state {
//             copied_state.push(*i);
//         }

//         Area { width: self.width, height: self.height, state: copied_state }
//     }
// }

fn main() {
    let mut area = Area::from_string(
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
    ).unwrap();

    let content = fs::read_to_string("assets/day11.in").expect("Something went wrong reading the file");
    let mut area = Area::from_string(&content).unwrap();

    loop {
        println!("{}", area);
        match area.simulate() {
            None => break,
            Some(value) => area = value
        }
    }

    println!("Terminated with {} seats", area.number_of_occupied());
}
