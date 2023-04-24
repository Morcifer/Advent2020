#![allow(non_snake_case)]
// mod day_1;
mod day_2;
mod utilities;

use crate::day_2::{part_1, part_2};
use crate::utilities::file_utilities::get_file_path;

fn main() {
    let day = 2;
    let is_test = false;

    println!(
        "Day {day} Part 1: {}",
        part_1(get_file_path(is_test, day))
    );
    println!(
        "Day {day} Part 2: {}",
        part_2(get_file_path(is_test, day))
    );
}
