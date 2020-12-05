mod seat;

use std::fs;
use seat::Seat;

fn main() {
    let content = fs::read_to_string("assets/day5.in").expect("Something went wrong reading the file");
    
    let seats: Vec<Seat> = content
        .split("\n")
        .map(|line| Seat::from_boarding_pass(line).expect("Could not decode boarding pass"))
        .collect();

    let mut ordered_seat_ids: Vec<usize> = seats
        .iter()
        .map(|seat| seat.get_seat_id())
        .collect();
    ordered_seat_ids.sort();

    let maximum_seat_id = *ordered_seat_ids.last().unwrap();
    println!("Maximum seat id is {}", maximum_seat_id);

    for i in 1..ordered_seat_ids.len() - 1 {
        let current = ordered_seat_ids.get(i).unwrap();
        let next = ordered_seat_ids.get(i + 1).unwrap();
        if next - current > 1 {
            println!("Found free spot(s) between {} and {}", current, next);
        }
    }
}