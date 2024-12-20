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

struct Velocity(isize, isize);

const EAST_ID: usize = 0;
const NORTH_ID: usize = 1;
const WEST_ID: usize = 2;
const SOUTH_ID: usize = 3;
const N_DIR: usize = 4;

const ROTATE_COST: isize = 1000;

const DIR_ID_TO_V: [Velocity; N_DIR] = [
    Velocity(1, 0),
    Velocity(0, -1),
    Velocity(-1, 0),
    Velocity(0, 1)
];

#[derive(Clone, Debug)]
enum Tile {
    Space([isize; N_DIR]),
    Wall,
}

struct Maze {
    tiles: Vec<Vec<Tile>>,
    start_pos: (isize, isize),
    end_pos: (isize, isize),
}

impl Maze {
    // start from end_pos and set value for some arbitrary amount of tiles from end_pos to be
    // score from it to the end, if already written write smaller value, maybe it will make it
    // faster so in finding smaller path you can faster drop longer paths
    //fn find_min_path_score(&self) -> usize {
    //    let mut min_score : Option<usize> = None;

    //    let path_tiles = vec![self.start_pos];

    //    self.check_path(&mut min_score, 0, path_tiles.clone(), self.start_pos, (1, 0));
    //    self.check_path(&mut min_score, 1000, path_tiles.clone(), self.start_pos, (0, -1));
    //    self.check_path(&mut min_score, 1000, path_tiles.clone(), self.start_pos, (0, 1));
    //    self.check_path(&mut min_score, 2000, path_tiles.clone(), self.start_pos, (-1, 0));

    //    return min_score.unwrap();
    //}

    fn remove_dead_ends(&mut self) {
        loop {
            let mut dead_ends_positions : Vec<(isize, isize)> = vec![];
            let max_y = self.tiles.len() - 1;
            let max_x = self.tiles.get(0).unwrap().len() - 1;
            for (y, tile_row) in self.tiles.iter().enumerate() {
                if y == 0 && y == max_y {
                    continue;
                }
                for (x, tile) in tile_row.iter().enumerate() {
                    if x == 0 && x == max_x {
                        continue;
                    }
                    if x as isize == self.start_pos.0 && y as isize == self.start_pos.1 {
                        continue;
                    }
                    if x as isize == self.end_pos.0 && y as isize == self.end_pos.1 {
                        continue;
                    }
                    match tile {
                        Tile::Space(_) => {
                            let mut wall_count = 0;
                            if let Tile::Wall = self.tiles.get(y - 1).unwrap().get(x).unwrap() {
                                wall_count += 1;
                            }
                            if let Tile::Wall = self.tiles.get(y).unwrap().get(x - 1).unwrap() {
                                wall_count += 1;
                            }
                            if let Tile::Wall = self.tiles.get(y).unwrap().get(x + 1).unwrap() {
                                wall_count += 1;
                            }
                            if let Tile::Wall = self.tiles.get(y + 1).unwrap().get(x).unwrap() {
                                wall_count += 1;
                            }

                            if wall_count > 2 {
                                dead_ends_positions.push((x as isize, y as isize));
                            }


                        },
                        Tile::Wall => (),
                    }

                }
            }
            if dead_ends_positions.len() == 0 {
                return
            } else {
                //println!("{:?}", dead_ends_positions);
                for pos in dead_ends_positions {
                    *self.tiles.get_mut(pos.1 as usize).unwrap().get_mut(pos.0 as usize).unwrap() = Tile::Wall;
                }
            }
        }
    }

    //fn check_path(&self, min_score: &mut Option<usize>, path_score: usize, path_tiles: Vec<(isize, isize)>, pos: (isize, isize), v: (isize, isize)) {
    //    let path_score = path_score + 1;
    //    let new_pos = (pos.0 + v.0, pos.1 + v.1);
    //    let mut path_tiles = path_tiles;

    //    if path_tiles.contains(&new_pos) {
    //        return;
    //    }

    //    if min_score.is_some_and(|score| score < path_score) {
    //        return;
    //    }

    //    path_tiles.push(new_pos.clone());

