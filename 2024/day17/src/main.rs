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
    part2(&input);
}

#[derive(Debug)]
struct ComputerState {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    instr_ptr: usize,
    program: Vec<usize>,
    output: Vec<usize>,
}

impl ComputerState {
    fn part1(&mut self) {
        loop {
            let opcode = self.program.get(self.instr_ptr);
            let operand = self.program.get(self.instr_ptr + 1);

            if opcode.is_none() || operand.is_none() { break; }

            let &opcode = opcode.unwrap();
            let &operand = operand.unwrap();

            self.execute_operation(opcode, operand);

            //println!("Opcode: {}, Operand: {}, {:?}", opcode, operand, self);
        }
        let s = self.output.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        println!("{}", s);
    }

    fn map_combo_operand(&self, operand: usize) -> usize {
        match operand {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => operand,
        }
    }

    fn execute_operation(&mut self, opcode: usize, operand: usize) {
        let combo_operand = self.map_combo_operand(operand);
        match opcode {
            0 => { self.reg_a = self.reg_a / (1 << combo_operand); self.instr_ptr += 2; },
            1 => { self.reg_b = self.reg_b ^ operand; self.instr_ptr += 2; },
            2 => { self.reg_b = combo_operand % 8; self.instr_ptr += 2; },
            3 => { if self.reg_a != 0 { self.instr_ptr = operand; } else { self.instr_ptr += 2; }},
            4 => { self.reg_b = self.reg_b ^ self.reg_c; self.instr_ptr += 2; },
            5 => { self.output.push(combo_operand % 8); self.instr_ptr += 2; },
            6 => { self.reg_b = self.reg_a / (1 << combo_operand); self.instr_ptr += 2; },
            _ => { self.reg_c = self.reg_a / (1 << combo_operand); self.instr_ptr += 2; },
        }
    }


    fn run_part2(&mut self) {

        let init_reg_a = 0;
        let init_reg_a = self.run_part2_inner(init_reg_a, 4).unwrap();
        let init_reg_a = self.run_part2_inner(init_reg_a * (8 as usize).pow(4), 8).unwrap();
        let init_reg_a = self.run_part2_inner(init_reg_a * (8 as usize).pow(4), 12).unwrap();
        let init_reg_a = self.run_part2_inner(init_reg_a * (8 as usize).pow(4), self.program.len() as u32).unwrap();
        println!("{}", init_reg_a);
    }

    fn run_part2_inner(&mut self, init_reg_a: usize, n_output: u32) -> Option<usize> {
        let mut init_reg_a = init_reg_a;
        let init_reg_a_limit = (8 as usize).pow(n_output);
        let mut cmp_program = self.program.clone();
        for _ in 0..(self.program.len() - n_output as usize) { cmp_program.remove(0); }

        loop {
            self.reg_a = init_reg_a;
            self.reg_b = 0;
            self.reg_c = 0;
            self.instr_ptr = 0;
            self.output = vec![];
            loop {
                let opcode = self.program.get(self.instr_ptr);
                let operand = self.program.get(self.instr_ptr + 1);

                if opcode.is_none() || operand.is_none() { break; }

                let &opcode = opcode.unwrap();
                let &operand = operand.unwrap();

                self.execute_operation(opcode, operand);

                if !cmp_program.starts_with(&self.output) {
                    break;
                } else if self.output.len() == cmp_program.len() {
                    return Some(init_reg_a);
                }

                //println!("Opcode: {}, Operand: {}, {:?}", opcode, operand, self);
            }
            init_reg_a += 1;
            if init_reg_a >= init_reg_a_limit { return None; }
        }
    }
}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut computer_state = parse_input(input);
    //println!("{:?}", computer_state);

    println!("Part1:");
    computer_state.part1();
}


#[allow(dead_code)]
fn part2(input: &str) {
    let mut computer_state = parse_input(input);
    println!("Part2:");
    computer_state.run_part2();
}

fn parse_input(input: &str) -> ComputerState {
    let mut lines_iter = input.lines();

    let reg_a = lines_iter.next().unwrap().split(": ").skip(1).next().unwrap().parse::<usize>().unwrap();
    let reg_b = lines_iter.next().unwrap().split(": ").skip(1).next().unwrap().parse::<usize>().unwrap();
    let reg_c = lines_iter.next().unwrap().split(": ").skip(1).next().unwrap().parse::<usize>().unwrap();
    let instr_ptr = 0;
    
    lines_iter.next();

    let program = lines_iter.next().unwrap().split(": ").skip(1).next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect();

    return ComputerState { reg_a, reg_b, reg_c, instr_ptr, program, output: vec![] };
}
