#[derive(Debug)]
pub struct Password {
    fields: Vec<(String, String)>
}

impl Password {
    pub fn from_string(input: &str) -> Password {
        let mut fields: Vec<(String, String)> = vec![];
        for attribute in input.split_whitespace() {
            let matches: Vec<&str> = attribute.split(':').collect();
            fields.push((matches[0].to_string() , matches[1].to_string()))
        }
        Password {fields: fields }
    }

    pub fn get_by_name(&self, key: &str) -> Option<&str> {
        for (local_key, local_value) in &self.fields {
            if key == *local_key {
                return Some(local_value);
            }
        }
        None
    }

    pub fn get_attributes(&self) -> Vec<String> {
        self.fields.iter().map(|(attribute, _)| attribute.to_owned()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_will_load_from_string() {

        let password = Password::from_string("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm");

        assert_eq!(Some("gry"), password.get_by_name("ecl"));
        assert_eq!(Some("860033327"), password.get_by_name("pid"));
        assert_eq!(Some("2020"), password.get_by_name("eyr"));
        assert_eq!(Some("#fffffd"), password.get_by_name("hcl"));
        assert_eq!(Some("1937"), password.get_by_name("byr"));
        assert_eq!(Some("2017"), password.get_by_name("iyr"));
        assert_eq!(Some("147"), password.get_by_name("cid"));
        assert_eq!(Some("183cm"), password.get_by_name("hgt"));
        assert_eq!(None, password.get_by_name("noob"));
    }
}