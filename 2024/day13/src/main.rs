use std::env;
use std::fmt;
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
    part1(&input);
    part2(&input);
}

#[derive(Clone)]
struct ButtonBehaviour {
    dx: usize,
    dy: usize
}

#[derive(Clone)]
struct MachineInfo {
    button_a: ButtonBehaviour,
    button_b: ButtonBehaviour,
    prize_pos: (usize, usize)
}

impl MachineInfo {
    fn calculate_token_cost(&self) -> Option<usize> {
        let (px, py) = self.prize_pos;
        let (px, py) = (px as isize, py as isize);
        let (xa, ya) = (self.button_a.dx as isize, self.button_a.dy as isize);
        let (xb, yb) = (self.button_b.dx as isize, self.button_b.dy as isize);
        let n_a = (px*yb - py*xb) / (xa*yb - ya*xb);
        let n_b = (py - n_a*ya) / yb;

        let px_matches = px == (n_a * xa + n_b * xb);
        let py_matches = py == (n_a * ya + n_b * yb);

        if px_matches && py_matches {
            return Some((3*n_a + n_b) as usize);
        } else {
            return None;
        }
    }
}

#[allow(dead_code)]
fn part1(input: &str) {
    let machine_infos = parse_input(input);
    //for machine_info in machine_infos.iter() {
    //    println!("{}", machine_info);
    //}
    let result = machine_infos.iter().map(|m| m.calculate_token_cost().unwrap_or(0)).sum::<usize>();

    println!("Part1: {}", result);
}

#[allow(dead_code)]
fn part2(input: &str) {
    let mut machine_infos = parse_input(input);
    for machine_info in machine_infos.iter_mut() {
        machine_info.prize_pos.0 += 10000000000000;
        machine_info.prize_pos.1 += 10000000000000;
        //println!("{}", machine_info);
    }
    let result = machine_infos.iter().map(|m| m.calculate_token_cost().unwrap_or(0)).sum::<usize>();

    println!("Part2: {}", result);
}

fn parse_input(input: &str) -> Vec<MachineInfo> {
    let mut machine_infos = vec![];

    let re_button = Regex::new(r"Button [AB]: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let mut line_iter = input.lines();
    loop {
        let button_a_captures = re_button.captures(line_iter.next().unwrap()).unwrap();
        let button_b_captures = re_button.captures(line_iter.next().unwrap()).unwrap();
        let prize_captures = re_prize.captures(line_iter.next().unwrap()).unwrap();

        let button_a = ButtonBehaviour{
            dx: button_a_captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            dy: button_a_captures.get(2).unwrap().as_str().parse::<usize>().unwrap()
        };
        let button_b = ButtonBehaviour{
            dx: button_b_captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            dy: button_b_captures.get(2).unwrap().as_str().parse::<usize>().unwrap()
        };

        let prize_pos = (
            prize_captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            prize_captures.get(2).unwrap().as_str().parse::<usize>().unwrap()
        );

        machine_infos.push(MachineInfo { button_a, button_b, prize_pos });
        if line_iter.next().is_none() { break }
    }

    return machine_infos;
}

impl fmt::Display for MachineInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A: ({}), B: ({}), Prize: ({},{})",
            self.button_a, self.button_b, self.prize_pos.0, self.prize_pos.1)
    }
}

impl fmt::Display for ButtonBehaviour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X+{}, Y+{}", self.dx, self.dy)
    }
}
