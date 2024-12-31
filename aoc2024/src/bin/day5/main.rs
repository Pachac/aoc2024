use aoc2024::{check_if_test, input_path};

pub mod day5;

const DAY: u8 = 5;

fn main() {
    println!("Day 5: Print Queue");
    let test = check_if_test();
    if test {
        println!("Using test input");
    }
    day5::run(input_path(DAY, test));
}
