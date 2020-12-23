#[derive(Debug)]
struct Game {
    max_label: usize,
    current_label: usize,
    ring: Vec<usize>,
}

impl Game {
    fn new(ring: &Vec<usize>) -> Game {
        Game { ring: ring.clone(), current_label: ring[0], max_label: ring.len() }
    }

    fn do_move(&mut self) {
        // Take away some cups
        let take_out_1 = self.ring.remove(((self.current_position() + 1) % self.ring.len()) as usize);
        let take_out_2 = self.ring.remove(((self.current_position() + 1) % self.ring.len()) as usize);
        let take_out_3 = self.ring.remove(((self.current_position() + 1) % self.ring.len()) as usize);

        let mut designated_label = None;
        let mut search_label = self.current_label;
        loop {
            search_label = search_label - 1;
            if search_label == 0 {
                search_label = self.max_label;
            }

            for (position, cup) in self.ring.iter().enumerate() {
                if search_label == *cup {
                    println!("Found lower cup of {} at position {}", cup, search_label);
                    designated_label = Some(cup);
                    break;
                }
            }

            if designated_label.is_some() {
                break;
            }
        }

        let designated_label = *designated_label.unwrap();

        self.ring.insert((self.get_position_of(designated_label) + 1) % self.ring.len(), take_out_3);
        self.ring.insert((self.get_position_of(designated_label) + 1) % self.ring.len(), take_out_2);
        self.ring.insert((self.get_position_of(designated_label) + 1) % self.ring.len(), take_out_1);

        self.current_label = self.ring[(self.current_position() + 1) % self.ring.len()];
    }

    fn get_position_of(&self, label: usize) -> usize {
        for (a_position, a_label) in self.ring.iter().enumerate() {
            if *a_label == label {
                return a_position;
            }
        }
        panic!("Should not occur!");
    }

    fn current_position(&self) -> usize {
        self.get_position_of(self.current_label)
    }
}

fn main() {
    let mut game = Game::new(&vec![1,9,8,7,5,3,4,6,2]);


    println!("{:?}", game);

    for _ in (0..100) {
        game.do_move();
        println!("{:?}", game);
    }
}