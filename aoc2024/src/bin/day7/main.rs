use aoc2024::{check_if_test, input_path};

pub mod day7;

const DAY: u8 = 7;

fn main() {
    println!("Day 7: Bridge Repair");
    let test = check_if_test();
    if test {
        println!("Using test input");
    }
    day7::run(input_path(DAY, test));
}
