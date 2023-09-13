#![allow(non_snake_case)]
#[cfg(test)]
mod day_1;
#[cfg(test)]
mod day_10;
#[cfg(test)]
mod day_11;
#[cfg(test)]
mod day_12;
#[cfg(test)]
mod day_13;
#[cfg(test)]
mod day_14;
#[cfg(test)]
mod day_15;
#[cfg(test)]
mod day_16;
#[cfg(test)]
mod day_17;
mod day_18;
#[cfg(test)]
mod day_19;
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
#[cfg(test)]
mod day_7;
#[cfg(test)]
mod day_8;
#[cfg(test)]
mod day_9;

mod utilities;

use crate::day_18::{part_1, part_2};
use crate::utilities::file_utilities::get_file_path;

fn main() {
    let day = 18;
    let is_test = false;

    println!(
        "Day {day} Part 1: {}",
        part_1(get_file_path(is_test, day, None))
    );
    println!(
        "Day {day} Part 2: {}",
        part_2(get_file_path(is_test, day, None))
    );
}
