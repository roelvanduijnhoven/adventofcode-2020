use std::collections::HashMap;

fn memory_game(input: &Vec<usize>, iterations: usize) -> usize {
    let mut memory: HashMap<usize, (usize, usize)> = HashMap::new();
    for (position, item) in input.iter().enumerate() {
        memory.insert(*item, (position, position));
    }

    let mut previous = input[input.len() - 1];
    for i in input.len()..iterations {
        let (first, last) = memory.get(&previous).unwrap();
        previous = last - first;

        memory.insert(previous, match memory.get(&previous) {
            None => (i, i),
            Some((_, last)) => (*last, i),
        });
    }

    previous
}

fn main() {
    let input = vec![0,6,1,7,2,19,20];

    println!("Solution to part 1 = {}", memory_game(&input, 2020));
    println!("Solution to part 2 = {}", memory_game(&input, 30000000));
}
