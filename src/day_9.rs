use std::str;

use crate::utilities::file_utilities::read_lines;

fn parse_line(line: &str) -> usize {
    line.parse::<usize>().unwrap()
}

fn parse_data(file_path: String) -> Vec<usize> {
    read_lines(file_path)
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .collect()
}

pub fn part_1(file_path: String, preamble: usize) -> usize {
    let numbers = parse_data(file_path);

    for (i, number) in numbers.iter().enumerate().skip(preamble) {
        let mut match_exists = false;

        for prev_1 in &numbers[i - preamble..i] {
            for prev_2 in &numbers[i - preamble..i] {
                if prev_1 + prev_2 == *number {
                    match_exists = true;
                    break;
                }
            }
        }

        if !match_exists {
            return *number;
        }
    }

    0
}

pub fn part_2(file_path: String, preamble: usize) -> usize {
    let invalid_number = part_1(file_path.clone(), preamble);

    let numbers = parse_data(file_path);

    let contiguous_set = numbers
        .iter()
        .enumerate()
        .filter_map(|(i, number)| {
            let mut sum = *number;
            let mut set = vec![*number];
            let mut index = i;

            while sum < invalid_number && index < numbers.len() - 1 {
                index += 1;
                sum += numbers[index];
                set.push(numbers[index])
            }

            if sum == invalid_number {
                Some(set)
            } else {
                None
            }
        })
        .next()
        .unwrap();

    contiguous_set.iter().min().unwrap() + contiguous_set.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 127)]
    #[case(false, 217430975)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: usize) {
        let preamble = if is_test { 5 } else { 25 };
        assert_eq!(expected, part_1(get_file_path(is_test, 9, None), preamble));
    }

    #[rstest]
    #[case(true, 62)]
    #[case(false, 28509180)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        let preamble = if is_test { 5 } else { 25 };
        assert_eq!(expected, part_2(get_file_path(is_test, 9, None), preamble));
    }
}
