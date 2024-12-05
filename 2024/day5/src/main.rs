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

#[allow(dead_code)]
fn part1(input: &str) {
    let mut page_ordering_rules : Vec<Vec<usize>> = vec![];
    for _ in 0..100 {
        page_ordering_rules.push(vec![]);
    }

    let mut lines_iter = input.lines();
    while let Some(rule_str) = lines_iter.next() {
        if rule_str.is_empty() {
            break;
        }
        let mut rule_str_iter = rule_str.split('|');
        let page_before = rule_str_iter.next().unwrap().parse::<usize>().unwrap();
        let page_after = rule_str_iter.next().unwrap().parse::<usize>().unwrap();
        page_ordering_rules.get_mut(page_after).unwrap().push(page_before);
    }

    //println!("page_ordering_rules: {:?}", page_ordering_rules);

    let mut middle_page_number_sum = 0;

    for update_str in lines_iter {
        let mut page_numbers = vec![];
        for page_str in update_str.split(',') {
            page_numbers.push(page_str.parse::<usize>().unwrap())
        }
        //println!("page_numbers: {:?}", page_numbers);
        let middle_page_number = page_numbers[page_numbers.len()/2];
        middle_page_number_sum += middle_page_number;

        for (page_idx, page_number) in page_numbers.iter().enumerate() {
            if vec_slice_have_common(page_ordering_rules.get(*page_number).unwrap(), page_numbers.get(page_idx+1..).unwrap()) {
                //println!("update {} fails on page number {} at idx {}", update_str, page_number, page_idx);
                middle_page_number_sum -= middle_page_number;
                break;
            }
        }
    }

    println!("Part1: {}", middle_page_number_sum);
}

#[allow(dead_code)]
fn part2(input: &str) {
    let mut page_ordering_rules : Vec<Vec<usize>> = vec![];
    for _ in 0..100 {
        page_ordering_rules.push(vec![]);
    }

    let mut lines_iter = input.lines();
    while let Some(rule_str) = lines_iter.next() {
        if rule_str.is_empty() {
            break;
        }
        let mut rule_str_iter = rule_str.split('|');
        let page_before = rule_str_iter.next().unwrap().parse::<usize>().unwrap();
        let page_after = rule_str_iter.next().unwrap().parse::<usize>().unwrap();
        page_ordering_rules.get_mut(page_after).unwrap().push(page_before);
    }

    //println!("page_ordering_rules: {:?}", page_ordering_rules);

    let mut middle_page_number_sum = 0;

    for update_str in lines_iter {
        let mut page_numbers = vec![];
        for page_str in update_str.split(',') {
            page_numbers.push(page_str.parse::<usize>().unwrap())
        }
        //println!("page_numbers: {:?}", page_numbers);
        let mut fix_page_numbers = false;

        for (page_idx, page_number) in page_numbers.iter().enumerate() {
            if vec_slice_have_common(page_ordering_rules.get(*page_number).unwrap(), page_numbers.get(page_idx+1..).unwrap()) {
                //println!("update {} fails on page number {} at idx {}", update_str, page_number, page_idx);
                fix_page_numbers = true;
                break;
            }
        }

        if fix_page_numbers {
            let ordered_page_numbers = order_page_numbers(page_numbers, &page_ordering_rules);

            let middle_page_number = ordered_page_numbers[ordered_page_numbers.len()/2];
            middle_page_number_sum += middle_page_number;
        }
    }

    println!("Part2: {}", middle_page_number_sum);
}

fn vec_slice_have_common(vec: &Vec<usize>, slice: &[usize]) -> bool {
    for v in vec {
        for s in slice {
            if *v == *s { return true };
        }
    }
    return false;
}

fn order_page_numbers(mut page_numbers: Vec<usize>, page_ordering_rules: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut ordered_page_numbers = vec![];
    //println!("Ordering {:?}", page_numbers);

    let loop_count = page_numbers.len();
    for _ in 0..loop_count {
        let mut page_idx_to_move = loop_count;
        for (page_idx, page_number) in page_numbers.iter().enumerate() {
            let pages_that_has_to_be_before = page_ordering_rules.get(*page_number).unwrap();
            if pages_that_has_to_be_before.is_empty() {
                page_idx_to_move = page_idx;
                //println!("{}", page_number);
                break;
            }
            if !vec_slice_have_common(pages_that_has_to_be_before, page_numbers.as_slice()) {
                page_idx_to_move = page_idx;
                //println!("{}", page_number);
                break;
            }
        }
        ordered_page_numbers.push(page_numbers.swap_remove(page_idx_to_move));
    }

    //println!("Ordered {:?}", ordered_page_numbers);
    return ordered_page_numbers;
}
