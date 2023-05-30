#![allow(non_snake_case)]
#[cfg(test)]
mod day_1;
#[cfg(test)]
mod day_2;
#[cfg(test)]
mod day_3;
#[cfg(test)]
mod day_4;
#[cfg(test)]
mod day_5;
#[cfg(test)]
mod day_6;
mod day_7;
mod utilities;

use crate::day_7::{part_1, part_2};
use crate::utilities::file_utilities::get_file_path;

fn main() {
    let day = 7;
    let is_test = true;

    println!("Day {day} Part 1: {}", part_1(get_file_path(is_test, day)));
    println!("Day {day} Part 2: {}", part_2(get_file_path(is_test, day)));
}
