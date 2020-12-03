mod puzzle;

use puzzle::Puzzle;
use std::fs;

fn read_puzzle_from_file(input_path: String) -> Puzzle {
    let content = fs::read_to_string(input_path).expect("Something went wrong reading the file");
    return Puzzle::read_from_string(&content);
}

fn get_tree_encounters(puzzle: &Puzzle, x_step: usize, y_step: usize ) -> usize {
    let mut x = 0; 
    let mut y = 0;

    let mut trees_encountered = 0;

    loop {
        x += x_step;
        y += y_step;

        if y >= puzzle.height {
            break;
        }

        if puzzle.is_tree(x, y) {
            trees_encountered += 1;
        }
    }
        
    return trees_encountered;
}

fn main() {
    let puzzle = read_puzzle_from_file("assets/day3.input".to_string());

    let right_1_down_1 = get_tree_encounters(&puzzle, 1, 1);
    let right_3_down_1 = get_tree_encounters(&puzzle, 3, 1);
    let right_5_down_1 = get_tree_encounters(&puzzle, 5, 1);
    let right_7_down_1 = get_tree_encounters(&puzzle, 7, 1);
    let right_1_down_2 = get_tree_encounters(&puzzle, 1, 2);

    println!(
        "{} x {} x {} x {} x {} = {}",
        right_1_down_1, right_1_down_2, right_3_down_1, right_5_down_1, right_7_down_1,
        right_1_down_1 * right_1_down_2 * right_3_down_1 * right_5_down_1 * right_7_down_1
    );
}
