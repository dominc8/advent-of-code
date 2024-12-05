use std::env;
use std::fs;
use regex::Regex;

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
    let mut sum = 0;
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let muls: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    for mul in muls {
        let mut val_str_iter = mul.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap().split(",");
        let x = val_str_iter.next().unwrap().parse::<u32>().unwrap();
        let y = val_str_iter.next().unwrap().parse::<u32>().unwrap();
        sum += x * y;
    }
    println!("Part1: {}", sum);
}

#[allow(dead_code)]
fn part2(input: &str) {
    let mut sum = 0;
    let mut mult_factor = 1;
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let ops: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    for op in ops {
        if op.starts_with("m") {
            let mut val_str_iter = op.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap().split(",");
            let x = val_str_iter.next().unwrap().parse::<u32>().unwrap();
            let y = val_str_iter.next().unwrap().parse::<u32>().unwrap();
            sum += mult_factor * x * y;
        } else if op.starts_with("don") {
            mult_factor = 0;
        } else {
            mult_factor = 1;
        }
    }
    println!("Part2: {}", sum);
}
