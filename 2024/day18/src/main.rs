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
    //part1(&input);
    part2(&input);
}

#[derive(Clone)]
enum Tile {
    Space(isize),
    Wall,
}

const GRID_SIZE: isize = 71;
const DEFAULT_DIST: isize = GRID_SIZE * GRID_SIZE;
const INPUT_LINES_SIZE: usize = 3450;

struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    //fn remove_dead_ends(&mut self) {
    //    loop {
    //        let mut dead_ends_positions : Vec<(isize, isize)> = vec![];
    //        let max_y = self.tiles.len() - 1;
    //        let max_x = self.tiles.get(0).unwrap().len() - 1;
    //        for (y, tile_row) in self.tiles.iter().enumerate() {
    //            if y == 0 && y == max_y {
    //                continue;
    //            }
    //            for (x, tile) in tile_row.iter().enumerate() {
    //                if x == 0 && x == max_x {
    //                    continue;
    //                }
    //                if x as isize == self.start_pos.0 && y as isize == self.start_pos.1 {
    //                    continue;
    //                }
    //                if x as isize == self.end_pos.0 && y as isize == self.end_pos.1 {
    //                    continue;
    //                }
    //                match tile {
    //                    Tile::Space => {
    //                        let mut wall_count = 0;
    //                        if let Tile::Wall = self.tiles.get(y - 1).unwrap().get(x).unwrap() {
    //                            wall_count += 1;
    //                        }
    //                        if let Tile::Wall = self.tiles.get(y).unwrap().get(x - 1).unwrap() {
    //                            wall_count += 1;
    //                        }
    //                        if let Tile::Wall = self.tiles.get(y).unwrap().get(x + 1).unwrap() {
    //                            wall_count += 1;
    //                        }
    //                        if let Tile::Wall = self.tiles.get(y + 1).unwrap().get(x).unwrap() {
    //                            wall_count += 1;
    //                        }

    //                        if wall_count > 2 {
    //                            dead_ends_positions.push((x as isize, y as isize));
    //                        }


    //                    },
    //                    Tile::Wall => (),
    //                }

    //            }
    //        }
    //        if dead_ends_positions.len() == 0 {
    //            return
    //        } else {
    //            //println!("{:?}", dead_ends_positions);
    //            for pos in dead_ends_positions {
    //                *self.tiles.get_mut(pos.1 as usize).unwrap().get_mut(pos.0 as usize).unwrap() = Tile::Wall;
    //            }
    //        }
    //    }
    //}

    fn fill_step_count(&mut self, step_count: isize, pos: (isize, isize), v: (isize, isize)) {
        let step_count = step_count + 1;
        let new_pos = (pos.0 + v.0, pos.1 + v.1);

        if new_pos.0 < 0 || new_pos.0 >= GRID_SIZE || new_pos.1 < 0 || new_pos.1 >= GRID_SIZE { return };

        match self.tiles.get_mut(new_pos.1 as usize).unwrap().get_mut(new_pos.0 as usize).unwrap() {
            Tile::Space(tile_step_count) => {
                if step_count < *tile_step_count {
                    //println!("({},{}) set step_count to {}", new_pos.1, new_pos.0, step_count);
                    *tile_step_count = step_count;
                    let v0 = v;
                    self.fill_step_count(step_count, new_pos, v0);
                    let v1 = (v.1, v.0);
                    self.fill_step_count(step_count, new_pos, v1);
                    let v2 = (-v.1, -v.0);
                    self.fill_step_count(step_count, new_pos, v2);
                }
            },
            Tile::Wall => (),
        }
    }

}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut grid = parse_input(input, 1024);
    println!("{}", grid);
    grid.fill_step_count(0, (0,0), (1,0));
    grid.fill_step_count(0, (0,0), (0,1));
    println!("{}", grid);

    if let Tile::Space(result) = grid.tiles.get(GRID_SIZE as usize - 1).unwrap().get(GRID_SIZE as usize - 1).unwrap() {
        println!("Part1: {}", result);
    } else {
    }
}


#[allow(dead_code)]
fn part2(input: &str) {
    let mut lt_lines_to_parse = 0;
    let mut gt_lines_to_parse = INPUT_LINES_SIZE;
    let mut n_lines_to_parse = INPUT_LINES_SIZE / 2;
    loop {
        let mut grid = parse_input(input, n_lines_to_parse);
        grid.fill_step_count(0, (0,0), (1,0));
        grid.fill_step_count(0, (0,0), (0,1));

        if let Tile::Space(dist) = grid.tiles.get(GRID_SIZE as usize - 1).unwrap().get(GRID_SIZE as usize - 1).unwrap() {
            if *dist < DEFAULT_DIST {
                println!("Part2: There is path ({}) to ({}, {}) after {} parsed lines", dist, GRID_SIZE - 1, GRID_SIZE - 1, n_lines_to_parse);
                lt_lines_to_parse = n_lines_to_parse;
                n_lines_to_parse = (n_lines_to_parse + gt_lines_to_parse) / 2;
            } else {
                println!("Part2: There is no path to ({}, {}) after {} parsed lines", GRID_SIZE - 1, GRID_SIZE - 1, n_lines_to_parse);
                gt_lines_to_parse = n_lines_to_parse;
                n_lines_to_parse = (n_lines_to_parse + lt_lines_to_parse) / 2;
                if n_lines_to_parse == lt_lines_to_parse {
                    let result = input.lines().take(gt_lines_to_parse).last().unwrap();
                    println!("Part2: {}", result);
                    break;
                }
            }

        } else {
            println!("Part2: There is wall at ({}, {})", GRID_SIZE - 1, GRID_SIZE - 1);
        }
    }
}

fn parse_input(input: &str, line_count: usize) -> Grid {
    let mut tiles = vec![vec![Tile::Space(DEFAULT_DIST); GRID_SIZE as usize]; GRID_SIZE as usize];
    for line in input.lines().take(line_count) {
        if let Some((str_x, str_y)) = line.split_once(',') {
            let x = str_x.parse::<isize>().unwrap();
            let y = str_y.parse::<isize>().unwrap();
            *tiles.get_mut(y as usize).unwrap().get_mut(x as usize).unwrap() = Tile::Wall;
        }
    }
    *tiles.get_mut(0).unwrap().get_mut(0).unwrap() = Tile::Space(0);

    return Grid{tiles};
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_, tile_row) in self.tiles.iter().enumerate() {
            let s : String = 
                    tile_row.iter().map(|t|  match t { Tile::Space(_) => '.', Tile::Wall => '#',}).collect();
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}
