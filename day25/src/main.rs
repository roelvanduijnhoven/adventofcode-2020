fn calculate(subject: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = value * subject;
        value = value % 20201227;
    }    
    value
}

fn bruteforce_loop_size(subject: usize, result: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;
    loop {
        loop_size += 1;

        value = value * subject;
        value = value % 20201227;
        if value == result {
            return loop_size;
        }

        if loop_size % 10_000 == 0 {
            println!("Made some progress {}", loop_size);
        }
    }
}

fn main() {
    let part_a = bruteforce_loop_size(7, 3248366);
    let part_b = bruteforce_loop_size(7, 4738476);

    println!("{}, {}", calculate(3248366, part_b), calculate(4738476, part_a));



    // let mut loop_size = 0;
    // loop {
    //     let result = transform(7, loop_size);
    //     if result == 3248366 {
    //         panic!("Result with loop size of {}", loop_size);
    //     }

        // if loop_size % 10_000 == 0 {
        //     println!("Made some progress {}", loop_size);
        // }
        
    //     loop_size += 1;
    // }
}
