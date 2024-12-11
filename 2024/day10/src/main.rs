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
    part2(&input);
}

struct TopoMap {
    heights: Vec<Vec<i32>>
}

impl TopoMap {
    fn sum_trailhead_scores(&self, is_part1: bool) -> usize {
        let mut score = 0;
        for (y, row) in self.heights.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if *value == 0 {
                    //println!("checking neighbors of pos=({},{})", x, y);
                    let mut end_positions = vec![];
                    end_positions.append(&mut self.get_trailhead_end_positions(*value, (x as i32 - 1, y as i32)));
                    end_positions.append(&mut self.get_trailhead_end_positions(*value, (x as i32 + 1, y as i32)));
                    end_positions.append(&mut self.get_trailhead_end_positions(*value, (x as i32, y as i32 - 1)));
                    end_positions.append(&mut self.get_trailhead_end_positions(*value, (x as i32, y as i32 + 1)));
                    if is_part1 {
                        end_positions.sort();
                        end_positions.dedup();
                    }
                    let trailhead_score = end_positions.len();
                    //println!("trailhead_score of ({},{}) is {}", x, y, trailhead_score);
                    score += trailhead_score;
                }

            }
        }

        return score;
    }

    fn get_trailhead_end_positions(&self, prev_value: i32, pos: (i32, i32)) -> Vec<(i32, i32)> {
        if pos.0 < 0 || pos.1 < 0 {
            // println!("pos({},{}) is out of bounds", pos.0, pos.1);
            return vec![];
        }
        if self.heights.get(pos.1 as usize).is_none() {
            // println!("pos({},{}) is out of bounds", pos.0, pos.1);
            return vec![];
        };

        if self.heights.get(pos.1 as usize).unwrap().get(pos.0 as usize).is_none() {
            // println!("pos({},{}) is out of bounds", pos.0, pos.1);
            return vec![];
        };

        let x = pos.0 as usize;
        let y = pos.1 as usize;
        let value = *self.heights.get(y).unwrap().get(x).unwrap();
        if value - prev_value != 1 {
            // println!("pos({},{}) value difference is not 1, {}->{}", pos.0, pos.1, prev_value, value);
            return vec![];
        };

        if value == 9 {
            //println!("pos({},{}) value is 9", pos.0, pos.1);
            return vec![pos];
        };

        //println!("pos=({},{}) matches criteria with {}->{}", pos.0, pos.1, prev_value, value);
        let mut end_positions = vec![];
        end_positions.append(&mut self.get_trailhead_end_positions(value, (x as i32 - 1, y as i32)));
        end_positions.append(&mut self.get_trailhead_end_positions(value, (x as i32 + 1, y as i32)));
        end_positions.append(&mut self.get_trailhead_end_positions(value, (x as i32, y as i32 - 1)));
        end_positions.append(&mut self.get_trailhead_end_positions(value, (x as i32, y as i32 + 1)));
        return end_positions;
    }

}

#[allow(dead_code)]
fn part1(input: &str) {
    let topomap = parse_topomap(input);
    let is_part1 = true;
    //println!("{}", topomap);
    let result = topomap.sum_trailhead_scores(is_part1);

    println!("Part1: {}", result);
}


#[allow(dead_code)]
fn part2(input: &str) {
    let topomap = parse_topomap(input);
    let is_part1 = false;
    //println!("{}", topomap);
    let result = topomap.sum_trailhead_scores(is_part1);

    println!("Part2: {}", result);
}

fn parse_topomap(input: &str) -> TopoMap {
    let mut heights = vec![];

    for line in input.lines() {
        heights.push(line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect());
    }

    return TopoMap{heights};
}

impl fmt::Display for TopoMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.heights {
            for v in row {
                write!(f, "{}", v)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

