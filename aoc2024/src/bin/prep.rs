use std::fs;

fn main() {
    let args = std::env::args();
    let day = args.skip(1).next().expect("Please provide a day number");
    let day = day.parse::<u8>().expect("Day number must be a number");
    let main_template = "templates/main.txt";
    let lib_template = "templates/lib.txt";

    println!("Day {} prep", day);

    let day_folder = format!("src/bin/day{}", day);
    if fs::metadata(&day_folder).is_ok() {
        println!("Folder {} exists", day_folder);
    } else {
        println!("Folder {} does not exist", day_folder);
        fs::create_dir(&day_folder).expect("Failed to create folder");
        let main_text = fs::read_to_string(main_template).expect("Failed to read main template");
        let lib_text = fs::read_to_string(lib_template).expect("Failed to read lib template");
        let main_file = format!("{}/main.rs", day_folder);
        let lib_file = format!("{}/day{}.rs", day_folder, day);
        fs::write(main_file, main_text.replace("{{day}}", &day.to_string())).expect("Failed to write main file");
        fs::write(lib_file, lib_text.replace("{{day}}", &day.to_string())).expect("Failed to write lib file");
        println!("Created folder {} and code template.", day_folder);
    }

    let input = format!("input/day{}.txt", day);
    let test_input = format!("input/day{}_test.txt", day);
    if fs::metadata(&input).is_ok() {
        println!("Input file {} exists", input);
    } else {
        println!("Input file {} does not exist", input);
        fs::write(input, "").expect("Failed to create input file");
        println!("Created input file");
    }

    if fs::metadata(&test_input).is_ok() {
        println!("Test input file {} exists", test_input);
    } else {
        println!("Test input file {} does not exist", test_input);
        fs::write(test_input, "").expect("Failed to create test input file");
        println!("Created test input file");
    }

}