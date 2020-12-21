use std::fs;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut output = vec![];

    for line in input.split("\n") {
        let parts = line.split(" (contains ").collect::<Vec<&str>>();
        
        let ingredients = parts[0].split(" ").collect::<Vec<&str>>();
        let allergens = parts[1][0..parts[1].len() - 1].split(", ").collect::<Vec<&str>>();

        output.push((ingredients, allergens));
    }

    output
}

fn find_singular(one_of: &HashMap<&str, Vec<&str>>, remaining: &Vec<&str>) -> Option<String> {
    for (_, candidate_ingredients) in one_of {
        
        let temp = candidate_ingredients
            .iter()
            .filter(|ingredient| remaining.contains(ingredient))
            .map(|ingredient| *ingredient)
            .collect::<Vec<&str>>();

        if temp.len() == 1 {
            return Some(String::from(temp[0]));
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("assets/day21.in").unwrap();
    let contents = parse(&input);

    let mut one_of: HashMap<&str, Vec<&str>> = HashMap::new();

    let mut all_ingredients = vec![];
    for (ingredient, _) in &contents {
        for ingredient in ingredient {
            if !all_ingredients.contains(ingredient) {
                all_ingredients.push(ingredient);
            }
        }
    }

    for (ingredients, allergens) in &contents {
        for allergen in allergens {
            // Load with initial values
            if !one_of.contains_key(allergen) {
                one_of.insert(allergen, ingredients.clone());
            }

            let candidates = one_of.get(allergen).unwrap();

            let filtered_candidates = candidates
                .iter()
                .filter(|ingredient| ingredients.contains(ingredient))
                .map(|ingredient| *ingredient)
                .collect::<Vec<&str>>();

            one_of.insert(&allergen, filtered_candidates.clone());
        }
    }

    let mut remaining: Vec<&str> = all_ingredients.clone();

    loop {
        let result = find_singular(&one_of, &remaining);
        match result {
            Some(singular_ingredient) => {      
                remaining.retain(|ingredient| ingredient != &singular_ingredient);
            },
            None => break
        }
    }
    
    println!("{:?}", remaining);

    let mut count = 0;
    for (ingredients, _) in &contents {
        for ingredient in ingredients {
            if remaining.contains(ingredient) {
                count += 1;
            }
        }
    }

    println!("Found {}", count);
}
