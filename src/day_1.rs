use itertools::Itertools;

use crate::utilities::file_utilities::read_lines;

fn parse_line_to_int(line: String) -> i32 {
    line.parse::<i32>().unwrap()
}

fn parse_data(file_path: String) -> Vec<i32> {
    read_lines(file_path)
        .into_iter()
        .map(parse_line_to_int)
        .collect()
}

fn find_product_of_combination_with_given_sum(numbers: Vec<i32>, combination_size: usize) -> i32 {
    for combination in numbers.into_iter().combinations(combination_size) {
        if combination.iter().cloned().sum::<i32>() == 2020 {
            return combination.into_iter().product::<i32>();
        }
    }

    -1
}

pub fn part_1(file_path: String) -> i32 {
    let numbers = parse_data(file_path);
    find_product_of_combination_with_given_sum(numbers, 2)
}

pub fn part_2(file_path: String) -> i32 {
    let numbers = parse_data(file_path);
    find_product_of_combination_with_given_sum(numbers, 3)
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
        assert_eq!(expected, part_1(get_file_path(is_test, 1, None)));
    }

    #[rstest]
    #[case(true, 241861950)]
    #[case(false, 42140160)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 1, None)));
    }
}
