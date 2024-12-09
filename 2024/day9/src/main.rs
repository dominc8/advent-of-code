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

struct Disk {
    file_blocks: Vec<Option<u32>>,
}

#[derive(Debug)]
struct FileBlock {
    start_idx: usize,
    len: usize
}

impl Disk {
    fn compress(&mut self) {
        let mut head_index = 0;
        let mut tail_index = self.file_blocks.len() - 1;
        while head_index < tail_index {
            if self.file_blocks.get(head_index).unwrap().is_some() {
                head_index += 1;
            } else if self.file_blocks.get(tail_index).unwrap().is_none() {
                tail_index -= 1;
            } else {
                self.file_blocks.swap(head_index, tail_index);
            }
        }
    }

    fn compress_part2(&mut self) {
        let mut search_start_idx = (self.file_blocks.len() - 1) as isize;
        loop {
            match self.find_prev_file_block(search_start_idx as usize) {
                Some(fb) => {
                    //println!("Found taken fileblock: {:?}", fb);
                    search_start_idx = fb.start_idx as isize - 1;
                    if let Some(empty_fb) = self.find_next_empty_block_with_length(0, fb.len) {
                        if empty_fb.start_idx > fb.start_idx { continue }
                        //println!("Found empty block: {:?}", empty_fb);
                        let block_copy = self.file_blocks.get(fb.start_idx).unwrap().clone();
                        self.file_blocks.get_mut(empty_fb.start_idx..(empty_fb.start_idx + fb.len)).unwrap().fill(block_copy);
                        self.file_blocks.get_mut(fb.start_idx..(fb.start_idx + fb.len)).unwrap().fill(None);
                        //println!("{}", self);
                    }
                },
                None => break,
            }
        }
    }

    fn find_prev_file_block(&self, search_start_idx: usize) -> Option<FileBlock> {
        let mut block_last_idx = -1;
        let mut idx = search_start_idx as isize;
        let mut id = 0;
        while idx >= 0 {
            if let Some(block_id) = self.file_blocks.get(idx as usize).unwrap() {
                block_last_idx = idx;
                id = *block_id;
                break;
            }
            idx -= 1;
        }
        if block_last_idx < 0 { return None };

        while idx >= 0 {
            match self.file_blocks.get(idx as usize).unwrap() {
                Some(block_id) => {
                    if *block_id != id { break };
                },
                None => break,
            }
            idx -= 1;
        }
        let block_first_idx = idx + 1;

        let start_idx = block_first_idx as usize;
        let len = (block_last_idx - block_first_idx + 1) as usize;
        return Some(FileBlock{start_idx, len});
    }

    fn find_next_empty_block_with_length(&self, search_start_idx: usize, min_size: usize) -> Option<FileBlock> {
        let mut search_start_idx = search_start_idx;
        loop {
            match self.find_next_empty_block(search_start_idx) {
                Some(fb) => {
                    if fb.len >= min_size {
                        return Some(fb);
                    }
                    search_start_idx = fb.start_idx + fb.len;
                },
                None => return None,
            }
        }
    }

    fn find_next_empty_block(&self, search_start_idx: usize) -> Option<FileBlock> {
        let mut block_first_idx = -1;
        let mut idx = search_start_idx as isize;
        while idx < self.file_blocks.len() as isize {
            if self.file_blocks.get(idx as usize).unwrap().is_none() {
                block_first_idx = idx;
                break;
            }
            idx += 1;
        }

        if block_first_idx < 0 { return None };

        while idx < self.file_blocks.len() as isize {
            if self.file_blocks.get(idx as usize).unwrap().is_some() {
                break;
            }
            idx += 1;
        }
        let block_last_idx = idx - 1;

        let start_idx = block_first_idx as usize;
        let len = (block_last_idx - block_first_idx + 1) as usize;
        return Some(FileBlock{start_idx, len});
    }

    fn checksum(&self) -> usize {
        self.file_blocks.iter().enumerate().map(|(idx, id)| idx * (id.unwrap_or_default() as usize)).sum()
    }
}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut disk = parse_disk(input);
    //println!("{}", disk);
    disk.compress();
    //println!("{}", disk);
    let result = disk.checksum();

    println!("Part1: {}", result);
}


#[allow(dead_code)]
fn part2(input: &str) {
    let mut disk = parse_disk(input);
    //println!("{}", disk);
    disk.compress_part2();
    //println!("{}", disk);
    let result = disk.checksum();

    println!("Part2: {}", result);
}

fn parse_disk(input: &str) -> Disk {
    let mut file_blocks = vec![];

    let mut is_file = true;
    let mut file_id : u32 = 0;

    for c in input.lines().next().unwrap().chars() {
        let file_block_len = c.to_digit(10).unwrap() as usize;
        let mut blocks;
        if is_file {
            blocks = vec![Some(file_id); file_block_len];
            file_id += 1;
        } else {
            blocks = vec![None; file_block_len];
        }
        is_file = !is_file;
        if file_block_len > 0 
        {
            file_blocks.append(&mut blocks);
        }
    }
    return Disk{file_blocks};
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in &self.file_blocks {
            match block {
                Some(value) => write!(f, "{}", value)?,
                None => write!(f, ".")?,
            };
        }
        Ok(())
    }
}

