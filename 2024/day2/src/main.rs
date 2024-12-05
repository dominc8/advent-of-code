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

fn is_report_safe(levels: &Vec<i32>) -> bool {
    const MIN_ALLOWED_DIFF : i32 = 1;
    const MAX_ALLOWED_DIFF : i32 = 3;

    let mut vec_iter = levels.iter();
    let v0 = vec_iter.next().unwrap();
    let v1 = vec_iter.next().unwrap();

    let diff_mult;
    if v1 > v0 {
        diff_mult = 1;
    } else {
        diff_mult = -1;
    }
    let mut min_diff = (v1 - v0) * diff_mult;
    let mut max_diff = (v1 - v0) * diff_mult;

    let mut prev = v1;

    for curr in vec_iter {
        let diff = (curr - prev) * diff_mult;
        if diff > max_diff {
            max_diff = diff;
        }
        if diff < min_diff {
            min_diff = diff;
        }
        prev = curr;
    }

    return min_diff >= MIN_ALLOWED_DIFF && max_diff <= MAX_ALLOWED_DIFF;
}

fn clone_vector_without_index(vec: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut result;
    if index == vec.len() - 1 {
        result = vec.clone();
        result.pop();
    } else {
        let mut left = vec.clone();
        let right = left.split_off(index + 1);
        left.pop();
        result = [left, right].concat();
    }
    return result;
}

fn is_report_safe_with_tolerance(levels: &Vec<i32>) -> bool {
    if is_report_safe(levels) {
        return true;
    }

    let vec_len = levels.len();
    for idx in 0..vec_len {
        if is_report_safe(&clone_vector_without_index(levels, idx)) {
            return true;
        }
    }
    return false;
}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut n_safe_report = 0;
    for line in input.lines() {
        let mut v1 = Vec::new();
        for line_iter in line.split_ascii_whitespace() {
            v1.push(line_iter.parse::<i32>().unwrap());
        }
        if is_report_safe(&v1) {
            n_safe_report += 1;
        }

    }

    println!("Part1: {}", n_safe_report);
}

#[allow(dead_code)]
fn part2(input: &str) {
    let mut n_safe_report = 0;
    for line in input.lines() {
        let mut v1 = Vec::new();
        for line_iter in line.split_ascii_whitespace() {
            v1.push(line_iter.parse::<i32>().unwrap());
        }
        if is_report_safe_with_tolerance(&v1) {
            n_safe_report += 1;
        }

    }

    println!("Part2: {}", n_safe_report);
}
