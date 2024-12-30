use std::env;
use std::fmt;
use std::fs;
use itertools::Itertools;

fn read_file(filename: &str) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join(filename);

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}


struct PinTuple(i32, i32, i32, i32, i32);

impl PinTuple {
    fn check(&self, other: &Self) -> bool {
        (self.0 + other.0 < 6) && 
        (self.1 + other.1 < 6) && 
        (self.2 + other.2 < 6) && 
        (self.3 + other.3 < 6) && 
        (self.4 + other.4 < 6)
    }
}

struct Lock(PinTuple);
struct Key(PinTuple);

fn main() {
    let input = read_file("input.txt");
    part1(&input);
    part2(&input);
}


#[allow(dead_code)]
fn part1(input: &str) {
    let (locks, keys) = parse_input(input);
    let mut result = 0;

    for key in keys.iter() {
        //println!("{}", key);
        for lock in locks.iter() {
            //println!("{}", lock);
            if key.0.check(&lock.0) {
                result += 1;
            }
        }
    }
    println!("Part1: {}", result);
}


#[allow(dead_code)]
fn part2(input: &str) {
}

fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = vec![];
    let mut keys = vec![];

    for chunk in &input.lines().chunks(8) {
        let mut lines = chunk.collect_vec();
        let last_chunk_line = lines.pop().unwrap();
        if last_chunk_line.len() == 0 {
            lines.pop();
        }
        match lines[0] {
            "....." => {
                let key = Key(parse_pin_tuple(&lines));
                keys.push(key);
            },
            _ => {
                let lock = Lock(parse_pin_tuple(&lines));
                locks.push(lock);
            }
        }
    }

    return (locks, keys);
}

fn parse_pin_tuple(lines: &Vec<&str>) -> PinTuple {
    let mut pin_count = [0;5];
    for line in &lines[1..] {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                pin_count[i] += 1;
            }
        }
    }

    return PinTuple(pin_count[0], pin_count[1], pin_count[2], pin_count[3], pin_count[4])
}

impl fmt::Display for Lock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lock({})", self.0)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key({})", self.0)
    }
}

impl fmt::Display for PinTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{},{},{}", self.0, self.1, self.2, self.3, self.4)
    }
}
