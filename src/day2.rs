use std::str::FromStr;
use crate::data_utils::read_lines;

pub fn solve_a() {
    let lines = read_lines("./src/data/day2.txt");
    let encoding = to_numbers_vector(&lines);
    let scores: Vec<usize> = encoding.iter().map(|arg| get_score(arg)).collect();
   println!("Score: {}", scores.iter().sum::<usize>());
}

pub fn solve_b() {
    let lines = read_lines("./src/data/day2.txt");
    let scores: Vec<usize> = to_char_vec(&lines).iter().map(|(left, right)| strategy_outcome(left, right)).collect();
    println!("Score: {}", scores.iter().sum::<usize>());
}

fn points_from_symbol(symbol: &str) -> usize {
    if symbol == "A" {
        return 1;
    }
    if symbol == "B" {
        return 2;
    }
    if symbol == "C" {
        return 3;
    }
    panic!();
}

fn loose_to(arg: &str) -> String {
    if arg == "A" {
        return "C".to_string();
    }
    if arg == "B" {
        return "A".to_string();
    }
    if arg == "C" {
        return "B".to_string();
    }
    panic!();
}

fn win_to(arg: &str) -> String {
    if arg == "A" {
        return "B".to_string();
    }
    if arg == "B" {
        return "C".to_string();
    }
    if arg == "C" {
        return "A".to_string();
    }
    panic!();
}

fn strategy_outcome(left: &str, right: &str) -> usize {
    if right == "X" {
        return 0 + points_from_symbol(&loose_to(&left));
    }
    if right == "Y" {
        return 3 + points_from_symbol(&left);
    }
    if right == "Z" {
        return 6 + points_from_symbol(&win_to(&left));
    }
    panic!();
}

fn to_numbers_vector(lines: &Vec<String>) -> Vec<(usize, usize)> {
    let mut str_vec = vec![];
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        str_vec.push((split[0], split[1]));
    }
    str_vec.iter().map(|(left, right)| to_num_pairs(left, right)).collect()
}

fn to_char_vec(lines: &Vec<String>) -> Vec<(&str, &str)> {
    let mut str_vec = vec![];
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        str_vec.push((split[0], split[1]));
    }
    str_vec
}

fn to_num_pairs(left: &str, right: &str) -> (usize, usize) {
    (assign_number(&left.to_lowercase()), assign_number(&right.to_lowercase()))
}

fn assign_number(arg: &str) -> usize {
    if arg == "a" || arg == "x" {
        return 0;
    }
    if arg == "b" || arg == "y" {
        return 1;
    }
    if arg == "c" || arg == "z" {
        return 2;
    }
    panic!();
}

fn get_score(duel: &(usize, usize)) -> usize {
    let &(left, right) = duel;
    if (left + 1) % 3 == right {
        return 6 + right + 1;
    }
    if left == right {
        return 3 + right + 1;
    }
    right + 1
}
