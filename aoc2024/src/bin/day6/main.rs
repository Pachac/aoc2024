use aoc2024::{check_if_test, input_path};

pub mod day6;

const DAY: u8 = 6;

fn main() {
    println!("Day 6: Guard Gallivant");
    let test = check_if_test();
    if test {
        println!("Using test input");
    }
    day6::run(input_path(DAY, test));
}
