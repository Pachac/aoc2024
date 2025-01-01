use aoc2024::{check_if_test, input_path};

pub mod day8;

const DAY: u8 = 8;

fn main() {
    println!("Day 8: Resonant Collinearity");
    let test = check_if_test();
    if test {
        println!("Using test input");
    }
    day8::run(input_path(DAY, test));
}
