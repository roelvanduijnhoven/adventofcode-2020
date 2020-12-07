use std::collections::HashMap;

pub struct BagPolicy {
    bags_in_system: Vec<String>,
    containment_map: HashMap<String, Vec<String>>,
}

impl BagPolicy {
    pub fn new () -> BagPolicy {
        BagPolicy { containment_map: HashMap::new(), bags_in_system: vec![] }
    }

    pub fn learn(&mut self, target: &str, container: &str) {
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
    }

    pub fn get_bags(&self) -> &Vec<String> {
        &self.bags_in_system
    }

    pub fn can_contain(&self, target: &str, container: &str) -> bool {
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

            let can_contain = match self.containment_map.get(fits_in_target) {
                None => continue,
                Some(value) => value,
            };

            for new_target in can_contain {
                queue.push(new_target);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_will_store() {
        let mut policy = BagPolicy::new();
        policy.learn("shiny gold", "bright white");
        policy.learn("shiny gold", "muted yellow");
        policy.learn("muted yellow", "dark orange");
        policy.learn("bright white", "light red");
        policy.learn("muted yellow", "light red");

        assert!(policy.can_contain("shiny gold", "bright white"));
        assert!(policy.can_contain("shiny gold", "muted yellow"));
        assert!(policy.can_contain("muted yellow", "dark orange"));
        assert!(policy.can_contain("bright white", "light red"));
        assert!(policy.can_contain("muted yellow", "light red"));

        assert!(policy.can_contain("shiny gold", "bright white"));
        assert!(policy.can_contain("shiny gold", "muted yellow"));
        assert!(policy.can_contain("shiny gold", "dark orange"));
        assert!(policy.can_contain("shiny gold", "light red"));
        
        assert_eq!(false, policy.can_contain("shiny gold", "shiny gold"));
        assert_eq!(false, policy.can_contain("shiny gold", "faded blue"));
        assert_eq!(false, policy.can_contain("shiny gold", "dark olive"));
        assert_eq!(false, policy.can_contain("shiny gold", "vibrant plum"));
        assert_eq!(false, policy.can_contain("shiny gold", "dotted black"));
    }

    #[test]
    fn it_will_detect_cycles() {
        let mut policy = BagPolicy::new();
        policy.learn("shiny gold", "bright white");
        policy.learn("bright white", "shiny gold");
        
        assert!(policy.can_contain("shiny gold", "bright white"));
        assert!(policy.can_contain("bright white", "shiny gold"));
        assert_eq!(false, policy.can_contain("shiny gold", "huh"));
        assert_eq!(false, policy.can_contain("bright white", "huh"));
        assert_eq!(false, policy.can_contain("huh", "shiny gold"));
        assert_eq!(false, policy.can_contain("huh", "bright white"));
    }
}