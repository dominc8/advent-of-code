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

#[derive(Clone)]
struct Region {
    id: char,
    points: Vec<(usize, usize)>
}

impl Region {
    fn is_new_point_valid(&self, id: char, point: (usize, usize)) -> bool {
        if self.id != id { return false };
        match self.points.iter()
            .find(|&&p| (p.0.abs_diff(point.0).pow(2) + p.1.abs_diff(point.1).pow(2)) == 1) {
                Some(_) => true,
                None => false,
        }
    }

    fn can_merge_regions(&self, other: &Region) -> bool {
        other.points.iter().find(|&&p| self.is_new_point_valid(other.id, p)).is_some()
    }

    fn calculate_perimeter(&self) -> usize {
        let mut perimeter = self.points.len() * 4;
        for (id0, p0) in self.points.iter().enumerate() {
            for (id1, p1) in self.points.iter().enumerate() {
                if id0 != id1 {
                    if (p0.0.abs_diff(p1.0).pow(2) + p0.1.abs_diff(p1.1).pow(2)) == 1 {
                        perimeter -= 1;
                    }
                }
            }
        }
        return perimeter;
    }

    fn calculate_area(&self) -> usize {
        return self.points.len();
    }

    fn calculate_price(&self) -> usize {
        return self.calculate_area() * self.calculate_perimeter();
    }
}

struct RegionMap {
    full_map: Vec<Vec<char>>,
    regions: Vec<Region>
}

impl RegionMap {
    fn calculate_total_price(&self) -> usize {
        self.regions.iter().map(|r| r.calculate_price()).sum()
    }
}

#[allow(dead_code)]
fn part1(input: &str) {
    let region_map = parse_regionmap(input);
    //println!("{}", region_map);
    //for r in region_map.regions.iter() {
    //    println!("{}", r);
    //}
    let result = region_map.calculate_total_price();

    println!("Part1: {}", result);
}

#[allow(dead_code)]
fn part2(input: &str) {
    let result = 0;
    println!("Part2: {}", result);
}

fn parse_regionmap(input: &str) -> RegionMap {
    let full_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut regions: Vec<Region> = vec![];

    for (y, row) in full_map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            regions.push(Region{id: c, points: vec![(x, y)]});
        }
    }

    // slow af
    let mut regions_to_merge: Option<(usize, usize)> = None;
    loop {
        'outer: for (id0, r0) in regions.iter().enumerate() {
            for (id1, r1) in regions.iter().enumerate() {
                if id0 != id1 && r0.can_merge_regions(r1) {
                    regions_to_merge = Some((id0, id1));
                    break 'outer;
                }
            }
        }
        match regions_to_merge {
            None => {
                //println!("no regions to merge");
                break;
            },
            Some((id0, id1)) => {
                //println!("merging regions {} and {}", id0, id1);
                let mut r1 = regions.get(id1).unwrap().clone();
                regions.get_mut(id0).unwrap().points.append(&mut r1.points);
                regions.swap_remove(id1);
            }
        }
        regions_to_merge = None;
    }


    return RegionMap{full_map, regions};
}

impl fmt::Display for RegionMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.full_map.iter() {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self.id)?;
        for p in self.points.iter() {
            write!(f, "{:?}, ", p)?;
        }
        writeln!(f, "")?;
        Ok(())
    }
}
