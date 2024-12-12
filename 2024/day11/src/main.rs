use std::env;
use std::fmt;
use std::fs;
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
struct Stones {
    values_str: String,
}

// it just works
fn get_stones_count(stone_str: &str, n_iter: u32) -> usize {
    if n_iter == 0 { return 1 };

    match stone_str {
        "0" => match n_iter {
                0..3 => 1,
                3 => 2,
                _ => get_stones_count("0", n_iter - 4) + 2*get_stones_count("2", n_iter - 4) + get_stones_count("4", n_iter - 4),
            },
        "1" => match n_iter {
                0..2 => 1,
                2 => 2,
                _ => get_stones_count("0", n_iter - 3) + 2*get_stones_count("2", n_iter - 3) + get_stones_count("4", n_iter - 3),
            },
        "2" => match n_iter {
                0..2 => 1,
                2 => 2,
                _ => get_stones_count("0", n_iter - 3) + 2*get_stones_count("4", n_iter - 3) + get_stones_count("8", n_iter - 3),
            },
        "3" => match n_iter {
                0..2 => 1,
                2 => 2,
                _ => get_stones_count("0", n_iter - 3) + get_stones_count("2", n_iter - 3) + get_stones_count("6", n_iter - 3) + get_stones_count("7", n_iter - 3),
            },
        "4" => match n_iter {
                0..2 => 1,
                2 => 2,
                _ => get_stones_count("0", n_iter - 3) + get_stones_count("6", n_iter - 3) + get_stones_count("8", n_iter - 3) + get_stones_count("9", n_iter - 3),
            },
        "5" => match n_iter {
                0..3 => 1,
                3 => 2,
                4 => 4,
                _ => 2*get_stones_count("0", n_iter - 5) + 2*get_stones_count("2", n_iter - 5) + get_stones_count("4", n_iter - 5) + 3*get_stones_count("8", n_iter - 5),
            },
        "6" => match n_iter {
                0..3 => 1,
                3 => 2,
                4 => 4,
                _ => get_stones_count("2", n_iter - 5) + 2*get_stones_count("4", n_iter - 5) + 2*get_stones_count("5", n_iter - 5) + get_stones_count("6", n_iter - 5) + get_stones_count("7", n_iter - 5) + get_stones_count("9", n_iter - 5),
            },
        "7" => match n_iter {
                0..3 => 1,
                3 => 2,
                4 => 4,
                _ => get_stones_count("0", n_iter - 5) + 2*get_stones_count("2", n_iter - 5) + get_stones_count("3", n_iter - 5) + 2*get_stones_count("6", n_iter - 5) + get_stones_count("7", n_iter - 5) + get_stones_count("8", n_iter - 5),
            },
        "8" => match n_iter {
                0..3 => 1,
                3 => 2,
                4 => 4,
                _ => get_stones_count("8", n_iter - 4) + 2*get_stones_count("2", n_iter - 5) + get_stones_count("3", n_iter - 5) + get_stones_count("6", n_iter - 5) + 2*get_stones_count("7", n_iter - 5),
            },
        "9" => match n_iter {
                0..3 => 1,
                3 => 2,
                4 => 4,
                _ => get_stones_count("1", n_iter - 5) + get_stones_count("3", n_iter - 5) + get_stones_count("4", n_iter - 5) + 2*get_stones_count("6", n_iter - 5) + 2*get_stones_count("8", n_iter - 5) + get_stones_count("9", n_iter - 5),
            },
         _ => {
             if stone_str.len() % 2 == 0 {
                 let (s0, s1) = stone_str.split_at(stone_str.len() / 2);
                 get_stones_count(s0, n_iter - 1) + get_stones_count(s1, n_iter - 1)
             } else {
                 get_stones_count((stone_str.parse::<usize>().unwrap() * 2024).to_string().as_str(), n_iter - 1)
             }
         }
    }
}

fn part2_init_hashmap(n_iter: u32) -> HashMap<String, Vec<usize>> {
    let mut hashmap: HashMap<String, Vec<usize>> = HashMap::new();

    for value in 0..10 {
        let key = value.to_string();
        let mut values_str_vec: Vec<usize> = vec![];
        for i in 0..n_iter {
            values_str_vec.push(get_stones_count(&key, i));
        }
        hashmap.insert(key, values_str_vec);
    }

    return hashmap;
}

impl Stones {
    fn blink(&mut self) {
        let new_values_str: Option<String> =
            self.values_str.split_ascii_whitespace().map(|num_str| {
                match num_str {
                    "0" => String::from("1"), 
                     _ => {
                         if num_str.len() % 2 == 0 {
                             let (s0, s1) = num_str.split_at(num_str.len() / 2);
                             let (s0, s1) = ( s0.parse::<usize>().unwrap().to_string(), s1.parse::<usize>().unwrap().to_string());
                             s0 + " " + &s1
                         } else {
                             (num_str.parse::<usize>().unwrap() * 2024).to_string()
                         }
                     }
                }
            }).reduce(|acc, e| { acc + " " + &e} );
        self.values_str = new_values_str.unwrap();
    }

    fn blink_n(&mut self, n: u32) {
        for _ in 0..n { self.blink(); }
    }

    fn count_blink_part2(&mut self, n: usize, hashmap: &HashMap<String, Vec<usize>>) -> usize {
        if n == 0 { return self.count_stones(); }

        let mut count = 0;
        self.blink();
        for str in self.values_str.split_ascii_whitespace() {
            if n < 50 && hashmap.contains_key(str) {
                count += hashmap.get(str).unwrap().get(n - 1).unwrap();
            } else {
                let mut temp = Stones{values_str: str.to_string()};
                count += temp.count_blink_part2(n - 1, &hashmap);
            }
        }
        return count;
    }


    fn part2(&mut self, n: usize, hashmap: &HashMap<String, Vec<usize>>) -> usize {
        let mut count = 0;

        for str in self.values_str.split_ascii_whitespace() {
            let mut temp = Stones{values_str: str.to_string()};
            count += temp.count_blink_part2(n, &hashmap);
        }

        return count;
    }

    fn count_stones(&self) -> usize {
        return self.values_str.split_ascii_whitespace().count();
    }
}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut stones = parse_stones(input);
    stones.blink_n(25);
    let result = stones.count_stones();

    println!("Part1: {}", result);
}


#[allow(dead_code)]
fn part2(input: &str) {
    let hashmap = part2_init_hashmap(50);

    let mut stones = parse_stones(input);
    let result = stones.part2(75, &hashmap);
    println!("Part2: {}", result);
}

fn parse_stones(input: &str) -> Stones {
    let values_str = String::from(input.lines().next().unwrap());
    return Stones{values_str};
}

impl fmt::Display for Stones {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.values_str)?;
        Ok(())
    }
}
