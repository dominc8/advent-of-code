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
    //part2(&input);
}

#[derive(Clone)]
enum NumericButton {
    B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, BA
}

#[derive(Clone, PartialEq, Eq)]
enum DirectionalButton {
    BLeft, BRight, BUp, BDown, BA
}

#[derive(Clone)]
enum Direction {
    Up, Down, Left, Right
}


struct NumericKeypad {
    curr_button: NumericButton
}

//+---+---+---+
//| 7 | 8 | 9 |
//+---+---+---+
//| 4 | 5 | 6 |
//+---+---+---+
//| 1 | 2 | 3 |
//+---+---+---+
//    | 0 | A |
//    +---+---+


impl NumericKeypad {
    fn execute_code(&mut self, code: &str) -> Vec<Vec<DirectionalButton>> {
        println!("Executing code: {}", code);
        let mut button_sequences = vec![];
        for c in code.chars() {
            let next_button = match c {
                '0' => NumericButton::B0,
                '1' => NumericButton::B1,
                '2' => NumericButton::B2,
                '3' => NumericButton::B3,
                '4' => NumericButton::B4,
                '5' => NumericButton::B5,
                '6' => NumericButton::B6,
                '7' => NumericButton::B7,
                '8' => NumericButton::B8,
                '9' => NumericButton::B9,
                 _ => NumericButton::BA,
            };
            let potential_button_sequence = self.move_to(next_button);

            let mut button_sequence = if potential_button_sequence.eq(&[DirectionalButton::BUp, DirectionalButton::BUp, DirectionalButton::BLeft]) {
                vec![DirectionalButton::BLeft, DirectionalButton::BUp, DirectionalButton::BUp]
            } else {
                potential_button_sequence
            };

            button_sequence.push(DirectionalButton::BA);
            button_sequences.push(button_sequence);
        }

        for button_sequence in button_sequences.iter() {
            for button in button_sequence.iter() {
                print!("{}", button);
            }
        }
        println!("");
        return button_sequences;
    }

    fn move_to(&mut self, dest_button: NumericButton) -> Vec<DirectionalButton> {
        let curr_pos = self.button_to_pos(&self.curr_button);
        let dest_pos = self.button_to_pos(&dest_button);

        let dx = dest_pos.0 - curr_pos.0;
        let dy = dest_pos.1 - curr_pos.1;

        let x_dir = if dx > 0 { DirectionalButton::BRight } else { DirectionalButton::BLeft };
        let y_dir = if dy > 0 { DirectionalButton::BUp } else { DirectionalButton::BDown };

        let mut vec_x = vec![x_dir; dx.abs() as usize];
        let mut vec_y = vec![y_dir; dy.abs() as usize];

        self.curr_button = dest_button;

        if curr_pos.1 == 0 {
            vec_y.append(&mut vec_x);
            return vec_y
        } else {
            vec_x.append(&mut vec_y);
            return vec_x
        }
    }

    fn button_to_pos(&self, button: &NumericButton) -> (isize, isize) {
        match button {
            NumericButton::B0 => (1,0),
            NumericButton::B1 => (0,1),
            NumericButton::B2 => (1,1),
            NumericButton::B3 => (2,1),
            NumericButton::B4 => (0,2),
            NumericButton::B5 => (1,2),
            NumericButton::B6 => (2,2),
            NumericButton::B7 => (0,3),
            NumericButton::B8 => (1,3),
            NumericButton::B9 => (2,3),
            NumericButton::BA => (2,0),
        }
    }

    fn pos_to_button(&self, pos: (isize, isize)) -> NumericButton {
        match pos {
            (1,0) => NumericButton::B0,
            (0,1) => NumericButton::B1,
            (1,1) => NumericButton::B2,
            (2,1) => NumericButton::B3,
            (0,2) => NumericButton::B4,
            (1,2) => NumericButton::B5,
            (2,2) => NumericButton::B6,
            (0,3) => NumericButton::B7,
            (1,3) => NumericButton::B8,
            (2,3) => NumericButton::B9,
            _ => NumericButton::BA,
        }
    }
}

struct DirectionalKeypad {
    curr_button: DirectionalButton
}

//    +---+---+
//    | ^ | A |
//+---+---+---+
//| < | v | > |
//+---+---+---+

