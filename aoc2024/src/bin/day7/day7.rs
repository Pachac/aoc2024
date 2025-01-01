pub fn run(path: String) {
    let equations = parse_equations(path);
    let mut result = 0;
    equations.iter().for_each(|equation| {
        if equation.is_solvable() {
            result += equation.result;
        }
    });
    println!("Sum of solvable equations: {}", result);
}

#[derive(Debug)]
struct Equation {
    result: u64,
    values: Vec<u64>,
    operators: Vec<char>,
}

impl Equation {
    fn from_string(line: &str) -> Self {
        let split: Vec<&str> = line.split(":").collect();
        if split.len() != 2 {
            panic!("Invalid equation: {line}");
        }
        let result = split[0].parse().unwrap();
        let values = split[1].split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
        Self {
            result,
            values,
            operators: vec!['+', '*', '|'],
        }
    }

    fn is_solvable(&self) -> bool {
        let operators = find_combinations(&self.operators, self.values.len() - 1);
        for operator in &operators {
            if self.is_solution(operator) {
                return true;
            }
        }
        false
    }

    fn is_solution(&self, operators: &Vec<char>) -> bool {
        let mut calculation = self.values[0];
        for (i, operator) in operators.iter().enumerate() {
            let value = self.values[i+1];
            match operator {
                '+' => calculation += value,
                '*' => calculation *= value,
                '|' => calculation = concatenate_numbers(calculation, value),
                _ => panic!("Invalid operator: {}", operator),
            }
            if calculation > self.result {
                return false;
            }
        }   
        calculation == self.result
    }
}

fn parse_equations(path: String) -> Vec<Equation> {
    let input = std::fs::read_to_string(path).unwrap();
    input.lines().map(|line| Equation::from_string(line)).collect()
}

fn find_combinations(operators: &Vec<char>, length: usize) -> Vec<Vec<char>> {
    let mut combinations = Vec::new();
    let mut current = Vec::new();
    find_combinations_recursive(&operators, length, &mut current, &mut combinations);
    combinations
}

fn find_combinations_recursive(operators: &Vec<char>, length: usize, current: &mut Vec<char>, combinations: &mut Vec<Vec<char>>) {
    if current.len() == length {
        combinations.push(current.clone());
        return;
    }
    for operator in operators {
        current.push(*operator);
        find_combinations_recursive(operators, length, current, combinations);
        current.pop();
    }
}

fn concatenate_numbers(left_number: u64, right_number: u64) -> u64 {
    left_number * 10u64.pow(right_number.ilog10() + 1) + right_number
}