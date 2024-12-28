use std::env;
use std::fmt;
use std::fs;
//use itertools::Itertools;
use std::collections::HashMap;

fn read_file(filename: &str) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join(filename);

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

enum LogicalOperation {
    AND, OR, XOR
}

impl LogicalOperation {
    fn eval(&self, x: bool, y: bool) -> bool {
        match self {
            LogicalOperation::AND => x && y,
            LogicalOperation::OR => x || y,
            LogicalOperation::XOR => x != y,
        }
    }
}

struct Gate {
    op: LogicalOperation,
    input0: String,
    input1: String,
    output: String,
}

struct Device {
    wires: HashMap<String, Option<bool>>,
    gates: Vec<Gate>,
}

impl Device {
    fn run(&mut self) {
        loop {
            let mut finished = true;
            for gate in self.gates.iter() {
                let output_val = self.wires.get(&gate.output).unwrap();
                let input0_val = self.wires.get(&gate.input0).unwrap();
                let input1_val = self.wires.get(&gate.input1).unwrap();

                if output_val.is_none() && input0_val.is_some() && input1_val.is_some() {
                    let output_val = Some(gate.op.eval(input0_val.unwrap(), input1_val.unwrap()));
                    self.wires.insert(gate.output.clone(), output_val);
                    finished = false;
                }
            }
            if finished { return };
        }
    }

    fn part1(&self) -> usize {
        let mut result = 0;
        for wire in self.wires.iter() {
            if wire.0.starts_with('z') {
                let bit_idx = wire.0.strip_prefix('z').unwrap().parse::<usize>().unwrap();
                let bit_val = if wire.1.unwrap() { 1 } else { 0 };
                result += bit_val << bit_idx;
            }
        }
        return result;
    }
}

fn main() {
    let input = read_file("input.txt");
    part1(&input);
    part2(&input);
}


#[allow(dead_code)]
fn part1(input: &str) {
    let mut device = parse_input(input);
    //println!("{}", device);
    //println!("Running...");
    device.run();
    //println!("{}", device);

    let result = device.part1();
    println!("Part1: {}", result);
}


#[allow(dead_code)]
fn part2(input: &str) {
}

fn parse_input(input: &str) -> Device {
    let mut wires = HashMap::new();
    let mut gates = vec![];
    for line in input.lines() {
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        match words.len() {
            2 => {
                let wire = words.get(0).unwrap().strip_suffix(':').unwrap();
                let initial_value = words.get(1).unwrap().parse::<u32>().unwrap();
                let initial_value = initial_value == 1;
                wires.insert(wire.to_string(), Some(initial_value));
            },
            5 => {
                let input0 = words.get(0).unwrap();
                let op = match *words.get(1).unwrap() {
                    "AND" => LogicalOperation::AND,
                    "OR" => LogicalOperation::OR,
                    _ => LogicalOperation::XOR,
                };
                let input1 = words.get(2).unwrap();
                let output = words.get(4).unwrap();

                if !wires.contains_key(*input0) {
                    wires.insert(input0.to_string(), None);
                }
                if !wires.contains_key(*input1) {
                    wires.insert(input1.to_string(), None);
                }
                if !wires.contains_key(*output) {
                    wires.insert(output.to_string(), None);
                }

                let input0 = input0.to_string();
                let input1 = input1.to_string();
                let output = output.to_string();
                gates.push(Gate{op, input0, input1, output});

            },
            _ => (),
        }
    }
    return Device{wires, gates};
}

impl fmt::Display for LogicalOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op_str = match self {
            LogicalOperation::AND => "AND",
            LogicalOperation::OR => "OR",
            LogicalOperation::XOR => "XOR",
        };
        write!(f, "{}", op_str)
    }
}
impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} -> {}", self.input0, self.op, self.input1, self.output)
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for wire in self.wires.iter() {
            writeln!(f, "{}: {:?}", wire.0, wire.1)?;
        }
        writeln!(f, "")?;
        for gate in self.gates.iter() {
            writeln!(f, "{}", gate)?;
        }
        Ok(())
    }
}
