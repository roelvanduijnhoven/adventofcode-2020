#[derive(Debug)]
pub struct Puzzle {
    internal_width: usize,
    pub height: usize,
    trees: Vec<bool>,
}

impl Puzzle {
    pub fn is_tree(&self, x: usize, y: usize) -> bool {
        let wrapped_x = x % self.internal_width;
        let position = y * self.internal_width + wrapped_x;
        return self.trees[position];
    }

    pub fn read_from_string(input: &str) -> Puzzle {
        let mut internal_width = 0;
        let mut height = 0;
        let mut trees = vec![];
        for line in input.split("\n") {
            internal_width = 0;
            for character in line.chars() {
                trees.push(character == '#');
                internal_width += 1;
            }
            height += 1;
        }
        Puzzle { internal_width: internal_width, height: height, trees: trees }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_will_detect_normal_positions() {
        let puzzle = Puzzle::read_from_string("..#..\n####.");
        assert_eq!(false, puzzle.is_tree(0, 0));
        assert_eq!(false, puzzle.is_tree(1, 0));
        assert_eq!(true, puzzle.is_tree(2, 0));
        assert_eq!(false, puzzle.is_tree(3, 0));
        assert_eq!(false, puzzle.is_tree(4, 0));
        assert_eq!(true, puzzle.is_tree(0, 1));
        assert_eq!(true, puzzle.is_tree(1, 1));
        assert_eq!(true, puzzle.is_tree(2, 1));
        assert_eq!(true, puzzle.is_tree(3, 1));
        assert_eq!(false, puzzle.is_tree(4, 1));
    }

    #[test]
    fn it_will_wrap_on_columns() {
        let puzzle = Puzzle::read_from_string("..#..\n####.");
        assert_eq!(false, puzzle.is_tree(5, 0));
        assert_eq!(false, puzzle.is_tree(11, 0));
        assert_eq!(true, puzzle.is_tree(7, 0));
        assert_eq!(false, puzzle.is_tree(18, 0));
        assert_eq!(false, puzzle.is_tree(9, 0));
        assert_eq!(true, puzzle.is_tree(50, 1));
        assert_eq!(true, puzzle.is_tree(51, 1));
        assert_eq!(true, puzzle.is_tree(22, 1));
        assert_eq!(true, puzzle.is_tree(23, 1));
        assert_eq!(false, puzzle.is_tree(9, 1));       
    }    
}