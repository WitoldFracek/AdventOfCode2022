use std::collections::HashSet;
use itertools::Itertools;
use crate::data_utils::read_lines;

pub fn solve() {
    let distinct_markers = 14;
    let lines = read_lines("./src/data/day6.txt");
    let signal: Vec<char> = lines[0].chars().collect();
    let end = signal.len() - distinct_markers;
    for i in 0..end {
        if is_marker(&signal[i..(i+distinct_markers)]) {
            println!("No signal chars: {}", i + distinct_markers);
            break;
        }
    };

}

fn is_marker(input: &[char]) -> bool {
    let unique: Vec<char> = input
        .iter()
        .cloned()
        .unique()
        .collect();
    input.len() == unique.len()
}