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

struct Robot {
    pos: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn tick(&mut self, lim: (isize, isize)) {
        let new_x = (self.pos.0 + self.velocity.0) % lim.0;
        let new_y = (self.pos.1 + self.velocity.1) % lim.1;

        self.pos.0 = (new_x + lim.0) % lim.0;
        self.pos.1 = (new_y + lim.1) % lim.1;
    }
}

struct MapState {
    width: isize,
    height: isize,
    robots: Vec<Robot>,
}

impl MapState {
    fn tick(&mut self) {
        self.robots.iter_mut().for_each(|r| r.tick((self.width, self.height)));
    }

    fn tick_n(&mut self, n: usize) {
        for _ in 0..n { self.tick() }
    }

    fn find_next_symmetrical_tick(&mut self) -> usize {
        let mut n_tick = 0;
        loop {
            self.tick();
            n_tick += 1;
            let (a, b, c, d) = self.calculate_quadrants_count();
            if (a == b) && (c == d) && self.is_symmetrical() { return n_tick; }
        }
    }

    fn calculate_quadrants_count(&self) -> (usize, usize, usize, usize) {
        let mut top_left_count = 0;
        let mut top_right_count = 0;
        let mut bottom_left_count = 0;
        let mut bottom_right_count = 0;

        for robot in self.robots.iter() {
            let is_left = robot.pos.0 < self.width / 2;
            let is_right = robot.pos.0 > self.width / 2;
            let is_top = robot.pos.1 < self.height / 2;
            let is_bottom = robot.pos.1 > self.height / 2;

            if is_left && is_top { top_left_count += 1 }
            if is_right && is_top { top_right_count += 1 }
            if is_left && is_bottom { bottom_left_count += 1 }
            if is_right && is_bottom { bottom_right_count += 1 }
        }

        return (top_left_count, top_right_count, bottom_left_count, bottom_right_count);
    }

    fn calculate_safety_factor(&self) -> usize {
        let (a, b, c, d) = self.calculate_quadrants_count();
        return a * b * c * d;
    }

    fn is_symmetrical(&self) -> bool {
        let mut char_map: Vec<char> = vec!['.'; (self.width * self.height) as usize];

        for robot in self.robots.iter() {
            let lin_pos = robot.pos.0 + robot.pos.1 * self.width;
            *char_map.get_mut(lin_pos as usize).unwrap() = '1';
        }

        let h = self.height as usize;
        let w = self.width as usize;

        for i in 0..h {
            let line = char_map.get(i*w..(i+1)*w).unwrap();
            let mut has_symmetry = true;
            for j in 0..w/2 {
                has_symmetry = has_symmetry && line[j] == line[w - j - 1];
            }
            if !has_symmetry { return false }
        }
        return true;
    }
}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut map_state = parse_input(input);
    println!("{}", map_state);
    map_state.tick_n(100);
    println!("{}", map_state);
    let result = map_state.calculate_safety_factor();

    println!("Part1: {}", result);
}


#[allow(dead_code)]
fn part2(input: &str) {
    let mut map_state = parse_input(input);

    for _ in 0..100 {
        let n_tick = map_state.find_next_symmetrical_tick();
        println!("{}", map_state);
        println!("Part2: {}", n_tick);
    }
}

fn parse_input(input: &str) -> MapState {
    let mut robots = vec![];
    for line in input.lines() {
        let mut word_iter = line.split_ascii_whitespace();
        let mut pos_val_iter = word_iter.next().unwrap().strip_prefix("p=").unwrap().split(',');
        let pos = (pos_val_iter.next().unwrap().parse::<isize>().unwrap(), pos_val_iter.next().unwrap().parse::<isize>().unwrap());

        let mut velocity_val_iter = word_iter.next().unwrap().strip_prefix("v=").unwrap().split(',');
        let velocity = (velocity_val_iter.next().unwrap().parse::<isize>().unwrap(), velocity_val_iter.next().unwrap().parse::<isize>().unwrap());

        robots.push(Robot{pos, velocity});
    }
    //example.txt
    let width = 11;
    let height = 7;
    //input.txt
    let width = 101;
    let height = 103;


    return MapState{width, height, robots};
}

impl fmt::Display for MapState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut char_map: Vec<char> = vec!['.'; (self.width * self.height) as usize];

        for robot in self.robots.iter() {
            let lin_pos = robot.pos.0 + robot.pos.1 * self.width;
            let mut_char = char_map.get_mut(lin_pos as usize).unwrap();
            if *mut_char == '.' {
                *mut_char = '1';
            } else {
                *mut_char = (mut_char.to_digit(10).unwrap() + 1).to_string().chars().next().unwrap();
            }
        }
        for (i, c) in char_map.iter().enumerate() {
            write!(f, "{}", c)?;
            if (i+1) as isize % self.width == 0 {
                writeln!(f, "")?;
            }
        }

        Ok(())
    }
}
