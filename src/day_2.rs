use std::str;

use crate::utilities::file_utilities::read_lines;

struct PolicyPassword {
    first_number: usize,
    second_number: usize,
    character: String,
    password: String,
}

fn parse_line(line: &str) -> PolicyPassword {
    // 2-9 c: ccccccccc
    let password_policy: Vec<&str> = line.split(':').map(str::trim).collect();
    let policy = password_policy[0];
    let password = password_policy[1];

    let numbers_character: Vec<&str> = policy.split(' ').map(str::trim).collect();
    let numbers = numbers_character[0];
    let character = numbers_character[1];

    let number_split: Vec<&str> = numbers.split('-').map(str::trim).collect();
    let first_number = number_split[0];
    let second_number = number_split[1];

    PolicyPassword {
        first_number: first_number.parse::<usize>().unwrap(),
        second_number: second_number.parse::<usize>().unwrap(),
        character: String::from(character),
        password: String::from(password),
    }
}

fn parse_data(file_path: String) -> Vec<PolicyPassword> {
    read_lines(file_path)
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .collect()
}

pub fn part_1(file_path: String) -> i32 {
    parse_data(file_path)
        .iter()
        .filter(|password_policy| {
            let matches = password_policy
                .password
                .matches(&password_policy.character)
                .count();

            password_policy.first_number <= matches && matches <= password_policy.second_number
        })
        .count() as i32
}

pub fn part_2(file_path: String) -> i32 {
    parse_data(file_path)
        .iter()
        .filter(|password_policy| {
            let first_equal = password_policy.password
                [password_policy.first_number - 1..=password_policy.first_number - 1]
                == password_policy.character;
            let second_equal = password_policy.password
                [password_policy.second_number - 1..=password_policy.second_number - 1]
                == password_policy.character;

            (first_equal && !second_equal) || (!first_equal && second_equal)
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 2)]
    #[case(false, 422)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 2)));
    }

    #[rstest]
    #[case(true, 1)]
    #[case(false, 451)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 2)));
    }
}
