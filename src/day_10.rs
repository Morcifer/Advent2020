use crate::utilities::file_utilities::read_lines;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str;

fn parse_line(line: &str) -> usize {
    line.parse::<usize>().unwrap()
}

fn parse_data(file_path: String) -> Vec<usize> {
    let mut adaptors: Vec<usize> = read_lines(file_path)
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .collect();

    adaptors.insert(0, 0); // Initial outlet
    adaptors.push(adaptors.iter().max().unwrap() + 3); // Final adapter
    adaptors.sort();

    adaptors
}

pub fn part_1(file_path: String) -> usize {
    let sorted_adaptors = parse_data(file_path);

    let mut one_diff = 0;
    let mut _two_diff = 0;
    let mut three_diff = 0;

    for (first, second) in sorted_adaptors.into_iter().tuple_windows() {
        match second - first {
            1 => one_diff += 1,
            2 => _two_diff += 1,
            3 => three_diff += 1,
            _ => panic!(),
        }
    }

    one_diff * three_diff
}

pub fn part_2(file_path: String) -> usize {
    let sorted_adaptors = parse_data(file_path);
    let sorted_adaptors_hashset: HashSet<usize> =
        HashSet::from_iter(sorted_adaptors.iter().cloned());
    let target = sorted_adaptors.iter().max().unwrap();

    let mut adaptor_path_count: HashMap<usize, usize> = vec![(0, 1)].into_iter().collect();

    for adaptor in sorted_adaptors.iter() {
        let path_count_here = adaptor_path_count[adaptor];

        for delta in 1..=3 {
            let next_adaptor = *adaptor + delta;
            if !sorted_adaptors_hashset.contains(&next_adaptor) {
                continue;
            }
            *adaptor_path_count.entry(next_adaptor).or_insert(0) += path_count_here;
        }
    }

    adaptor_path_count[target]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 35)]
    #[case(false, 1914)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_1(get_file_path(is_test, 10, None)));
    }

    #[rstest]
    #[case(true, 8)]
    #[case(false, 9256148959232)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 10, None)));
    }
}
