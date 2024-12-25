use std::env;
use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

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
struct Computer {
    name: String,
    connected: Vec<String>
}

struct Computers(Vec<Computer>);

impl Computers {
    fn create_add_connection(&mut self, name0: &str, name1: &str) {
        if let Some(comp) = self.0.iter_mut().find(|comp| comp.name == name0) {
            comp.connected.push(name1.to_string());
        } else {
            self.0.push(Computer { name: name0.to_string(), connected: vec![name1.to_string()] });
        }

        if let Some(comp) = self.0.iter_mut().find(|comp| comp.name == name1) {
            comp.connected.push(name0.to_string());
        } else {
            self.0.push(Computer { name: name1.to_string(), connected: vec![name0.to_string()] });
        }
    }

    fn sort_connected_vectors(&mut self) {
        for computer in self.0.iter_mut() {
            computer.connected.sort();
        }
    }

    fn are_connected(&self, name0: &str, name1: &str) -> bool {
        if let Some(comp0) = self.0.iter().find(|comp| comp.name == name0) {
            if comp0.connected.iter().find(|name| *name == name1).is_some() {
                return true;
            }
        }
        return false;
    }

    fn get_all_three_connected_computers_names(&self) -> Vec<(String, String, String)> {
        let mut v = vec![];
        for computer in self.0.iter() {
            if computer.connected.len() > 2 {
                for computer_pair in computer.connected.iter().combinations(2) {
                    if self.are_connected(computer_pair[0], computer_pair[1]) {
                        let mut names = vec![computer.name.clone(), computer_pair[0].clone(), computer_pair[1].clone()];
                        names.sort();
                        let name2 = names.pop().unwrap();
                        let name1 = names.pop().unwrap();
                        let name0 = names.pop().unwrap();
                        v.push((name0, name1, name2));
                    }
                }
            }
        }
        v.sort();
        v.dedup();
        return v;
    }

    fn get_all_three_connected_computers_names_part1(&self) -> Vec<(String, String, String)> {
        let v = self.get_all_three_connected_computers_names();
        v.iter()
            .filter(|(name0, name1, name2)| name0.starts_with('t') || name1.starts_with('t') || name2.starts_with('t'))
            .map(|(name0, name1, name2)| (name0.clone(), name1.clone(), name2.clone()))
            .collect()
    }

    fn part2(&mut self) {
        for computer in self.0.iter().filter(|comp| comp.name.starts_with('t')) {
            let mut temp_computers = Computers(vec![computer.clone()]);
            println!("-------------------------------------------------------");
            for c_name in computer.connected.iter() {
                if let Some(c) = self.0.iter().find(|comp| comp.name == *c_name) {
                    let mut temp_c = c.clone();
                    let mut temp_c_new_connected = vec![c.name.clone()];
                    for n in temp_c.connected.iter() {
                        if computer.connected.iter().find(|name| *name == n).is_some() {
                            temp_c_new_connected.push(n.clone());
                        }
                    }
                    temp_c_new_connected.sort();
                    temp_c.connected = temp_c_new_connected;
                    temp_computers.0.push(temp_c);
                }
            }
            for c in temp_computers.0.iter() {
                println!("{}: {:?}", c.name, c.connected);
            }
            println!("-------------------------------------------------------");
        }
    }
}

#[allow(dead_code)]
fn part1(input: &str) {
    let computers = parse_input(input);
    let part1_connections = computers.get_all_three_connected_computers_names_part1();

    let result = part1_connections.len();
    println!("Part1: {}", result);
}


#[allow(dead_code)]
fn part2(input: &str) {
    let mut computers = parse_input(input);
    computers.sort_connected_vectors();

    // It is not that hard to eyeball it tbh
    computers.part2();
}

fn parse_input(input: &str) -> Computers {
    let mut computers = Computers(vec![]);
    for line in input.lines() {
        let (name0, name1) = line.split_once('-').unwrap();

        computers.create_add_connection(name0, name1);
    }
    return computers;
}
