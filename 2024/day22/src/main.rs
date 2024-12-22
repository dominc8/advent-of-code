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
    part1(&input);
    //part2(&input);
}

struct SecretNumber(usize);

impl SecretNumber {
    fn evolve_n(&mut self, n: usize) {
        for _ in 0..n {
            self.evolve();
        }
    }

    fn evolve(&mut self) {
        self.mix_prune(self.0 * 64);
        self.mix_prune(self.0/32);
        self.mix_prune(self.0 * 2048);
    }

    fn mix_prune(&mut self, value: usize) {
        self.0 = (self.0 ^ value) % 16777216;
    }
}


#[allow(dead_code)]
fn part1(input: &str) {
    let mut secret_numbers = parse_input(input);

    let mut result = 0;
    for secret_number in secret_numbers.iter_mut() {
        secret_number.evolve_n(2000);
        result += secret_number.0;
    }
    println!("Part1: {}", result);
}


#[allow(dead_code)]
fn part2(input: &str) {
}

fn parse_input(input: &str) -> Vec<SecretNumber> {
    let mut secret_numbers = vec![];
    for line in input.lines() {
        secret_numbers.push(SecretNumber(line.parse::<usize>().unwrap()));
    }
    return secret_numbers;
}
