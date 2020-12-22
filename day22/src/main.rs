use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Player {
    cards: VecDeque<usize>,
}

impl Player {
    fn from(player: &Player, number_of_cards: usize) -> Player {
        let mut cards = player.cards.clone();
        loop {
            if cards.len() <= number_of_cards {
                break;
            }

            cards.pop_back();
        }

        Player { cards: cards }
    }

    fn draw(&mut self) -> usize {
        self.cards.pop_front().unwrap()
    }

    fn put_back(&mut self, card: usize) {
        self.cards.push_back(card);
    }

    fn lost(&self) -> bool {
        self.cards.len() == 0
    }

    fn count(&self) -> usize {
        self.cards.len()
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for (position, card) in self.cards.iter().rev().enumerate() {
            score += card * (position + 1);
        }
        score
    }
}

fn recursive_comat(mut player_a: Player, mut player_b: Player) -> (bool, usize, usize) {
    let mut seen: HashMap<(Vec<usize>, Vec<usize>), bool> = HashMap::new();

    loop {
        // Test if we seen this configuration before
        let key = (
            player_a.cards.clone().into_iter().collect::<Vec<usize>>(), 
            player_b.cards.clone().into_iter().collect::<Vec<usize>>()
        );

        if seen.contains_key(&key) {
            return (true, player_a.score(), player_b.score());
        }

        seen.insert(key, true);

        // If not, lets play!
        let draw_a = player_a.draw().clone();
        let draw_b = player_b.draw().clone();

        let mut player_a_won = false;

        if player_a.count() >= draw_a && player_b.count() >= draw_b {
            let (sub_player_a_won, _, _)  = recursive_comat(Player::from(&player_a, draw_a), Player::from(&player_b, draw_b));
            player_a_won = sub_player_a_won;
        } else {
            player_a_won = draw_a > draw_b;
        }

        if player_a_won { 
            player_a.put_back(draw_a);
            player_a.put_back(draw_b);
        } else {
            player_b.put_back(draw_b);
            player_b.put_back(draw_a);
        }

        if player_a.lost() || player_b.lost() {
            return (player_b.lost(), player_a.score(), player_b.score())
        }
    }
}

fn main() {
    // let mut player_a = Player { cards: VecDeque::from(vec![9, 2 ,6, 3, 1]) };
    // let mut player_b = Player { cards: VecDeque::from(vec![5, 8, 4, 7, 10]) };

    let mut player_a = Player { cards: VecDeque::from(vec![28, 13, 25, 16, 38, 3, 14, 6, 29, 2, 47, 20, 35, 43, 30, 39, 21, 42, 50, 48, 23, 11, 34, 24, 41]) };
    let mut player_b = Player { cards: VecDeque::from(vec![27, 37, 9, 10, 17, 31, 19, 33, 40, 12, 32, 1, 18, 36, 49, 46, 26, 4, 45, 8, 15, 5, 44, 22, 7]) };

    // let mut player_a = Player { cards: VecDeque::from(vec![43, 19]) };
    // let mut player_b = Player { cards: VecDeque::from(vec![2, 29, 14]) };

    let (player_a_won, score_player_a, score_player_b) = recursive_comat(player_a, player_b);

    println!("Player a winner? {}", player_a_won);
    println!("Score player a {}", score_player_a);
    println!("Score player b {}", score_player_b);
}
