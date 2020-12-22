use std::collections::VecDeque;

#[derive(Debug)]
struct Player {
    cards: VecDeque<usize>,
}

impl Player {
    fn draw(&mut self) -> usize {
        self.cards.pop_front().unwrap()
    }

    fn put_back(&mut self, mut cards: Vec<usize>) {
        cards.sort();
        cards.reverse();
        for card in cards {
            self.cards.push_back(card);
        }
    }

    fn lost(&self) -> bool {
        self.cards.len() == 0
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for (position, card) in self.cards.iter().rev().enumerate() {
            score += card * (position + 1);
        }
        score
    }
}

fn main() {
    let mut player_a = Player { cards: VecDeque::from(vec![9, 2 ,6, 3, 1]) };
    let mut player_b = Player { cards: VecDeque::from(vec![5, 8, 4, 7, 10]) };

    let mut player_a = Player { cards: VecDeque::from(vec![28, 13, 25, 16, 38, 3, 14, 6, 29, 2, 47, 20, 35, 43, 30, 39, 21, 42, 50, 48, 23, 11, 34, 24, 41]) };
    let mut player_b = Player { cards: VecDeque::from(vec![27, 37, 9, 10, 17, 31, 19, 33, 40, 12, 32, 1, 18, 36, 49, 46, 26, 4, 45, 8, 15, 5, 44, 22, 7]) };

    loop {
        let draw_a = player_a.draw().clone();
        let draw_b = player_b.draw().clone();

        if draw_a > draw_b { 
            player_a.put_back(vec![draw_a, draw_b]);
        } else {
            player_b.put_back(vec![draw_a, draw_b]);
        }

        if player_a.lost() || player_b.lost() {
            break;
        }
    }

    println!("Score player a {}", player_a.score());
    println!("Score player b {}", player_b.score());
}
