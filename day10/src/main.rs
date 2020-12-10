fn get_difference(mut input: Vec<usize>) {
    input.sort();

    let target = input[input.len() - 1] + 3;

    let mut number_of_1_jumps = 0;
    let mut number_of_3_jumps = 1;

    let mut current = 0;
    for adapter in input {
        if current == target {
            break
        }

        let difference = adapter - current;
        if difference > 3 {
            continue;
        }

        if difference == 1 {
            number_of_1_jumps += 1;
        } else if difference == 3 {
            number_of_3_jumps += 1;
        }

        current += difference;
    }

    println!("1 jumps = {}, 3 jumps = {}", number_of_1_jumps, number_of_3_jumps);
}

fn distinct_ways(sorted_adapters: &Vec<usize>, target_jolt: usize) -> usize {
    // Dynamic programming table
    // table[p, s] describes how many combinations you have when starting with s jolt, and you can pick
    // from adapter[p..].
    let mut table: [[usize; 300]; 300] = [[0; 300]; 300];

    for position in (0..sorted_adapters.len()).rev() {
        let adapter = sorted_adapters[position];
        for jolt in 0..target_jolt + 1 {        
            let mut if_taken = 0;
            if jolt <= adapter && adapter <= jolt + 3 {
                if adapter == target_jolt {
                    if_taken = 1;
                } else {
                    if_taken = table[position + 1][adapter];
                }
            }

            let not_taken = table[position + 1][jolt];
            table[position][jolt] = not_taken + if_taken;
        }
    }

    table[0][0]
}

fn main() {
    // let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    // input.sort();

    // let mut input = vec![
    //     28,
    //     33,
    //     18,
    //     42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3
    // ];
    // input.sort();

    let mut input = vec![99, 151, 61, 134, 112, 70, 75, 41, 119, 137, 158, 50, 167, 60, 116, 117, 62, 82, 31, 3, 72, 88, 165, 34, 8, 14, 27, 108, 166, 71, 51, 42, 135, 122, 140, 109, 1, 101, 2, 77, 85, 76, 143, 100, 127, 7, 107, 13, 148, 118, 56, 159, 133, 21, 154, 152, 130, 78, 54, 104, 160, 153, 95, 49, 19, 69, 142, 63, 11, 12, 29, 98, 84, 28, 17, 146, 161, 115, 4, 94, 24, 126, 136, 91, 57, 30, 155, 79, 66, 141, 48, 125, 162, 37, 40, 147, 18, 20, 45, 55, 83];
    input.sort();

    println!("{}", distinct_ways(&input, input[input.len() - 1]));

    // get_difference(a.clone());
}
