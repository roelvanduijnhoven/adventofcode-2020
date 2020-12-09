pub struct Encryption {
    look_back: usize,
    last_numbers: Vec<usize>,
}

#[derive(Debug)]
pub struct NotASumOfLastDigits {}

impl Encryption {
    pub fn new(preamble: &[usize]) -> Encryption {
        let mut preamble_vec = vec![0; preamble.len()];
        preamble_vec.clone_from_slice(preamble);

        Encryption {
            look_back: preamble.len(),
            last_numbers: preamble_vec,
        }
    }

    pub fn parse(&mut self, input: usize) -> Result<usize, NotASumOfLastDigits> {
        let is_found = self.is_sum_found_in_last_digits(input);
        
        // Note that we also need to to store unparsable results to the last digit stream in order to comply.
        self.last_numbers.push(input);

        return match is_found {
            true => Ok(input),
            false => Err(NotASumOfLastDigits {}),
        }
    }

    pub fn is_sum_found_in_last_digits(&self, total: usize) -> bool {
        let start: usize = self.last_numbers.len() - self.look_back;
        for i in &self.last_numbers[start..] {
            for j in &self.last_numbers[start..] {
                if i == j {
                    continue;
                }

                if i + j == total {
                    return true;
                }
            }
        }

        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_will_work() {
        let mut encryption = Encryption::new(&vec![35, 20, 15, 25, 47]);

        let input: Vec<usize> = vec![40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
        for item in input {
            let next = encryption.parse(item);
            if item == 127 {
                assert!(next.is_err());
            } else {
                assert_eq!(item, next.unwrap());
            }
        }
    }
}