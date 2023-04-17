#![allow(non_snake_case)]

mod utilities;

use std::io::{self};

// use itertools::Itertools;
// use regex::Regex;

use crate::utilities::file_utilities::{get_file_path, read_lines};


fn parse_line(line: io::Result<String>) -> String {
    line.unwrap()
}


fn run(file_path: String) {
    let required_fields = vec![
        "byr", // (Birth Year)
        "iyr", // (Issue Year)
        "eyr", // (Expiration Year)
        "hgt", // (Height)
        "hcl", // (Hair Color)
        "ecl", // (Eye Color)
        "pid", // (Passport ID)
    ];

    let optional_fields = vec![
        "cid", // (Country ID)
    ];

    let inputs: Vec<String> = read_lines(file_path)
        .expect("This should work fine...")
        .map(|line| parse_line(line))
        .collect();

    let mut valid_passports = 0;

    let mut combined_passport: String = "".to_owned();
    let mut is_valid_passport = true;

    for (index, input) in inputs.iter().enumerate() {
        combined_passport.push_str(&input);
        combined_passport.push_str(" ");

        if input == "" || index == inputs.len() - 1 {
            for required_field in &required_fields {
                is_valid_passport = is_valid_passport && combined_passport.contains(required_field);
            }

            if is_valid_passport {
                valid_passports += 1;
                println!("{:?}: {:?}", valid_passports, combined_passport);
            }

            combined_passport = "".to_owned();
            is_valid_passport = true;
        }
    }

    let part_1_answer = valid_passports;

    println!("Part 1 answer: {part_1_answer}.");  // 215 is too low.
}


fn main() {
    let day = 4;
    let is_test = true;

    let file_path = get_file_path(is_test, day);

    run(file_path);
}
