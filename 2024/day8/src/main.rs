use std::env;
use std::fs;

use itertools::Itertools;
use gcd::Gcd;

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
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Antenna {
    frequency_char: char,
    positions: Vec<Pos>,
}

#[derive(Debug)]
struct Map {
    antennas: Vec<Antenna>,
    antinodes: Vec<Vec<bool>>,
    width: i32,
    length: i32,
}

impl Antenna {
    fn calculate_antinodes(&self) -> Vec<Pos> {
        let mut positions = vec![];
        for antenna_pair in self.positions.iter().combinations(2) {
            let ant0 = antenna_pair.get(0).unwrap();
            let ant1 = antenna_pair.get(1).unwrap();

            positions.push(Pos{x: 2 * ant0.x - ant1.x, y: 2 * ant0.y - ant1.y});
            positions.push(Pos{x: 2 * ant1.x - ant0.x, y: 2 * ant1.y - ant0.y});
        }
        return positions;
    }

    fn calculate_antinodes_part2(&self, x_limit: i32, y_limit: i32) -> Vec<Pos> {
        let mut positions = vec![];
        for antenna_pair in self.positions.iter().combinations(2) {
            let ant0 = antenna_pair.get(0).unwrap();
            let ant1 = antenna_pair.get(1).unwrap();

            let (dx, dy) = {
                let dx = ant1.x - ant0.x;
                let dy = ant1.y - ant0.y;
                let dgcd = (dx.abs() as u32).gcd(dy.abs() as u32) as i32;
                (dx / dgcd, dy / dgcd)
            };

            let mut x = ant0.x;
            let mut y = ant0.y;

            while x >= 0 && x < x_limit &&
                    y >= 0 && y < y_limit {
                positions.push(Pos{x, y});
                x -= dx;
                y -= dy;
            }

            x = ant0.x + dx;
            y = ant0.y + dy;

            while x >= 0 && x < x_limit &&
                    y >= 0 && y < y_limit {
                positions.push(Pos{x, y});
                x += dx;
                y += dy;
            }
        }
        return positions;
    }
}

impl Map {
    fn mark_antinodes(&mut self) {
        for ant in self.antennas.iter() {
            for antinode in ant.calculate_antinodes() {
                if antinode.x >= 0 && antinode.x < self.width &&
                    antinode.y >= 0 && antinode.y < self.length {
                        *self.antinodes.get_mut(antinode.y as usize).unwrap().
                            get_mut(antinode.x as usize).unwrap() = true;
                }
            }
        }
    }

    fn mark_antinodes_part2(&mut self) {
        for ant in self.antennas.iter() {
            for antinode in ant.calculate_antinodes_part2(self.width, self.length) {
                if antinode.x >= 0 && antinode.x < self.width &&
                    antinode.y >= 0 && antinode.y < self.length {
                        *self.antinodes.get_mut(antinode.y as usize).unwrap().
                            get_mut(antinode.x as usize).unwrap() = true;
                }
            }
        }
    }

    fn count_antinodes(&self) -> usize {
        self.antinodes.iter().map(|row| row.iter().filter(|x| **x).count()).sum()
    }

    fn print_antinodes(&self) {
        for row in self.antinodes.iter() {
            let s: String = row.iter().map(|b| if *b { '#' } else { '.' }).collect();
            println!("{}", s);
        }
    }

}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut map = parse_map(input);
    map.mark_antinodes();
    //map.print_antinodes();
    let result = map.count_antinodes();

    println!("Part1: {}", result);
}

fn parse_map(input: &str) -> Map {
    let mut antennas: Vec<Antenna> = vec![];
    let mut width = 0;
    let mut length = 0;
    for (y, line) in input.lines().enumerate() {
        length += 1;
        width = line.chars().count() as i32;
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                let mut antenna_entry_exists = false;
                for ant in &mut antennas {
                    if ant.frequency_char == c {
                        ant.positions.push(Pos{x: x as i32,y: y as i32});
                        antenna_entry_exists = true;
                        break;
                    }
                }
                if !antenna_entry_exists {
                    antennas.push(Antenna{frequency_char: c, positions: vec![Pos{x: x as i32,y: y as i32}]})
                }
            }
        }
    }

    let antinodes: Vec<Vec<bool>> = Vec::from_iter((0..length).map(|_| vec![false; width as usize]));

    return Map{antennas, antinodes, width, length};
}


#[allow(dead_code)]
fn part2(input: &str) {
    let mut map = parse_map(input);
    map.mark_antinodes_part2();
    //map.print_antinodes();
    let result = map.count_antinodes();

    println!("Part2: {}", result);
}