impl DirectionalKeypad {
    fn execute_sequences(&mut self, sequences: &Vec<Vec<DirectionalButton>>) -> Vec<Vec<DirectionalButton>> {
        let mut button_sequences = vec![];
        for sequence in sequences {
            for button in sequence {
                let mut button_sequence = self.move_to(button);
                button_sequence.push(DirectionalButton::BA);
                button_sequences.push(button_sequence);
            }
        }

        for button_sequence in button_sequences.iter() {
            for button in button_sequence.iter() {
                print!("{}", button);
            }
        }
        println!("");
        return button_sequences;
    }

    fn move_to(&mut self, dest_button: &DirectionalButton) -> Vec<DirectionalButton> {
        let curr_pos = self.button_to_pos(&self.curr_button);
        let dest_pos = self.button_to_pos(&dest_button);

        let dx = dest_pos.0 - curr_pos.0;
        let dy = dest_pos.1 - curr_pos.1;

        let x_dir = if dx > 0 { DirectionalButton::BRight } else { DirectionalButton::BLeft };
        let y_dir = if dy > 0 { DirectionalButton::BUp } else { DirectionalButton::BDown };

        let mut vec_x = vec![x_dir; dx.abs() as usize];
        let mut vec_y = vec![y_dir; dy.abs() as usize];

        self.curr_button = dest_button.clone();

        if curr_pos.0 == 0 {
            vec_x.append(&mut vec_y);
            return vec_x
        } else {
            vec_y.append(&mut vec_x);
            return vec_y
        }
    }

    fn button_to_pos(&self, button: &DirectionalButton) -> (isize, isize) {
        match button {
            DirectionalButton::BLeft => (0,0),
            DirectionalButton::BRight => (2,0),
            DirectionalButton::BUp => (1,1),
            DirectionalButton::BDown => (1,0),
            DirectionalButton::BA => (2,1),
        }
    }

    fn pos_to_button(&self, pos: (isize, isize)) -> DirectionalButton {
        match pos {
            (0,0) => DirectionalButton::BLeft,
            (2,0) => DirectionalButton::BRight,
            (1,1) => DirectionalButton::BUp,
            (1,0) => DirectionalButton::BDown,
            _ => DirectionalButton::BA,
        }
    }
}

fn calculate_complexity(code: &str, sequence: &Vec<Vec<DirectionalButton>>) -> usize {
    let sequence_total_len: usize = sequence.iter().map(|v| v.len()).sum();
    let s: String = code.chars().into_iter().filter(|c| c.is_digit(10)).collect();
    let parsed_code_val = s.trim_start_matches('0').parse::<usize>().unwrap();
    println!("{} * {}", parsed_code_val, sequence_total_len);
    return parsed_code_val * sequence_total_len;
}

#[allow(dead_code)]
fn part1(input: &str) {
    let codes = parse_input(input);

    let mut result = 0;
    for code in codes {
        //TODO: optimize sequences
        let mut numpad = NumericKeypad{curr_button: NumericButton::BA};
        let mut dirpad0 = DirectionalKeypad{curr_button: DirectionalButton::BA};
        let mut dirpad1 = DirectionalKeypad{curr_button: DirectionalButton::BA};
        let directions = numpad.execute_code(code);
        let directions = dirpad0.execute_sequences(&directions);
        let directions = dirpad1.execute_sequences(&directions);
        let complexity = calculate_complexity(code, &directions);
        println!("{}", complexity);
        result += complexity;

    }

    //println!("{}", maze);
    //maze.remove_dead_ends();
    //maze.fill_path_score(0, maze.start_pos, EAST_ID);

    //let result = maze.get_min_path_score();

    println!("Part1: {}", result);
}


#[allow(dead_code)]
fn part2(input: &str) {
}

fn parse_input(input: &str) -> Vec<&str> {
    let mut codes = vec![];
    for line in input.lines() {
        codes.push(line);
    }
    return codes;
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        write!(f, "{}", c)
    }
}

impl fmt::Display for DirectionalButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            DirectionalButton::BUp => '^',
            DirectionalButton::BDown => 'v',
            DirectionalButton::BLeft => '<',
            DirectionalButton::BRight => '>',
            DirectionalButton::BA => 'A',
        };
        write!(f, "{}", c)
    }
}
