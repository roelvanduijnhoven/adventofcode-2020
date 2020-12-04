mod password;
mod validator;

use password::Password;
use validator::is_valid_pasword;
use std::fs;

fn main() {
    let content = fs::read_to_string("assets/day4.in").expect("Something went wrong reading the file");
    let passwords: Vec<Password> = content
        .split("\n\n")
        .map(|input| Password::from_string(input))
        .collect();

    let mut valid_passwords = 0;
    for password in passwords {
        if is_valid_pasword(&password) {
            valid_passwords += 1;
        }
    }

    println!("{} valid passwords", valid_passwords);
}
