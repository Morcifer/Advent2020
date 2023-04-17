use std::io;

use itertools::Itertools;

use crate::utilities::file_utilities::read_lines;

fn parse_line_to_int(line: io::Result<String>) -> i32 {
    line.unwrap().parse::<i32>().unwrap()
}

fn parse_data(file_path: String) -> Vec<i32> {
    read_lines(file_path)
        .expect("This should work fine...")
        .map(parse_line_to_int)
        .collect()
}

pub fn day_1_part_1(file_path: String) -> i32 {
    let numbers = parse_data(file_path);

    for pair in numbers.into_iter().combinations(2) {
        if pair[0] + pair[1] == 2020 {
            return pair[0] * pair[1];
        }
    }

    -1
}

pub fn day_1_part_2(file_path: String) -> i32 {
    let numbers = parse_data(file_path);

    for pair in numbers.into_iter().combinations(3) {
        if pair[0] + pair[1] + pair[2] == 2020 {
            return pair[0] * pair[1] * pair[2];
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::file_utilities::get_file_path;

    #[test]
    fn test_part_1() {
        assert_eq!(514579, day_1_part_1(get_file_path(true, 1)));
        assert_eq!(1010299, day_1_part_1(get_file_path(false, 1)));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(241861950, day_1_part_2(get_file_path(true, 1)));
        assert_eq!(42140160, day_1_part_2(get_file_path(false, 1)));
    }
}
