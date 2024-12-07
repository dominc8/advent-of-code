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

fn main() {
    let input = read_file("input.txt");
    part1(&input);
    part2(&input);
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
enum Operator2 {
    Add,
    Multiply,
    Concatenate,
}

#[derive(Debug)]
struct Equation {
    result : u64,
    numbers: Vec<u64>
}

impl Equation {
    fn is_valid(&self) -> bool {
        for op_variation in (0..(self.numbers.len() - 1)).map(|_| vec![Operator::Add, Operator::Multiply]).multi_cartesian_product() {
            let mut number_iter = self.numbers.iter();
            let mut value = *number_iter.next().unwrap();
            for (op, number) in op_variation.into_iter().zip(number_iter) {
                value = match op {
                    Operator::Add => value + number,
                    Operator::Multiply => value * number,
                };
            }
            if value == self.result {
                return true;
            }
        }
        return false;
    }

    fn is_valid2(&self) -> bool {
        for op_variation in (0..(self.numbers.len() - 1)).map(|_| vec![Operator2::Add, Operator2::Multiply, Operator2::Concatenate]).multi_cartesian_product() {
            //println!("op_variation: {:?}, numbers: {:?}", op_variation, self.numbers);
            let mut number_iter = self.numbers.iter();
            let mut value = *number_iter.next().unwrap();
            for (op, number) in op_variation.into_iter().zip(number_iter) {
                //println!("{}", value);
                value = match op {
                    Operator2::Add => value + number,
                    Operator2::Multiply => value * number,
                    Operator2::Concatenate => value * (10 as u64).pow(count_digits(*number)) + number,
                };
            }
            if value == self.result {
                return true;
            }
        }
        return false;
    }
}

fn count_digits(x: u64) -> u32 {
    if x == 0 { return 1 };
    let mut n_digits = 0;
    let mut x = x;
    while x > 0 {
        n_digits += 1;
        x /= 10;
    }
    return n_digits;

}

#[allow(dead_code)]
fn part1(input: &str) {
    let equations = parse_equations(input);
    let result = equations.iter()
        .map(|eq| if eq.is_valid() { eq.result } else { 0 })
        .reduce(|acc, result| acc + result).unwrap_or_default();

    println!("Part1: {}", result);
}

fn parse_equations(input: &str) -> Vec<Equation> {
    let mut equations = vec![];
    for line in input.lines() {
        let mut elements_iter = line.split_ascii_whitespace();
        let result = elements_iter.next().unwrap().strip_suffix(':').unwrap().parse::<u64>().unwrap();

        let mut equation = Equation{result, numbers: vec![]};
        for num_str in elements_iter {
            equation.numbers.push(num_str.parse::<u64>().unwrap());
        }
        equations.push(equation);
    }

    return equations;
}


#[allow(dead_code)]
fn part2(input: &str) {
    let equations = parse_equations(input);
    let result = equations.iter()
        .map(|eq| if eq.is_valid2() { eq.result } else { 0 })
        .reduce(|acc, result| acc + result).unwrap_or_default();

    println!("Part2: {}", result);
}
