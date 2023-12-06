use std::fs::{File, read};
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use crate::data_utils::read_lines;

pub fn solve() {
    let n = 3;
    let lines = read_lines("./src/data/day1.txt");
    let packs = make_packs(&lines);
    let calories = get_elves_calories(&packs);
    let top = top_calories(n, &calories);
    println!("Max calories for top {}: {}", n, top.iter().sum::<i32>());
}

fn make_packs(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut pack = vec![];
    for line in lines {
        if line == "" {
            ret.push(pack.to_vec());
            pack.clear();
        }
        else {
            pack.push(i32::from_str(&line).unwrap());
        }
    }
    if !pack.is_empty() {
        ret.push(pack.to_vec());
    }
    ret
}

fn get_elves_calories(packs: &Vec<Vec<i32>>) -> Vec<i32> {
    packs.iter().map(|v| v.iter().sum()).collect()
}

fn top_calories(n: usize, calories: &Vec<i32>) -> Vec<i32> {
    let mut inner = calories.to_vec();
    inner.sort();
    let res: Vec<i32> = inner.iter().rev().map(|x| *x).collect();
    res[0..n].to_vec()
}