    //    //println!("path_score={}, pos={:?}", path_score, new_pos);
    //    if new_pos.0 == self.end_pos.0 && new_pos.1 == self.end_pos.1 {
    //        println!("Found path with score {}", path_score);
    //        *min_score = Some(path_score);
    //        return;
    //    }
    //    match self.tiles.get(new_pos.1 as usize).unwrap().get(new_pos.0 as usize).unwrap() {
    //        Tile::Space => {
    //            let v0 = v;
    //            self.check_path(min_score, path_score, path_tiles.clone(), new_pos, v0);
    //            let v1 = (v.1, v.0);
    //            self.check_path(min_score, path_score + 1000, path_tiles.clone(), new_pos, v1);
    //            let v2 = (-v.1, -v.0);
    //            self.check_path(min_score, path_score + 1000, path_tiles.clone(), new_pos, v2);
    //        },
    //        Tile::Wall => (),
    //    }
    //}

    fn fill_path_score(&mut self, path_score: isize, pos: (isize, isize), dir_id: usize) {
        let path_score = path_score + 1;
        let v = &DIR_ID_TO_V[dir_id];
        let new_pos = (pos.0 + v.0, pos.1 + v.1);

        match self.tiles.get_mut(new_pos.1 as usize).unwrap().get_mut(new_pos.0 as usize).unwrap() {
            Tile::Space(path_scores) => {
                if path_score < path_scores[dir_id] || path_scores[dir_id] < 0 {
                    //println!("({},{}) set step_count to {}", new_pos.1, new_pos.0, step_count);
                    path_scores[dir_id] = path_score;
                    self.fill_path_score(path_score, new_pos, dir_id);

                    let dir_id_rotated_left = (dir_id + 1) % N_DIR;
                    self.fill_path_score(path_score + ROTATE_COST, new_pos, dir_id_rotated_left);

                    let dir_id_rotated_right = (dir_id + N_DIR - 1) % N_DIR;
                    self.fill_path_score(path_score + ROTATE_COST, new_pos, dir_id_rotated_right);
                }
            },
            Tile::Wall => (),
        }
    }

    fn get_min_path_score(&self) -> isize {
        if let Tile::Space(path_scores) = self.tiles.get(self.end_pos.1 as usize).unwrap().get(self.end_pos.0 as usize).unwrap() {
            *Vec::from(path_scores).iter().filter(|&&x| x > 0).min().unwrap()
        } else {
            return -1;
        }
    }

}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut maze = parse_input(input);
    //println!("{}", maze);
    maze.remove_dead_ends();
    maze.fill_path_score(0, maze.start_pos, EAST_ID);

    let result = maze.get_min_path_score();

    println!("Part1: {}", result);
}


#[allow(dead_code)]
fn part2(input: &str) {
}

fn parse_input(input: &str) -> Maze {
    let mut tiles = vec![];
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let mut tile_row = vec![];
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Wall,
                'S' => { start_pos = (x, y); Tile::Space([0, ROTATE_COST, 2*ROTATE_COST, ROTATE_COST]) },
                'E' => { end_pos = (x, y); Tile::Space([-1, -1, -1, -1]) },
                _ => Tile::Space([-1, -1, -1, -1]),
            };
            tile_row.push(tile);
        }
        tiles.push(tile_row);
    }

    let start_pos = (start_pos.0 as isize, start_pos.1 as isize);
    let end_pos = (end_pos.0 as isize, end_pos.1 as isize);

    return Maze{tiles, start_pos, end_pos};
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, tile_row) in self.tiles.iter().enumerate() {
            let s : String = 
                if y as isize == self.end_pos.1 || y as isize == self.start_pos.1 {
                    tile_row.iter().enumerate().map(|(x, t)|
                        if (x as isize, y as isize) == self.start_pos {
                            'S'
                        } else if (x as isize, y as isize) == self.end_pos {
                            'E'
                        }
                        else {
                            match t { 
                                Tile::Space(_) => '.',
                                Tile::Wall => '#',
                            }
                        }
                        ).collect()
                } else {
                    tile_row.iter().map(|t|  match t { Tile::Space(_) => '.', Tile::Wall => '#',}).collect()
                };
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}
