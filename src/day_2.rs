#![allow(non_snake_case)]

mod utilities;

use std::io::{self};

// use itertools::Itertools;
use regex::Regex;

use crate::utilities::file_utilities::{get_file_path, read_lines};


fn parse_line(line: io::Result<String>) -> (usize, usize, String , String ) {
    // 2-9 c: ccccccccc
    let re = Regex::new(r"(\d{1,2})-(\d{1,2}) (\w): (\w{1,30})").unwrap();
    let line = line.unwrap();
    let caps = re.captures(&line).unwrap();

    println!("{:?}", caps);

    return (
        caps[1].parse::<usize>().unwrap(),
        caps[2].parse::<usize>().unwrap(),
        caps[3].to_owned(),
        caps[4].to_owned(),
    );
}

fn run(file_path: String) {
    let tuples: Vec<(usize, usize, String, String)> = read_lines(file_path)
        .expect("This should work fine...")
        .map(|line| parse_line(line))
        .collect();

    let mut valid_count = 0;
    let mut valid_count_2 = 0;

    for tuple in tuples {
        let (first_number, second_number, character, string) = tuple;
        let matches = string.matches(&character).count();

        if first_number <= matches && matches <= second_number {
            valid_count = valid_count + 1;
        }

        let first_equal = string[first_number - 1..=first_number - 1] == character;
        let second_equal = string[second_number - 1..=second_number - 1] == character;

        if (first_equal && !second_equal) || (!first_equal && second_equal) {
            valid_count_2 = valid_count_2 + 1;
        }
    }

    println!("There are {valid_count} valid passwords for part 1.");
    println!("There are {valid_count_2} valid passwords for part 2.");
}


fn main() {
    let day = 2;
    let is_test = false;

    let file_path = get_file_path(is_test, day);

    run(file_path);
}
