use std::fs;
use std::collections::HashMap;


struct BoxGrid {
    active: HashMap<(isize, isize, isize, isize), bool>,
}

impl BoxGrid {
    fn new() -> BoxGrid {
        BoxGrid {
            active: HashMap::new(),
        }
    }

    fn activate(&mut self, point: &(isize, isize, isize, isize)) {
        self.active.insert(*point, true);
    }

    fn deactivate(&mut self, point: &(isize, isize, isize, isize)) {
        self.active.remove(point);
    }

    fn count_active(&self) -> usize {
        let mut count = 0;
        for (_, value) in &self.active {
            if *value {
                count += 1;
            }
        }
        count
    }

    fn is_active(&self, point: &(isize, isize, isize, isize)) -> bool {
        match self.active.get(&point) {
            Some(value) => *value,
            None => false
        }
    }
    
    fn get_neighbors(&self, point: &(isize, isize, isize, isize)) -> Vec<(isize, isize, isize, isize)> {
        let (x, y, z, w) = *point;

        let mut neighbors = vec![];
        for an_x in x-1..x+2 {
            for an_y in y-1..y+2 {
                for an_z in z-1..z+2 {
                    for an_w in w-1..w+2 {
                        if x != an_x || y != an_y || z != an_z || w != an_w {
                            neighbors.push((an_x, an_y, an_z, an_w));
                        }
                    }
                }
            }
        }

        neighbors
    }
}

fn main() {
    let input = fs::read_to_string("assets/day17.in").unwrap();
    // let input = fs::read_to_string("assets/example.in").unwrap();

    let mut cube = BoxGrid::new();

    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == '#' {
                cube.activate(&(x as isize, y as isize, 0, 0));
            }
        }
    }
    
    for i in 0..6 {
        // println!("Iteration {}", i);
        
        // Get all unique points that could possible turn.
        // Note: we use a hashmap to avoid duplicates 
        let mut possible_candidates: HashMap<(isize, isize, isize, isize), bool> = HashMap::new(); 
        for (point, state) in &cube.active {
            if !state {
                continue;
            }

            possible_candidates.insert(*point, true);
            for possible_candidate in cube.get_neighbors(point) {
                possible_candidates.insert(possible_candidate, true);
            }
        }

        // Compute what cubes need to flip.
        let mut activate: Vec<(isize, isize, isize, isize)> = vec![];
        let mut deactivate: Vec<(isize, isize, isize, isize)> = vec![];
        for (point, _) in possible_candidates {
            let active_neighbors = cube
                .get_neighbors(&point)
                .iter()
                .filter(|a_point| cube.is_active(a_point))
                .count();

            let active = cube.is_active(&point);
            if active && (active_neighbors < 2 || active_neighbors > 3) {
                deactivate.push(point);
            } else if !active && active_neighbors == 3 {
                activate.push(point);
            }
        }

        // Apply new states
        for point in activate {
            cube.activate(&point);
        }
        for point in deactivate {
            cube.deactivate(&point);
        }
    }

    println!("Active cubes: {}", cube.count_active());
}
