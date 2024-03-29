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
#[cfg(test)]
mod day_18;
#[cfg(test)]
mod day_19;
#[cfg(test)]
mod day_2;
#[cfg(test)]
mod day_20;
#[cfg(test)]
mod day_21;
#[cfg(test)]
mod day_22;
#[cfg(test)]
mod day_23;
#[cfg(test)]
mod day_24;
mod day_25;
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

use crate::day_25::part_1;

// use crate::day_24::{part_1, part_2};
// use crate::utilities::file_utilities::get_file_path;

fn main() {
    let day = 25;
    let is_test = false;

    let data = if is_test {
        (5764801, 17807724)
    } else {
        (9717666, 20089533)
    };

    println!(
        "Day {day} Part 1: {}",
        part_1(data), // get_file_path(is_test, day, None))
    );
    // println!(
    //     "Day {day} Part 2: {}",
    //     get_file_path(is_test, day, None)),
    // );
}
