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

pub fn part_1(file_path: String) -> i32 {
    let numbers = parse_data(file_path);

    for pair in numbers.into_iter().combinations(2) {
        if pair[0] + pair[1] == 2020 {
            return pair[0] * pair[1];
        }
    }

    -1
}

pub fn part_2(file_path: String) -> i32 {
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
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 514579)]
    #[case(false, 1010299)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 1)));
    }

    #[rstest]
    #[case(true, 241861950)]
    #[case(false, 42140160)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 1)));
    }
}
