use std::{collections::{HashMap, HashSet}, fs, thread::sleep, time::Duration};

pub fn run(path: String) {
    let (map, start_position) = parse_input(path);

    let mut guard = Guard::new(start_position);
    let print = false;

    while !guard.out_of_bounds {
        guard.handle_movement(&map);
        if guard.check_for_loops(&map) {
            guard.obstacle_opportunities.insert(guard.direction.forward(guard.position));
        }
        guard.add_step();
        if print {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Steps: {}", guard.steps.len());
            print_map(&map, &guard);
            println!();
            sleep(Duration::from_millis(100));
        }
    }
    println!("Part 1 result: {}", guard.steps.len());
    println!("Part 2 result: {}", guard.obstacle_opportunities.len());
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn forward(&self, position: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (position.0 - 1, position.1),
            Direction::Down => (position.0 + 1, position.1),
            Direction::Left => (position.0, position.1 - 1),
            Direction::Right => (position.0, position.1 + 1),
        }
    }

}

#[derive(Debug, Clone)]
struct Guard {
    position: (i32, i32),
    direction: Direction,
    steps: HashMap<(i32, i32), Vec<Direction>>,
    out_of_bounds: bool,
    obstacle_opportunities: HashSet<(i32, i32)>,
    start_position: (i32, i32),
}

impl Guard {
    fn new(position: (i32, i32)) -> Self {
        Guard {
            position,
            direction: Direction::Up,
            steps: HashMap::new(),
            out_of_bounds: false,
            obstacle_opportunities: HashSet::new(),
            start_position: position,
        }
    }

    fn move_forward(&mut self) {
        self.position = self.direction.forward(self.position);
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn add_step(&mut self) {
        let floor = self.steps.entry(self.position).or_insert(Vec::new());
        floor.push(self.direction);
    }

    fn handle_movement(&mut self, map: &Vec<Vec<char>>) {
        
        let look_ahead = self.direction.forward(self.position);
        // If heading out of bounds exit
        if out_of_bounds(map, look_ahead) {
            self.out_of_bounds = true;
            return;
        }
        // If obstacle ahead turn right and try to move again.
        let symbol = map[look_ahead.0 as usize][look_ahead.1 as usize];
        if symbol == '#' {
            self.turn_right();
        }
        // Move forward
        else {
            self.move_forward();
        }
        
        
        
    }

    fn check_for_loops(&self, map: &Vec<Vec<char>>) -> bool {
        let look_ahead = self.direction.forward(self.position);
        // Prevent setting obstacle at out of bounds, start position, existing obstacle or existing opportunity
        if out_of_bounds(map, look_ahead) 
            || self.obstacle_opportunities.contains(&look_ahead) 
            || map[look_ahead.0 as usize][look_ahead.1 as usize] == '#'
            || look_ahead == self.start_position
            || self.steps.contains_key(&look_ahead) {
            return false;
        }
        let mut adjusted_map = map.clone();
        //Add potential obstacle ahead
        adjusted_map[look_ahead.0 as usize][look_ahead.1 as usize] = '#';
        let mut adjusted_guard = self.clone();
        
        let mut loop_found = false;
        while !loop_found {
            adjusted_guard.handle_movement(&adjusted_map);
            if adjusted_guard.out_of_bounds {
                return false;
            }
            if let Some(previous_steps) = adjusted_guard.steps.get(&adjusted_guard.position) {
                if previous_steps.contains(&adjusted_guard.direction) {
                    loop_found = true;
                }
            }
            else {
                adjusted_guard.add_step();
            }   
        }
        loop_found
    }
}

fn out_of_bounds(map: &Vec<Vec<char>>, position: (i32, i32)) -> bool {
    let (i, j) = position;
    i < 0 || j < 0 || i >= map.len() as i32 || j >= map[0].len() as i32
}

fn print_map(map: &Vec<Vec<char>>, guard: &Guard) {
    
    let mut map_print = map.clone();
    guard.steps.keys().for_each(|(i, j)| {
        if !out_of_bounds(map, (*i, *j)) {
            map_print[*i as usize][*j as usize] = 'X';
        }
    });
    let guard_symbol = match guard.direction {
        Direction::Up => '^',
        Direction::Down => 'v',
        Direction::Left => '<',
        Direction::Right => '>',
    };
    if !out_of_bounds(map, guard.position) {
        map_print[guard.position.0 as usize][guard.position.1 as usize] = guard_symbol;
    }
    for line in map_print {
        for symbol in line {
            print!("{}", symbol);
        }
        println!();
    }
}

fn parse_input(path: String) -> (Vec<Vec<char>>, (i32, i32)) {
    let input = fs::read_to_string(path).unwrap();
    let mut start_position = (0, 0);
    let map: Vec<Vec<char>> = input.lines().enumerate().map(|(i, line)| {
        line.chars().enumerate().map(|(j, symbol)|{
            if symbol == '^' {
                start_position = (i as i32, j as i32);
                return '.';
            }
            symbol
        }).collect()
    }).collect();
    (map, start_position)
}