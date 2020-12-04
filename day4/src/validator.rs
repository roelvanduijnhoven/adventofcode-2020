use crate::password::Password;

use regex::Regex;

fn string_in_between(input: &str, lower: usize, upper: usize) -> bool {
    return match input.parse::<usize>() {
        Ok(value) => value >= lower && value <= upper,
        _ => false
    }
}

pub fn is_valid_pasword(password: &Password) -> bool {
    let mandatory_attributes = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    
    for mandatory_attribute in mandatory_attributes {
        let attribute_result = password.get_by_name(mandatory_attribute);
        if attribute_result == None {
            return false;
        }

        let attribute_value = attribute_result.unwrap();
        
        if mandatory_attribute == "byr" && !string_in_between(attribute_value, 1920, 2002) {
            return false;
        } 

        if mandatory_attribute == "iyr" && !string_in_between(attribute_value, 2010, 2020) {
            return false;
        }

        if mandatory_attribute == "eyr" && !string_in_between(attribute_value, 2020, 2030) {
            return false;
        }

        if mandatory_attribute == "hgt" {
            let re = Regex::new(r"^([1-9][0-9]*)(cm|in)$").unwrap();
            let result = re.captures(&attribute_value);
            if result.is_none() {
                return false;
            }

            let result = result.unwrap();
            let height = result[1].parse::<usize>().unwrap();
            let height_type = result[2].parse::<String>().unwrap();

            if height_type == "cm" && (height < 150 || height > 193) {
                return false;
            }
            if height_type == "in" && (height < 59 || height > 76) {
                return false;
            }
        }

        if mandatory_attribute == "hcl" {
            let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            if !re.is_match(attribute_value) {
                return false;
            }
        }

        if mandatory_attribute == "ecl" {
            let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            if !re.is_match(attribute_value) {
                return false;
            }
        }        
    
        if mandatory_attribute == "pid" {
            let re = Regex::new(r"^[0-9]{9}$").unwrap();
            if !re.is_match(attribute_value) {
                return false;
            }
        }           
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_debug() {
        assert_eq!(true, is_valid_pasword(&Password::from_string("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1920 hcl:#623a2f")));
    }

    #[test]
    fn it_will_validate_passwords() {
        assert_eq!(true, is_valid_pasword(&Password::from_string("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f")));
        assert_eq!(true, is_valid_pasword(&Password::from_string("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm")));
        assert_eq!(true, is_valid_pasword(&Password::from_string("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022")));
        assert_eq!(true, is_valid_pasword(&Password::from_string("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719")));

        assert_eq!(false, is_valid_pasword(&Password::from_string("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926")));
        assert_eq!(false, is_valid_pasword(&Password::from_string("iyr:2019hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946")));
        assert_eq!(false, is_valid_pasword(&Password::from_string("hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277f")));
        assert_eq!(false, is_valid_pasword(&Password::from_string("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007")));
    }
}