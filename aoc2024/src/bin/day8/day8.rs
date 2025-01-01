use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

pub fn run(path: String) {
    let (antennas, dimensions) = parse_input(path);
    let mut result1: HashSet<Coordinate> = HashSet::new();
    for (_, coords) in antennas.iter() {
        let pairs = find_all_pairs(coords);
        pairs.iter().for_each(|(antenna1, antenna2)| {
            let distance = *antenna2 - *antenna1;
            let antinode1 = *antenna2 + distance;
            let antinode2 = *antenna1 - distance;

            if !antinode1.out_of_bounds(dimensions) {
                result1.insert(antinode1);
            }
            if !antinode2.out_of_bounds(dimensions) {
                result1.insert(antinode2);
            }
        });
    }
    println!("Part 1 result: {}", result1.len());
    let mut result2: HashSet<Coordinate> = HashSet::new();
    for (_, coords) in antennas.iter() {
        let pairs = find_all_pairs(coords);
        pairs.iter().for_each(|(antenna1, antenna2)| {
            let distance = *antenna2 - *antenna1;
            result2.insert(*antenna1);
            result2.insert(*antenna2);
            let mut antinode = *antenna2 + distance;
            loop {
                if antinode.out_of_bounds(dimensions) {
                    break;
                }
                result2.insert(antinode);
                antinode = antinode + distance;
            }
            let mut antinode = *antenna1 - distance;
            loop {
                if antinode.out_of_bounds(dimensions) {
                    break;
                }
                result2.insert(antinode);
                antinode = antinode - distance;
            }
        });
    }
    println!("Part 2 result: {}", result2.len());
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn out_of_bounds(&self, dimensions: Coordinate) -> bool {
        self.x < 0 || self.y < 0 || self.x >= dimensions.x || self.y >= dimensions.y
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn parse_input(path: String) -> (HashMap<char, Vec<Coordinate>>, Coordinate) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut antennas = HashMap::new();
    let max_x = input.lines().next().unwrap().len() as i32;
    let max_y = input.lines().count() as i32;
    let dimensions = Coordinate::new(max_x, max_y);
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, ch)| {
            match ch {
                '.' => (),
                _ => {
                    let coord = Coordinate::new(j as i32, i as i32);
                    let entry = antennas.entry(ch).or_insert(Vec::new());
                    entry.push(coord);
                }
            }
        });
    });
    (antennas, dimensions)
}

fn find_all_pairs(coords: &Vec<Coordinate>) -> Vec<(Coordinate, Coordinate)> {
    let mut pairs = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            pairs.push((coords[i], coords[j]));
        }
    }
    pairs
}