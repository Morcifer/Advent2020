#![allow(non_snake_case)]

mod utilities;

use std::io::{self};

// use itertools::Itertools;
// use regex::Regex;

use crate::utilities::file_utilities::{get_file_path, read_lines};


fn parse_line(line: io::Result<String>) -> String {
    line.unwrap()
}


fn tree_is_in_position(inputs: &Vec<String>, spot_row: usize, spot_column: usize) -> bool {
    let spot_row = spot_row;
    let spot_column = spot_column % inputs[0].len();

    return inputs[spot_row][spot_column..=spot_column].eq("#");
}


fn run(file_path: String) {
    let inputs: Vec<String> = read_lines(file_path)
        .expect("This should work fine...")
        .map(|line| parse_line(line))
        .collect();

    let height = inputs.len();

    let trees_per_slope: Vec<i64> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(dx, dy)| {
            let mut trees_in_slope = 0;

            let mut row = 0;
            let mut column = 0;

            while row + dy < height {
                column += dx;
                row += dy;

                if tree_is_in_position(&inputs, row, column) {
                    trees_in_slope += 1;
                }
            }

            trees_in_slope
        })
        .collect();

    println!("{:?}", trees_per_slope);

    let part_1_answer = trees_per_slope[1];
    let part_2_answer = trees_per_slope.iter().product::<i64>();

    println!("Part 1 answer: {part_1_answer}.");
    println!("Part 2 answer: {part_2_answer}.");
}


fn main() {
    let day = 3;
    let is_test = false;

    let file_path = get_file_path(is_test, day);

    run(file_path);
}
