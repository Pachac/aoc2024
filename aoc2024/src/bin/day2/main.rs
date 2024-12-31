use aoc2024::{check_if_test, input_path};

pub mod day2;

const DAY: u8 = 2;

fn main() {
    println!("Day 2: Red-Nosed Reports");
    let test = check_if_test();
    if test {
        println!("Using test input");
    }
    day2::run(input_path(DAY, test));
}