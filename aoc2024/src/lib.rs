pub fn input_path(day: u8, test: bool) -> String {
    if test {
        format!("input/day{}_test.txt", day)
    } else {
        format!("input/day{}.txt", day)
    }
}

pub fn check_if_test() -> bool {
    std::env::args().any(|x| x == "test")
}