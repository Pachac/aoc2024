use std::{collections::HashMap, fs};
use aoc2024::input_path;


pub fn run(test: bool) {
    let path = input_path(1, test);
    let input = fs::read_to_string(path).expect("file not found");
    
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    
    input.lines().for_each( |line| {
        line.split_ascii_whitespace()
            .enumerate()
            .for_each(|(i, x)| {
                let num = x.parse::<i32>().unwrap();
                if i == 0 {
                    left.push(num);
                } else if i == 1{
                    right.push(num);
                }
                else {
                    panic!("Too many numbers in line");
                }
            });
    });

    left.sort();
    right.sort();
    
    let mut result1: i32 = 0;
    for i in 0..left.len() {
        let difference = left[i] - right[i];
        if difference < 0 {
            result1 -= difference;
        } else {
            result1 += difference;
        }
    }
    println!("Part 1 result: {result1}");

    
    let mut right_map = HashMap::new();
    right.iter().for_each(|x| {
        let count = right_map.entry(x).or_insert(0);
        *count += 1;
    });

    let mut result2: i32 = 0;
    for i in 0..left.len() {
        let val = left[i];
        let count = right_map.get(&val).unwrap_or(&0);
        result2 += val * count;
        
    }
    
    println!("Part 2 result: {result2}");
    
}

