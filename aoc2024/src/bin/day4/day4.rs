const DIRECTIONS: [(i32,i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

pub fn run(path: String) {
    let input = std::fs::read_to_string(path).expect("Failed to read file");
    let puzzle = Puzzle::from_input(input);

    let text_to_find = "XMAS";
    let mut result = 0;
    for row in 0..puzzle.rows {
        for col in 0..puzzle.cols {
            result += find_xmas(&puzzle, text_to_find, row, col);
        }
    }
    println!("Result part 1: {}", result);

    let mut result2 = 0;
    for row in 0..puzzle.rows {
        for col in 0..puzzle.cols {
            let mas = find_x_mas(&puzzle, row, col);
            result2 += mas;
        }
    }
    println!("Result part 2: {}", result2);
}

fn find_xmas(puzzle: &Puzzle, xmas: &str, row: usize, col: usize) -> usize {
    if puzzle.letters[row][col] != xmas.chars().next().unwrap() {
        return 0;
    }
    let mut result: usize = 0;
    let xmas_len = xmas.len() - 1;
    DIRECTIONS.iter().for_each(|dir| {
        if ((dir.0 * xmas_len as i32) + row as i32)  >= 0 
        && ((dir.1 * xmas_len as i32) + col as i32)  >= 0 
        && ((dir.0 * xmas_len as i32) + row as i32)  < puzzle.rows as i32 
        && ((dir.1 * xmas_len as i32) + col as i32)  < puzzle.cols as i32 {
            let mut found = true;
            xmas.chars().enumerate().for_each(|(i, ch)| {
                let new_row = row as i32 + (i as i32 * dir.0);
                let new_col = col as i32 + (i as i32 * dir.1);
                if puzzle.letters[new_row as usize][new_col as usize] != ch {
                    found = false;
                }
            });
            if found {
                result += 1;
            }
        }
    });
    result
}


fn find_x_mas(puzzle: &Puzzle, row: usize, col: usize) -> usize {
    if row == 0 || col == 0 || row == puzzle.rows - 1 || col == puzzle.cols - 1 {
        return 0;
    }
    if puzzle.letters[row][col] != 'A' {
        return 0;
    }

    if ((puzzle.letters[row+1][col+1] == 'M' && puzzle.letters[row-1][col-1] == 'S') || 
        (puzzle.letters[row+1][col+1] == 'S' && puzzle.letters[row-1][col-1] == 'M'))
       &&
       ((puzzle.letters[row+1][col-1] == 'M' && puzzle.letters[row-1][col+1] == 'S') ||
        (puzzle.letters[row+1][col-1] == 'S' && puzzle.letters[row-1][col+1] == 'M')) {
        return 1;
    }
    0
}


#[derive(Debug)]
struct Puzzle {
    letters: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Puzzle {
    fn from_input(input: String) -> Self {
        let letters: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = letters.len();
        let cols = letters[0].len();
        Self { letters, rows, cols }
    }
}