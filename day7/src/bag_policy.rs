use regex::Regex;
use std::collections::HashMap;

pub struct BagPolicy {
    bags_in_system: Vec<String>,

    // Maps bag to all bags that can contain it.
    containment_map: HashMap<String, Vec<String>>,

    // Maps a container bag to all things it contains.
    container_map: HashMap<String, Vec<(String, usize)>>,
}

impl BagPolicy {
    pub fn new() -> BagPolicy {
        BagPolicy { 
            containment_map: HashMap::new(),
            bags_in_system: vec![],
            container_map: HashMap::new(),
        }
    }

    pub fn from_string(content: &str) -> BagPolicy {
        let mut policy = BagPolicy::new();
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
                policy.learn(container, &result[2], result[1].parse::<usize>().unwrap());
            }
        }
        policy
    }

    pub fn learn(&mut self, container: &str, target: &str, amount: usize) {
        if !self.bags_in_system.contains(&target.to_string()) {
            self.bags_in_system.push(target.to_string());
        }

        if !self.bags_in_system.contains(&container.to_string()) {
            self.bags_in_system.push(container.to_string());
        }

        self.containment_map
            .entry(String::from(target))
            .or_insert(vec![]);

        self.containment_map
            .get_mut(&String::from(target))
            .unwrap()
            .push(container.to_string());

        self.container_map
            .entry(String::from(container))
            .or_insert(vec![]);

        self.container_map
            .get_mut(&String::from(container))
            .unwrap()
            .push((target.to_string(), amount));
    }

    pub fn get_bags(&self) -> &Vec<String> {
        &self.bags_in_system
    }

    pub fn is_contained_in(&self, target: &str, container: &str) -> bool {
        if target == container {
            return false;
        }

        let mut queue = vec![target];
        let mut visited: Vec<&str> = vec![];

        loop {
            let fits_in_target = match queue.pop() {
                None => break,
                Some(value) => value,
            };

            if visited.contains(&fits_in_target) {
                continue
            }

            visited.push(fits_in_target);

            if fits_in_target == container {
                return true;
            }

            let is_contained_in = match self.containment_map.get(fits_in_target) {
                None => continue,
                Some(value) => value,
            };

            for new_target in is_contained_in {
                queue.push(new_target);
            }
        }

        false
    }

    pub fn count_bags(&self, bag: &str) -> usize {
        match self.container_map.get(bag) {
            None => 0,
            Some(bags) => 
                bags
                    .iter()
                    .map(|(child_bag, count)| count + count * self.count_bags(&child_bag))
                    .sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_will_store() {
        let policy = BagPolicy::from_string(
"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
        );
        
        assert!(policy.is_contained_in("shiny gold", "bright white"));
        assert!(policy.is_contained_in("shiny gold", "muted yellow"));
        assert!(policy.is_contained_in("muted yellow", "dark orange"));
        assert!(policy.is_contained_in("bright white", "light red"));
        assert!(policy.is_contained_in("muted yellow", "light red"));

        assert!(policy.is_contained_in("shiny gold", "bright white"));
        assert!(policy.is_contained_in("shiny gold", "muted yellow"));
        assert!(policy.is_contained_in("shiny gold", "dark orange"));
        assert!(policy.is_contained_in("shiny gold", "light red"));
        
        assert_eq!(false, policy.is_contained_in("shiny gold", "shiny gold"));
        assert_eq!(false, policy.is_contained_in("shiny gold", "faded blue"));
        assert_eq!(false, policy.is_contained_in("shiny gold", "dark olive"));
        assert_eq!(false, policy.is_contained_in("shiny gold", "vibrant plum"));
        assert_eq!(false, policy.is_contained_in("shiny gold", "dotted black"));

        assert_eq!(0, policy.count_bags("faded blue"));
        assert_eq!(0, policy.count_bags("dotted black"));
        assert_eq!(11, policy.count_bags("vibrant plum"));
        assert_eq!(7, policy.count_bags("dark olive"));
    }

    #[test]
    fn it_will_detect_cycles() {
        let mut policy = BagPolicy::new();
        policy.learn("shiny gold", "bright white", 1);
        policy.learn("bright white", "shiny gold", 1);
        
        assert!(policy.is_contained_in("shiny gold", "bright white"));
        assert!(policy.is_contained_in("bright white", "shiny gold"));
        assert_eq!(false, policy.is_contained_in("shiny gold", "huh"));
        assert_eq!(false, policy.is_contained_in("bright white", "huh"));
        assert_eq!(false, policy.is_contained_in("huh", "shiny gold"));
        assert_eq!(false, policy.is_contained_in("huh", "bright white"));
    }
}