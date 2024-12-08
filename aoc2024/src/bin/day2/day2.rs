use std::fs;

fn parse_input(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|x| 
            x.split_ascii_whitespace()
            .map(|y| 
                y.parse::<i32>().unwrap()
            ).collect()
        ).collect()
}

fn check_report_safety(report: &Vec<i32>, problem_dampener: bool) -> bool {
    let mut dampening_level = 0;
    let mut previous = report[0];
    let ascending = report[1] > previous;
    for i in 1..report.len() {
        let difference: i32;
        if ascending {
            difference = report[i] - previous;
        }
        else {
            difference = previous - report[i];
        }
        if difference < 1 || difference > 3 {
            if problem_dampener{
                // Try removing previous entry
                let mut adjusted_report = report.clone();
                adjusted_report.remove(i-1);

                let remove_previous = check_report_safety(&adjusted_report, false);
                if remove_previous {
                    return true;
                }
                if dampening_level > 0 {
                    return check_report_safety(&report[1..].to_vec(), false);
                }
                dampening_level += 1;
                continue;
            }
            return false;
        }
        previous = report[i];
    }
    true
}

pub fn run(input_path: String) {
    let input = fs::read_to_string(input_path).expect("Unable to read file");
    let reports = parse_input(input);
    let valid_reports1 = reports.iter().filter(|x| check_report_safety(x, false)).count();
    let valid_reports2 = reports.iter().filter(|x| check_report_safety(x, true)).count();
    
    println!("Part 1 result: {:?}", valid_reports1);
    println!("Part 2 result: {:?}", valid_reports2);
}