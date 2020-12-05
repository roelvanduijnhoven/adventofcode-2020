#[derive(Debug)]
pub struct Seat {
    pub row: usize,
    pub column: usize
}

#[derive(Debug)]
pub struct InvalidBoardingPassError;


impl Seat {
    pub fn from_boarding_pass(boarding_pass: &str) -> Result<Seat, InvalidBoardingPassError> {
        if boarding_pass.len() != 10 {
            return Err(InvalidBoardingPassError);
        }

        let chars: Vec<char> = boarding_pass.chars().collect();

        let mut low_row = 0;
        for i in 0..7 {
            let step_size = (2 as usize).pow((7 - 1 - i) as u32);
            match chars.get(i).unwrap() {
                'B' => low_row += step_size,
                'F' => (),
                _ => return Err(InvalidBoardingPassError)
            };
        }

        let mut low_column = 0;
        for i in 0..3 {
            let step_size = (2 as usize).pow((3 - 1 - i) as u32);
            match chars.get(7 + i).unwrap() {
                'R' => low_column += step_size,
                'L' => (),
                _ => return Err(InvalidBoardingPassError)
            };
        }

        Ok(Seat { row: low_row, column: low_column })
    }

    pub fn get_seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn assert_position(row: usize, column: usize, boarding_pass: &str) {
        let seat = Seat::from_boarding_pass(boarding_pass).unwrap();
        assert_eq!(row, seat.row);
        assert_eq!(column, seat.column);
    }

    #[test]
    fn it_will_decode_boarding_passes() {
        assert_position(70, 7, "BFFFBBFRRR");
        assert_position(14, 7, "FFFBBBFRRR");
        assert_position(102, 4, "BBFFBBFRLL");
    }
}