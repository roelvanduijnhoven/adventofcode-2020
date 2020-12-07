mod bag_policy;

use bag_policy::BagPolicy;
use std::fs;
use regex::Regex;

fn main() {
    let mut policy = BagPolicy::new();

    let content = fs::read_to_string("assets/day7.in").expect("Something went wrong reading the file");
    for line in content.split("\n") {
        let re = Regex::new(r"^(.+) bags contain (.+)\.$").unwrap();
        let result = re.captures(&line).unwrap();
        let container: &str = &result[1];

        if &result[2] == "no other bags" {
            continue;
        }

        let targets: Vec<&str> = result[2].split(", ").collect();
        for target in targets {
            let re = Regex::new(r"^([0-9]+) (.+) (bag|bags)$").unwrap();
            let result = re.captures(&target).unwrap();
            policy.learn(&result[2], container);
        }
    }

    let bags = policy.get_bags();
    let can_contain_shiny_gold: Vec<&String> = bags
        .iter()
        .filter(|bag| policy.can_contain("shiny gold", bag))
        .collect();
        
    println!("Shiny gold bag can be contained in {} unique bags directly", can_contain_shiny_gold.len());
}
