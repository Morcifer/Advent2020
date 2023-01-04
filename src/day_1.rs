#![allow(non_snake_case)]

mod utilities;

use std::io::{self};

use itertools::Itertools;

use crate::utilities::file_utilities::{get_file_path, read_lines};


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
