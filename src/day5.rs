use std::str::FromStr;
use crate::data_utils::read_lines;

pub fn solve() {
    let lines = read_lines("./src/data/day5.txt");
    let data_lines = get_data_lines(&lines);
    let ref_lines: Vec<&String> = lines
        .iter()
        .map(|x| x)
        .collect();
    let mut payload = prepare_payload(&data_lines);

    let instruction_lines = ref_lines[(data_lines.len() + 1)..].to_vec();
    let instructions: Vec<(i32, usize, usize)> = instruction_lines.iter()
        .map(|line| transform_instructions_to_tuples(*line))
        .collect();
    for (how_many, from_stack, to_stack) in instructions {
        payload.single_grasp_modify(how_many, from_stack - 1, to_stack - 1); // task b
        // payload.modify(how_many, from_stack - 1, to_stack - 1); // task a
    }
    println!("Result: {}", payload.get_top());
}

fn get_data_lines(lines: &Vec<String>) -> Vec<&String> {
    let mut ret = Vec::new();
    for line in lines {
        if !line.starts_with("move") && line != "" {
            ret.push(line);
        }
    }
    ret
}

fn prepare_payload(data_liens: &Vec<&String>) -> Payload {
    let last = data_liens.last().unwrap();
    let indices: Vec<i32> = last
        .split(' ')
        .filter(|s| *s != "")
        .map(|inx| i32::from_str(inx).unwrap())
        .collect();
    let rev_data_lines = reverse_crate_lines(data_liens);
    let mut payload = Payload::new(indices.len());
    for line in rev_data_lines {
        fill_crates_for_line(&mut payload, line);
    }
    payload
}

fn reverse_crate_lines<'a>(data_lines: &'a Vec<&String>) -> Vec<&'a String> {
    data_lines[..(data_lines.len() - 1)]
        .iter()
        .rev()
        .map(|s| *s)
        .collect()
}

fn fill_crates_for_line(payload: &mut Payload, data_line: &str) {
    let chars: Vec<char> = data_line.chars().collect();
    for i in 0..payload.len() {
        let index = i * 4 + 1;
        let chr = chars[index];
        if chr != ' ' {
            payload.stacks[i].push(chr);
        }
    }
    println!();
}

fn transform_instructions_to_tuples(instruction: &str) -> (i32, usize, usize) {
    let split: Vec<&str> = instruction.split(' ').collect();
    let how_many = i32::from_str(split[1]).unwrap();
    let from_stack = usize::from_str(split[3]).unwrap();
    let to_stack = usize::from_str(split[5]).unwrap();
    (how_many, from_stack, to_stack)
}

#[derive(Debug)]
struct Stack {
    pub stack: Vec<char>,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, chr: char) {
        self.stack.push(chr)
    }

    pub fn pop(&mut self) -> char {
        self.stack.pop().unwrap()
    }
}

struct Payload {
    pub stacks: Vec<Stack>,
}

impl Payload{
    pub fn new(size: usize) -> Self {
        let mut stacks = Vec::new();
        for i in 0..size {
            stacks.push(Stack::new())
        }
        Self { stacks }
    }

    pub fn len(&self) -> usize {
        self.stacks.len()
    }

    pub fn modify(&mut self, how_many: i32, from_stack: usize, to_stack: usize) {
        for _ in 0..how_many {
            let value = self.stacks[from_stack].pop();
            self.stacks[to_stack].push(value);
        }
    }

    pub fn single_grasp_modify(&mut self, how_many: i32, from_stack: usize, to_stack: usize) {
        let mut buffer = Vec::new();
        for _ in 0..how_many {
            let value = self.stacks[from_stack].pop();
            buffer.push(value);
        }
        for &value in buffer.iter().rev() {
            self.stacks[to_stack].push(value);
        }
    }

    pub fn get_top(&self) -> String {
        let mut ret = "".to_string();
        for stack in &self.stacks {
            ret = format!("{}{}", ret, stack.stack.last().unwrap());
        }
        String::from(ret)
    }
}
