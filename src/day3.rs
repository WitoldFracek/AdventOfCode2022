use std::cmp::max;
use std::collections::HashSet;
use crate::data_utils::read_lines;
use itertools::Itertools;

pub fn solve() {
    let lines = read_lines("./src/data/day3.txt");
    let sacks: Vec<(&str, &str)> = lines.iter().map(split_to_sacks).collect();
    let char_sacks: Vec<(Vec<char>, Vec<char>)> = sacks.iter().map(|(left, right)| (str_to_char_vec(left), str_to_char_vec(right))).collect();
    let repetitions: Vec<char> = char_sacks.iter().map(|(left, right)| find_repetition(left, right)).collect();
    let score: usize = repetitions.iter().map(|c| c.index() + 1).sum();
    println!("Score: {}", score);
}

pub fn solve_b() {
    let lines = read_lines("./src/data/day3.txt");
    let triples = sacks_to_triples(&lines);
    let repetitions: Vec<char> = triples.iter().map(|vec| find_multi_repetitions(vec)).collect();
    let score: usize = repetitions.iter().map(|c| c.index() + 1).sum();
    println!("Score: {}", score);
}

fn split_to_sacks(arg: &String) -> (&str, &str) {
    let len = arg.len();
    let left = &arg[0..(len / 2)];
    let right = &arg[(len / 2)..];
    (left, right)
}

fn str_to_char_vec(arg: &str) -> Vec<char> {
    arg.chars().map(|c| c).collect()
}

fn find_repetition(left: &[char], right: &[char]) -> char {
    let mut histogram = [0; 52];
    for chr in left {
        histogram[chr.index()] += 1;
    }
    for chr in right {
        if histogram[chr.index()] != 0 {
            return *chr;
        }
    }
    panic!("No char repetitions found!");
}

fn sacks_to_triples(lines: &Vec<String>) -> Vec<Vec<Vec<char>>> {
    let mut vec: Vec<Vec<char>> = vec![];
    let mut ret = vec![];
    for line in lines {
        vec.push(line.chars().unique().collect());
        if vec.len() == 3 {;
            ret.push(vec.to_vec());
            vec.clear();
        }
    }
    ret
}

fn find_multi_repetitions(args: &[Vec<char>]) -> char {
    let mut histogram = [0; 52];
    let count = args.len();
    for vec in args {
        for chr in vec {
            histogram[chr.index()] += 1;
        }
    }
    for (i, &elem) in histogram.iter().enumerate() {
        if elem == count {
            return i.to_char();
        }
    }
    panic!("No repetition found!");
}

trait Histo {
    fn index(&self) -> usize;
}

impl Histo for char {
    fn index(&self) -> usize {
        if !self.is_alphabetic() {
            panic!("Cannot find index for char '{}'", self);
        }
        let num = *self as usize;
        if self.is_uppercase() {
            num - 39
        } else {
            num - 97
        }
    }
}

trait HistoRev {
    fn to_char(&self) -> char;
}

impl HistoRev for usize {
    fn to_char(&self) -> char {
        match self {
            0..=25 => char::from_u32((self + 97) as u32).unwrap(),
            26..=51 => char::from_u32((self + 39) as u32).unwrap(),
            _ => panic!("Invalid usize value for char!")
        }
    }
}


