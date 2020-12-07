mod bag_policy;

use bag_policy::BagPolicy;
use std::fs;

fn main() {
    let content = fs::read_to_string("assets/day7.in").expect("Something went wrong reading the file");
    let policy = BagPolicy::from_string(&content);

    {
        let bags = policy.get_bags();
        let can_contain_shiny_gold: Vec<&String> = bags
            .iter()
            .filter(|bag| policy.is_contained_in("shiny gold", bag))
            .collect();    
        
        println!("Shiny gold bag can be contained in {} unique bags directly", can_contain_shiny_gold.len());
    }

    {
        println!("In a Shiny gold bag fit {} bags.", policy.count_bags("shiny gold"));
    }
}
