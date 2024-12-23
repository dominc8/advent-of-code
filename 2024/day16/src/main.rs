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

    fn check_path_backwards(&self, path_score: isize, pos: (isize, isize), dir_id: usize, curr_path: &mut Vec<(isize, isize)>, all_paths: &mut Vec<Vec<(isize, isize)>>) {
        let path_score = path_score - 1;
        let opposite_dir_id = (dir_id + N_DIR/2) % N_DIR;

        let v = &DIR_ID_TO_V[opposite_dir_id];
        let new_pos = (pos.0 + v.0, pos.1 + v.1);

        if new_pos.0 == self.start_pos.0 && new_pos.1 == self.start_pos.1 {
            all_paths.push(curr_path.clone());
            return;
        }

        curr_path.push(new_pos);

        match self.tiles.get(new_pos.1 as usize).unwrap().get(new_pos.0 as usize).unwrap() {
            Tile::Space(path_scores) => {
                if path_score == path_scores[dir_id] {
                    self.check_path_backwards(path_score, new_pos, dir_id, &mut curr_path.clone(), all_paths);
                }

                let dir_id_rotated_left = (dir_id + 1) % N_DIR;
                if path_score - ROTATE_COST == path_scores[dir_id_rotated_left] {
                    self.check_path_backwards(path_score - ROTATE_COST, new_pos, dir_id_rotated_left, &mut curr_path.clone(), all_paths);
                }

                let dir_id_rotated_right = (dir_id + N_DIR - 1) % N_DIR;
                if path_score - ROTATE_COST == path_scores[dir_id_rotated_right] {
                    self.check_path_backwards(path_score - ROTATE_COST, new_pos, dir_id_rotated_right, &mut curr_path.clone(), all_paths);
                }
            },
            Tile::Wall => (),
        }

    }

    fn get_all_best_paths(&self) -> Vec<Vec<(isize, isize)>> {
        let mut all_paths = vec![];

        let min_path_score = self.get_min_path_score();
        let dir_id = NORTH_ID;

        let mut curr_path = vec![];
        self.check_path_backwards(min_path_score, self.end_pos, dir_id, &mut curr_path, &mut all_paths);

        return all_paths;
    }

    fn count_all_unique_tiles_on_best_paths(&self) -> usize {
        let all_paths = self.get_all_best_paths();

        let mut all_tiles = vec![self.start_pos, self.end_pos];

        for path in all_paths {
            all_tiles.append(&mut path.clone());
        }
        all_tiles.sort();
        all_tiles.dedup();
        return all_tiles.len();
    }

    fn print_all_best_paths(&self) {
        let all_paths = self.get_all_best_paths();

        for (i, path) in all_paths.iter().enumerate() {
            println!("Path #{} of length: {}", i, path.len());
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
                                if path.contains(&(x as isize, y as isize)) {
                                    'O'
                                } else {
                                    match t { 
                                        Tile::Space(_) => '.',
                                        Tile::Wall => '#',
                                    }
                                }
                            }
                            ).collect()
                    } else {
                        tile_row.iter().enumerate().map(|(x, t)|
                            if path.contains(&(x as isize, y as isize)) {
                                'O'
                            } else {
                                match t { Tile::Space(_) => '.', Tile::Wall => '#',}
                            }).collect()
                    };
                println!("{}", s);
            }
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
    let mut maze = parse_input(input);
    //println!("{}", maze);
    maze.remove_dead_ends();
    maze.fill_path_score(0, maze.start_pos, EAST_ID);

    //maze.print_all_best_paths();
    let result = maze.count_all_unique_tiles_on_best_paths();

    println!("Part2: {}", result);
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
