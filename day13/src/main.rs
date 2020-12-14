fn first_problem() {
    let arrival_time = 1001287;
    let bus_ids = [13, 37, 461, 17, 19, 29, 739, 41, 23];

    let mut first_bus: Option<(usize, usize)> = None;
    for bus_id in bus_ids.iter() {
        let wait_time = bus_id - arrival_time % *bus_id;
        first_bus = match first_bus {
            None => Some((*bus_id, wait_time)),
            Some((_, a_bus_wait_time)) if wait_time < a_bus_wait_time => Some((*bus_id, wait_time)),
            Some(value) => Some(value),
        }
    }

    let (first_bus, wait_time) = first_bus.unwrap();

    println!("Problem 1: first bus {} arrives in {}, multiplication results in {}", first_bus, wait_time, first_bus * wait_time);
}

fn brute_force_example(start: usize, step: usize) {
    let mut current = start;
    loop {
        if current % 7 == 0 && current % 13 == 12 && current % 59 == 55 && current % 31 == 25 && current % 19 == 12 {
            println!("Found solution at {}", current);
            break;
        }

        current += step;
    }
}

fn brute_force(start: usize, step: usize) {
    let mut steps = 0;
    let mut current = start;
    loop {
        if current % 13 == 0 &&
           current % 37 == 37 - 7 &&
           current % 461 == 461 - 13 &&
           current % 17 == 7 && 
           current % 19 == 6 &&
           current % 29 == 16 &&
           current % 739 == 739 - 44 &&
           current % 41 == 28 &&
           current % 23 == 2
        {
            println!("Found solution at {}", current);
            break;
        }

        if steps % 100000000 == 0 {
            println!("Currently at {}", current);
        }

        steps += 1;
        current += step;
    }
}

fn second_problem() {
    println!("{:?}", shared_between_primes(7, 0, 13, 13 - 1).unwrap());
    println!("{:?}", shared_between_primes(7, 0, 59, 59 - 4).unwrap());
    println!("{:?}", shared_between_primes(7, 0, 31, 31 - 6).unwrap());
    println!("{:?}", shared_between_primes(7, 0, 19, 19 - 7).unwrap());
    brute_force_example(350, 413);

    println!("{:?}", shared_between_primes(13, 0, 37, 37 - 7).unwrap());
    println!("{:?}", shared_between_primes(13, 0, 461, 461 - 13).unwrap());
    println!("{:?}", shared_between_primes(13, 0, 17, 7).unwrap());
    println!("{:?}", shared_between_primes(13, 0, 19, 6).unwrap());
    println!("{:?}", shared_between_primes(13, 0, 29, 16).unwrap());
    println!("{:?}", shared_between_primes(13, 0, 739, 739 - 44).unwrap());
    println!("{:?}", shared_between_primes(13, 0, 41, 28).unwrap());
    println!("{:?}", shared_between_primes(13, 0, 23, 2).unwrap());
    brute_force(5980, 5993);


    // println!("We are going to do steps of {:?}", step_size);

    // let ab = shared_between(77, 0, 350, 0);
    // let cd = shared_between(56, 0, 126, 0);

    // let abcd = shared_between(3850, 0, 504, 0);

    // shared_between(3850, 0, 126, 0);
}

fn shared_between_primes(bus_a: usize, bus_time_a: usize, bus_b: usize, bus_time_b: usize) -> Option<(usize, usize)> {
    let mut matches: Vec<usize> = vec![];
    for t in 0..(bus_a * bus_b) * 2 {
        if t % bus_a == bus_time_a && t % bus_b == bus_time_b {
            matches.push(t);
        }
    }

    if matches.len() != 2 {
        panic!("Noooooooooooooo!");
    }

    Some((matches[0], matches[1] - matches[0]))
}

fn main() {
    // first_problem();
    // second_problem();

    let current: u128 = 83278918745344;
    println!("{}", current % 13 == 0 &&
    current % 37 == 37 - 7 &&
    current % 461 == 461 - 13 &&
    current % 17 == 7 && 
    current % 19 == 6 &&
    current % 29 == 16 &&
    current % 739 == 739 - 44 &&
    current % 41 == 28 &&
    current % 23 == 2);
}  