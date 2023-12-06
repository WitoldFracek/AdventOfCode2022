use std::str::FromStr;
use crate::data_utils::read_lines;

pub fn solve() {
    let lines = read_lines("./src/data/day4.txt");
    let mut intersections = 0;
    let mut overlaps = 0;
    for line in lines {
        let (left, right) = get_sections_string(&line);
        let (start1, end1) = get_range(left);
        let (start2, end2) = get_range(right);
        if is_fully_contained(start1, end1, start2, end2) {
            intersections += 1;
        }
        if does_overlap(start1, end1, start2, end2) {
            overlaps += 1;
        }
    }
    println!("Fully intersected: {}", intersections);
    println!("Ovelaps: {}", overlaps);
}

fn get_sections_string(line: &str) -> (&str, &str) {
    let split: Vec<&str> = line.split(',').collect();
    (split[0], split[1])
}

fn get_range(section: &str) -> (i32, i32) {
    let split: Vec<&str> = section.split('-').collect();
    let start = i32::from_str(split[0]).unwrap();
    let end = i32::from_str(split[1]).unwrap();
    (start, end)
}

fn does_overlap(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    if start1 <= start2 && start2 <= end1 {
        return true;
    }
    if start2 <= start1 && start1 <= end2 {
        return true;
    }
    false
}

fn is_fully_contained(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    if start1 >= start2 && end1 <= end2 {
        return true;
    }
    if start2 >= start1 && end2 <= end1 {
        return true;
    }
    false
}