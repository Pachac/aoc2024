use aoc2024::{check_if_test, input_path};

pub mod day4;

const DAY: u8 = 4;

fn main() {
    println!("Day 4: Ceres Search");
    let test = check_if_test();
    if test {
        println!("Using test input");
    }
    day4::run(input_path(DAY, test));
}
