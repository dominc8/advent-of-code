use std::env;
use std::fmt;
use std::fs;

fn read_file(filename: &str) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join(filename);

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

fn main() {
    let input = read_file("input.txt");
    part1(&input);
    part2(&input);
}


#[derive(Clone,Copy,PartialEq,Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Clone,Copy)]
enum MapField {
    FreeSpace(bool),
    Obstacle,
}

#[derive(Debug)]
struct Position {
    x : i32,
    y : i32,
}

#[derive(Clone,Copy)]
struct Guard {
    move_direction: Direction,
    x : i32,
    y : i32
}

#[derive(Clone)]
struct MapState {
    map_field: Vec<Vec<MapField>>,
    guard: Guard
}

#[derive(PartialEq,Eq)]
struct GuardOutOfBounds(bool);


impl MapState {
    fn move_guard(&mut self) -> GuardOutOfBounds {
        self.map_field[self.guard.y as usize][self.guard.x as usize] = MapField::FreeSpace(true);
        let (new_x, new_y) = match self.guard.move_direction {
            Direction::Up => (self.guard.x, self.guard.y - 1),
            Direction::Down => (self.guard.x, self.guard.y + 1),
            Direction::Left => (self.guard.x - 1, self.guard.y),
            Direction::Right => (self.guard.x + 1, self.guard.y),
        };
        if new_x < 0 || new_y < 0 {
            return GuardOutOfBounds(true);
        }
        let old_direction = self.guard.move_direction;
        let new_direction = match self.map_field.get(new_y as usize) {
            Some(row) => 
            {
                match row.get(new_x as usize) {
                    Some(field) => match field {
                        MapField::Obstacle => old_direction.rotate_right(),
                        _ => old_direction,
                    }
                    None => return GuardOutOfBounds(true)
                }
            },
            None => return GuardOutOfBounds(true)
        };
        if old_direction == new_direction {
            self.guard.x = new_x;
            self.guard.y = new_y;
        } else {
            self.guard.move_direction = new_direction;
        }

        return GuardOutOfBounds(false);
    }

    fn count_visited_fields(&self) -> u32 {
        let mut count = 0;
        for row in &self.map_field {
            for field in row {
                count += match field {
                    MapField::FreeSpace(true) => 1,
                    _ => 0,
                }
            }
        }
        return count;
    }

    fn clone_with_new_obstacle(&self, pos: &Position) -> Self {
        let mut new_map_state = self.clone();
        new_map_state.map_field[pos.y as usize][pos.x as usize] = MapField::Obstacle;
        return new_map_state;
    }

    fn guard_is_in_loop(&mut self) -> bool {
        let mut loop_counter = 0;
        let mut last_visited_fields_count = 0;
        while self.move_guard() == GuardOutOfBounds(false) {
            loop_counter += 1;
            if loop_counter % 1024 == 0 {
                let visited_fields_count = self.count_visited_fields();
                if last_visited_fields_count == visited_fields_count {
                    return true;
                }
                last_visited_fields_count = visited_fields_count;
            }
            //println!("----------------");
            //println!("{}", map_state);
        }
        return false;
    }
}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut map_state = create_map_state(input);
    //println!("{}", map_state);

    while map_state.move_guard() == GuardOutOfBounds(false) {
        //println!("----------------");
        //println!("{}", map_state);
    }

    println!("Part1: {}", map_state.count_visited_fields());
}

#[allow(dead_code)]
fn part2(input: &str) {
    let map_state = create_map_state(input);
    let new_obstacle_positions = get_possible_new_obstacle_positions(input);
    let mut found_loops_count = 0;

    for pos in new_obstacle_positions.iter() {
        let mut new_map_state = map_state.clone_with_new_obstacle(pos);
        if new_map_state.guard_is_in_loop() {
            found_loops_count += 1;
        }
    }

    println!("Part2: {}", found_loops_count);
}

fn create_map_state(input: &str) -> MapState {
    let mut map_field = vec![];
    let mut guard = Guard{move_direction: Direction::Up, x: 0, y: 0};
    for (row_idx, line) in input.lines().enumerate() {
        map_field.push(Vec::from_iter(line.chars().enumerate().map(
                    |(col_idx, c)| match c {
                        '^' => {
                            guard.x = col_idx as i32;
                            guard.y = row_idx as i32;
                            MapField::FreeSpace(true)
                        }
                        '#' => MapField::Obstacle,
                        _ => MapField::FreeSpace(false)
                    }
                    )));
    }
    MapState{map_field, guard}
}

fn get_possible_new_obstacle_positions(input: &str) -> Vec<Position> {
    let mut free_space_vec = vec![];
    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, c) in line.chars().enumerate() {
            if c == '.' {
                let y = row_idx as i32;
                let x = col_idx as i32;
                free_space_vec.push(Position{x, y});
            }
        }
    }
    return free_space_vec;
}

impl fmt::Display for MapField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char_to_write = match self {
            MapField::Obstacle => '#',
            MapField::FreeSpace(false) => '.',
            MapField::FreeSpace(true) => 'X',
        };
        write!(f,"{}", char_to_write)
    }
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char_to_write = match self.move_direction {
            Direction::Up => '^',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Down => 'v',
        };
        write!(f,"{}", char_to_write)
    }
}

impl fmt::Display for MapState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut row_idx = 0;
        for row in &self.map_field {
            let mut col_idx = 0;
            for field in row {
                if (self.guard.x, self.guard.y) == (col_idx, row_idx) {
                    write!(f,"{}", self.guard)?;
                } else {
                    write!(f,"{}", field)?;
                }
                col_idx += 1;
            }
            row_idx += 1;
            writeln!(f, "")?;
        }
        Ok(())
    }
}

