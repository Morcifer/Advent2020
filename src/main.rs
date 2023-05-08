#![allow(non_snake_case)]
#[cfg(test)]
mod day_1;
#[cfg(test)]
mod day_2;
mod day_3;
mod utilities;

use crate::day_3::{part_1, part_2};
use crate::utilities::file_utilities::get_file_path;

fn main() {
    let day = 3;
    let is_test = false;

    println!("Day {day} Part 1: {}", part_1(get_file_path(is_test, day)));
    println!("Day {day} Part 2: {}", part_2(get_file_path(is_test, day)));
}
