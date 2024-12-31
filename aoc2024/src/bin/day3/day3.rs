use regex::Regex;



pub fn run(path: String) {
    let input = std::fs::read_to_string(path).expect("Unable to read file");
    
    let muls: Vec<Mul> = find_muls(input.as_str());
    let result: i32 = muls.iter().map(|m| m.mul()).sum();
    println!("Result part 1: {}", result);

    let do_str = "do()";
    let dont_str = "don't()";

    let mut valid_muls: Vec<Mul> = Vec::new();
    let mut dont_split: Vec<&str> = input.split(dont_str).collect();

    // Find muls in the first part which is enabled by default
    valid_muls.append(&mut find_muls(dont_split.remove(0)));
    
    // Iterate over the dont parts
    for dont_part in dont_split {
        let mut do_split: Vec<&str> = dont_part.split(do_str).collect();
        // Remove the first part which is disabled
        do_split.remove(0);
        // Find muls in the remaining parts
        do_split.iter()
            .for_each(|part| valid_muls.append(&mut find_muls(part)));
    }

    //println!("Valid muls: {:?}", valid_muls);
    println!("Result part 2: {}", valid_muls.iter().map(|m| m.mul()).sum::<i32>());
}

fn find_muls(input: &str) -> Vec<Mul> {
    let re_mul = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let muls: Vec<Mul> = re_mul.find_iter(&input).map(|m| Mul::from_str(m.as_str())).collect();
    muls
}

#[derive(Debug)]
struct Mul {
    x: i32,
    y: i32,
}


impl Mul {
    fn from_str(mul_str: &str) -> Self {
        let nums: Vec<i32> = mul_str
            .split(|c| c == '(' || c == ',' || c == ')')
            .filter_map(|s| s.parse().ok())
            .collect();
        Self { x: nums[0], y: nums[1] }
    }

    fn mul(&self) -> i32 {
        self.x * self.y
    }
}