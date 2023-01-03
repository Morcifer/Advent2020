#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn get_file_path(is_test: bool, day: u32) -> String {
    let sub_folder = if is_test { "test" } else { "real" };
    format!("./data/{sub_folder}/day{day}.txt")
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn parse_line_to_int(line: io::Result<String>) -> i32 {
    return line.unwrap().parse::<i32>().unwrap();
}


fn run(file_path: String) {
    let numbers: Vec<i32> = read_lines(file_path)
        .expect("This should work fine...")
        .map(|line| parse_line_to_int(line))
        .collect();

    for pair in numbers.iter().cloned().into_iter().combinations(2) {
        if pair[0] + pair[1] == 2020 {
            println!("Day 1: {}", pair[0] * pair[1]);
            break;
        }
    }

    for pair in numbers.iter().cloned().into_iter().combinations(3) {
        if pair[0] + pair[1] + pair[2] == 2020 {
            println!("Day 2: {}", pair[0] * pair[1] * pair[2]);
            break;
        }
    }
}


fn main() {
    let day = 1;
    let is_test = false;

    let file_path = get_file_path(is_test, day);

    run(file_path);
}
