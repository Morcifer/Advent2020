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
mod day_15;
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

use crate::day_15::{part_1, part_2};
// use crate::utilities::file_utilities::get_file_path;

fn main() {
    let day = 15;
    let is_test = true;

    let numbers = if is_test {
        vec![0, 3, 6]
    } else {
        vec![6, 4, 12, 1, 20, 0, 16]
    };

    println!("Day {day} Part 1: {}", part_1(&numbers));
    println!("Day {day} Part 2: {}", part_2(&numbers));
}
