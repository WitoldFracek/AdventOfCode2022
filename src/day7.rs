use std::collections::HashMap;
use std::str::FromStr;
use crate::data_utils::read_lines;

pub fn solve() {
    let lines = read_lines("./src/data/day7.txt");
    calculate_size_for_dir(&lines);
}

fn is_command(arg: &str) -> bool{
    arg.starts_with("$")
}

fn get_command(command: &str) -> &str {
    let split: Vec<&str> = command.split(' ').collect();
    split[1]
}

fn cd_to(command: &str) -> &str {
    let split: Vec<&str> = command.split(' ').collect();
    split[2]
}

fn is_dir(arg: &str) -> bool {
    arg.starts_with("dir")
}

fn get_dir_name(arg: &str) -> &str {
    let split: Vec<&str> = arg.split(' ').collect();
    split[1]
}

fn get_file_size(name: &str) -> usize {
    let split: Vec<&str> = name.split(' ').collect();
    usize::from_str(split[0]).unwrap()
}

fn calculate_size_for_dir(lines: &Vec<String>) {
    // let mut map = HashMap::new();
    let mut current_dir = String::from("");
    for line in lines {
        if is_command(line) {
            current_dir = get_dir_change(line, &current_dir);
            println!(">> {}", current_dir);
        }
        else if is_dir(line) {

        }
    }
}

fn get_dir_change(line: &str, current_dir: &str) -> String {
    let command = get_command(line);
    if command == "ls" {
        let dirs: Vec<&str> = current_dir.split('/').collect();
        return concatenate(dirs[0], &dirs[1..(dirs.len() - 1)], "/");
    }
    else if command == "cd" {
        let to = cd_to(line);
        if to == ".." {
            let dirs: Vec<&str> = current_dir.split('/').collect();
            return concatenate(dirs[0], &dirs[1..(dirs.len() - 1)], "/");
        }
        else if to == "/" {
            return "".to_string();
        }
        return format!("{}/{}", current_dir, cd_to(line))
    }
    current_dir.to_string()
}

fn concatenate(start: &str, rest: &[&str], sep: &str) -> String {
    let mut ret = String::from(start);
    for &elem in rest {
        if ret != "" {
            ret = format!("{}{}{}", ret, sep, elem);
        } else {
            ret = format!("{}", elem);
        }

    }
    ret
}