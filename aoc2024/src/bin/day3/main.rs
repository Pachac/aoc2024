use aoc2024::{check_if_test, input_path};

pub mod day3;

const DAY: u8 = 3;

fn main() {
    println!("Day 3: Mull It Over");
    let test = check_if_test();
    if test {
        println!("Using test input");
    }
    day3::run(input_path(DAY, test));
}