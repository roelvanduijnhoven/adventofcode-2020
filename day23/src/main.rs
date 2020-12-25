#[derive(Debug)]
struct Game {
    max_label: usize,
    current_label: usize,
    ring: Vec<usize>,
}

impl Game {
    fn new(ring: &Vec<usize>) -> Game {
        let mut complete_ring = ring.clone();
        for i in ring.len()..1_000_000 {
            complete_ring.push(i + 1);
        }

        let max_label = complete_ring.len();
        let current_label = complete_ring[0];

        Game { ring: complete_ring, current_label, max_label }
    }

    fn do_move(&mut self) {
        let current_position = self.current_position();
        
        let take_out_1;
        let take_out_2;
        let take_out_3;
        if current_position == self.ring.len() - 1 {
            take_out_1 = self.ring.remove(0);
            take_out_2 = self.ring.remove(0);
            take_out_3 = self.ring.remove(0);
        } else if current_position == self.ring.len() - 2 {
            take_out_1 = self.ring.remove(current_position + 1);
            take_out_2 = self.ring.remove(0);
            take_out_3 = self.ring.remove(0);
        } else if current_position == self.ring.len() - 3 {
            take_out_1 = self.ring.remove(current_position + 1);
            take_out_2 = self.ring.remove(current_position + 1);
            take_out_3 = self.ring.remove(0);
        } else {
            let mut remove = self.ring.splice((current_position + 1)..(current_position + 4), vec![]);
            take_out_1 = remove.next().unwrap();
            take_out_2 = remove.next().unwrap();
            take_out_3 = remove.next().unwrap();
        }

        let mut designated_position = None;
        let mut search_label = self.current_label;
        loop {
            search_label = search_label - 1;
            if search_label == 0 {
                search_label = self.max_label;
            }

            for (position, cup) in self.ring.iter().enumerate() {
                if search_label == *cup {
                    designated_position = Some(position);
                    break;
                }
            }

            if designated_position.is_some() {
                break;
            }
        }

        let designated_position = designated_position.unwrap();

        self.ring.splice((designated_position + 1)..(designated_position + 1), vec![take_out_1, take_out_2, take_out_3]);

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
    // let mut game = Game::new(&vec![3,8,9,1,2,5,4,6,7]);
    let mut game = Game::new(&vec![1,9,8,7,5,3,4,6,2]);
    
    for i in (0..10_000_000) {
        if i % 10_000 == 0 {
            println!("{}%", (i as f32) / 10_000_000.0 * 100.0);
        }

        game.do_move();

        // println!("{:?}", game);
    }

    let position_of_1 = game.get_position_of(1);
    let a = game.ring[(position_of_1 + 1) % game.ring.len()];
    let b = game.ring[(position_of_1 + 2) % game.ring.len()];
    println!("Number 1 = {}", a);
    println!("Number 2 = {}", b);
    println!("Total is {}", a * b);

    // 199350161100 is too low
}