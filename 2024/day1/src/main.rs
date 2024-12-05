use std::env;
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
    part2(&input);
}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for line in input.lines() {
        let mut line_iter = line.split_ascii_whitespace();
        v1.push(line_iter.next().unwrap().parse::<i32>().unwrap());
        v2.push(line_iter.next().unwrap().parse::<i32>().unwrap());
    }

    v1.sort();
    v2.sort();

    let mut diff = 0;
    for (x, y) in v1.iter().zip(v2) {
        diff += (x - y).abs();
    }
    println!("Part1: {}", diff);
}

#[allow(dead_code)]
fn part2(input: &str) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for line in input.lines() {
        let mut line_iter = line.split_ascii_whitespace();
        v1.push(line_iter.next().unwrap().parse::<i32>().unwrap());
        v2.push(line_iter.next().unwrap().parse::<i32>().unwrap());
    }

    v1.sort();
    v2.sort();

    let mut similarity = 0;
    for x in v1.iter() {
        let mut count = 0;
        for y in v2.iter() {
            if x == y {
                count += 1;
            }
        }
        similarity += x * count;
    }
    println!("Part2: {}", similarity);
}
