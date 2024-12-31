use std::{collections::{HashMap, HashSet}, fs};

pub fn run(path: String) {
    let input = fs::read_to_string(path).unwrap();
    let (rules, updates) = parse_input(input);

    let mut result1 = 0;
    let mut result2 = 0;
    let mut part2_max_iter = 0;
    updates.iter().for_each(|update| {
        let valid = check_update(update, &rules);
        if valid {
            let middle = (update.len() - 1) / 2;
            //println!("{:?}", update[middle]);
            result1 += update[middle];
        }
        else {
            let mut corrected = false;
            let mut corrected_update = update.clone();
            let mut iter = 0;
            while !corrected {
                iter += 1;
                corrected_update = reorganize_update(&corrected_update, &rules);
                if check_update(&corrected_update, &rules) {
                    corrected = true;
                    let middle = (corrected_update.len() - 1) / 2;
                    result2 += corrected_update[middle];
                    if iter > part2_max_iter {
                        part2_max_iter = iter;
                    }
                }
            }
        }
    });
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
    println!("Part 2 max iteration: {}", part2_max_iter);

}

fn parse_input(input: String) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut parse_rules = true;
    input.lines().for_each(|line| {
        if line == "" {
            parse_rules = false;
        }
        else if parse_rules {
            let (key, value) = parse_rule(line);
            let entry = rules.entry(key).or_insert(Vec::new());
            entry.push(value);
        }
        else {
            let update: Vec<i32> = line.split(",").map(|x| {
                let num: i32 = x.parse().unwrap();
                num
            }).collect();
            updates.push(update);
        }
    });

    (rules, updates)
}

fn parse_rule(line: &str) -> (i32, i32) {
    let split: Vec<&str> = line.split("|").collect();
    (split[1].parse().unwrap(), split[0].parse().unwrap())
}

fn check_update(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut ban_set: HashSet<i32> = HashSet::new();
    let mut processed: HashSet<i32> = HashSet::new();
    for page in update {
        if ban_set.contains(page) {
            return false
        }
        processed.insert(*page);
        if rules.contains_key(page) {
            for previous in rules.get(page).unwrap() {
                if !processed.contains(previous) {
                    ban_set.insert(*previous);
                }
            }
        }
    }
    true
}

fn reorganize_update(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut ban_set: HashMap<i32, i32> = HashMap::new();
    let mut processed: HashSet<i32> = HashSet::new();
    let mut new_update: Vec<i32> = update.clone();
    for (i, page) in update.iter().enumerate() {
        if ban_set.contains_key(page) {
            let wrong_page = new_update.remove(i);
            let index = new_update.iter().position(|x| x == ban_set.get(page).unwrap()).unwrap();
            new_update.insert(index, wrong_page);
            return new_update;
        }
        processed.insert(*page);
        if rules.contains_key(page) {
            for previous in rules.get(page).unwrap() {
                if !processed.contains(previous) {
                    ban_set.entry(*previous).or_insert(*page);
                }
            }
        }
    }
    panic!("Shouldn't be able to reach this point if correct.");
}