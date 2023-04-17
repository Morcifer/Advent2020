#![allow(non_snake_case)]
mod day_1;
mod utilities;

use crate::day_1::{day_1_part_1, day_1_part_2};
use crate::utilities::file_utilities::get_file_path;

fn main() {
    let day = 1;
    let is_test = false;

    println!(
        "Day {day} Part 1: {}",
        day_1_part_1(get_file_path(is_test, day))
    );
    println!(
        "Day {day} Part 2: {}",
        day_1_part_2(get_file_path(is_test, day))
    );
}
