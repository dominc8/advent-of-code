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

fn count_horizontal_xmas(line: &str, x_pos: usize) -> i32 {
    let count = match line.get(x_pos.saturating_sub(3)..x_pos + 1)
    {
        Some("SAMX") => 1,
        _  => 0,
    };
    let count = count + match line.get(x_pos..x_pos + 4)
    {
        Some("XMAS") => 1,
        _  => 0,
    };
    return count;
}

fn count_vertical_xmas(upper_lines: Option<&[&str]>, lower_lines: Option<&[&str]>, x_pos: usize) -> i32 {
    let mut count = 0;

    if let Some(upper_lines) = upper_lines {
        let mut upper_checks_left = 3;
        let mut iter = upper_lines.iter();
        if let Some(line_s) = iter.next() {
            if line_s.chars().nth(x_pos) == Some('S') {
                upper_checks_left -= 1;
            }
        }
        if let Some(line_a) = iter.next() {
            if line_a.chars().nth(x_pos) == Some('A') {
                upper_checks_left -= 1;
            }
        }
        if let Some(line_m) = iter.next() {
            if line_m.chars().nth(x_pos) == Some('M') {
                upper_checks_left -= 1;
            }
        }
        if upper_checks_left == 0 {
            count += 1;
        }
    }

    if let Some(lower_lines) = lower_lines {
        let mut lower_checks_left = 3;
        let mut iter = lower_lines.iter();
        if let Some(line_m) = iter.next() {
            if line_m.chars().nth(x_pos) == Some('M') {
                lower_checks_left -= 1;
            }
        }
        if let Some(line_a) = iter.next() {
            if line_a.chars().nth(x_pos) == Some('A') {
                lower_checks_left -= 1;
            }
        }
        if let Some(line_s) = iter.next() {
            if line_s.chars().nth(x_pos) == Some('S') {
                lower_checks_left -= 1;
            }
        }
        if lower_checks_left == 0 {
            count += 1;
        }
    }

    return count;
}

fn count_diagonal_xmas(upper_lines: Option<&[&str]>, lower_lines: Option<&[&str]>, x_pos: usize) -> i32 {
    let mut count = 0;

    if let Some(upper_lines) = upper_lines {
        if x_pos >= 3 {
            let mut upper_checks_left = 3;
            let mut iter = upper_lines.iter();
            if let Some(line_s) = iter.next() {
                if line_s.chars().nth(x_pos-3) == Some('S') {
                    upper_checks_left -= 1;
                }
            }
            if let Some(line_a) = iter.next() {
                if line_a.chars().nth(x_pos-2) == Some('A') {
                    upper_checks_left -= 1;
                }
            }
            if let Some(line_m) = iter.next() {
                if line_m.chars().nth(x_pos-1) == Some('M') {
                    upper_checks_left -= 1;
                }
            }
            if upper_checks_left == 0 {
                count += 1;
            }
        }

        let mut upper_checks_left = 3;
        let mut iter = upper_lines.iter();
        if let Some(line_s) = iter.next() {
            if line_s.chars().nth(x_pos+3) == Some('S') {
                upper_checks_left -= 1;
            }
        }
        if let Some(line_a) = iter.next() {
            if line_a.chars().nth(x_pos+2) == Some('A') {
                upper_checks_left -= 1;
            }
        }
        if let Some(line_m) = iter.next() {
            if line_m.chars().nth(x_pos+1) == Some('M') {
                upper_checks_left -= 1;
            }
        }
        if upper_checks_left == 0 {
            count += 1;
        }
    }

    if let Some(lower_lines) = lower_lines {
        if x_pos >= 3 {
            let mut lower_checks_left = 3;
            let mut iter = lower_lines.iter();
            if let Some(line_m) = iter.next() {
                if line_m.chars().nth(x_pos-1) == Some('M') {
                    lower_checks_left -= 1;
                }
            }
            if let Some(line_a) = iter.next() {
                if line_a.chars().nth(x_pos-2) == Some('A') {
                    lower_checks_left -= 1;
                }
            }
            if let Some(line_s) = iter.next() {
                if line_s.chars().nth(x_pos-3) == Some('S') {
                    lower_checks_left -= 1;
                }
            }
            if lower_checks_left == 0 {
                count += 1;
            }
        }

        let mut lower_checks_left = 3;
        let mut iter = lower_lines.iter();
        if let Some(line_m) = iter.next() {
            if line_m.chars().nth(x_pos+1) == Some('M') {
                lower_checks_left -= 1;
            }
        }
        if let Some(line_a) = iter.next() {
            if line_a.chars().nth(x_pos+2) == Some('A') {
                lower_checks_left -= 1;
            }
        }
        if let Some(line_s) = iter.next() {
            if line_s.chars().nth(x_pos+3) == Some('S') {
                lower_checks_left -= 1;
            }
        }
        if lower_checks_left == 0 {
            count += 1;
        }
    }

    return count;
}

fn check_x_mas(lines: Option<&[&str]>, a_pos: usize) -> bool {
    if a_pos == 0 {
        return false;
    }

    if let Some(lines) = lines {
        let mut iter = lines.iter();
        let mut upper_left = None;
        let mut upper_right = None;
        let mut lower_left = None;
        let mut lower_right = None;
        if let Some(line) = iter.next() {
            upper_left = line.chars().nth(a_pos - 1);
            upper_right= line.chars().nth(a_pos + 1);
        }
        iter.next();
        if let Some(line) = iter.next() {
            lower_left = line.chars().nth(a_pos - 1);
            lower_right= line.chars().nth(a_pos + 1);
        }

        let mut checks_left = 2;

        match (upper_left, lower_right) {
            (Some('S'), Some('M')) => checks_left -= 1,
            (Some('M'), Some('S')) => checks_left -= 1,
            _ => ()
        };

        match (lower_left, upper_right) {
            (Some('S'), Some('M')) => checks_left -= 1,
            (Some('M'), Some('S')) => checks_left -= 1,
            _ => ()
        };

        return checks_left == 0;
    }

    return false;
}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut lines : Vec<&str> = vec![];
    for line in input.lines() {
        lines.push(line);
    }

    let mut count = 0;

    for (line_idx, line) in lines.iter().enumerate() {
        for (char_idx, char) in line.chars().enumerate() {
            match char
            {
                'X' => {
                    count += count_horizontal_xmas(line, char_idx);
                    count += count_vertical_xmas(
                                lines.get(line_idx.saturating_sub(3)..line_idx),
                                lines.get(line_idx+1..line_idx+4),
                                char_idx);
                    count += count_diagonal_xmas(
                                lines.get(line_idx.saturating_sub(3)..line_idx),
                                lines.get(line_idx+1..line_idx+4),
                                char_idx);
                },
                _ => {},
            };
        }
    }

    println!("Part1: {}", count);
}

#[allow(dead_code)]
fn part2(input: &str) {
    let mut lines : Vec<&str> = vec![];
    for line in input.lines() {
        lines.push(line);
    }

    let mut count = 0;

    for (line_idx, line) in lines.iter().enumerate() {
        for (char_idx, char) in line.chars().enumerate() {
            match char
            {
                'A' => {
                    if check_x_mas(
                            lines.get(line_idx.saturating_sub(1)..line_idx + 2),
                            char_idx) {
                        count += 1;
                    }
                },
                _ => {},
            };
        }
    }

    println!("Part2: {}", count);
}
