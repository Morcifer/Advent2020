use itertools::Itertools;

use crate::utilities::file_utilities::read_lines;
use std::collections::HashSet;

fn parse_line(line: String) -> Vec<HashSet<char>> {
    line.split(' ')
        .map(|answer| HashSet::from_iter(answer.chars()))
        .collect()
}

fn parse_data(file_path: String) -> Vec<Vec<HashSet<char>>> {
    read_lines(file_path)
        .into_iter()
        .coalesce(|x, y| {
            if x.is_empty() == y.is_empty() {
                Ok(format!("{} {}", x, y))
            } else {
                Err((x, y))
            }
        })
        .filter(|s| !s.is_empty())
        .map(parse_line)
        .collect()
}

pub fn part_1(file_path: String) -> i64 {
    parse_data(file_path)
        .iter()
        .map(|answers| answers.iter().flatten().collect::<HashSet<_>>())
        .map(|s| s.len() as i64)
        .sum()
}

pub fn part_2(file_path: String) -> i64 {
    parse_data(file_path)
        .iter()
        .map(|answers| {
            answers.iter().fold(answers[0].clone(), |acc, new| {
                acc.intersection(new).copied().collect()
            })
        })
        .map(|s| s.len() as i64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 11)]
    #[case(false, 6521)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 6)));
    }

    #[rstest]
    #[case(true, 6)]
    #[case(false, 3305)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 6)));
    }
}
