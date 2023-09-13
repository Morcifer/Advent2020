use std::str;

use crate::utilities::file_utilities::read_lines;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Character {
    Summation,
    Multiplication,
    OpenParentheses,
    CloseParentheses,
    Number(u64),
}

fn parse_line(line: &str) -> Vec<Character> {
    line.chars()
        .filter_map(|c| match c {
            ' ' => None,
            '+' => Some(Character::Summation),
            '*' => Some(Character::Multiplication),
            '(' => Some(Character::OpenParentheses),
            ')' => Some(Character::CloseParentheses),
            n => n.to_digit(10).map(|n| Character::Number(n as u64)),
        })
        .collect()
}

fn parse_data(file_path: String) -> Vec<Vec<Character>> {
    read_lines(file_path)
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .collect()
}

fn get_number(
    line: &Vec<Character>,
    start_index: usize,
    end_index: usize,
    recursion_function: fn(&Vec<Character>, usize, usize) -> u64,
) -> (u64, usize) {
    match line[start_index] {
        Character::Number(n) => (n, start_index + 1),
        Character::OpenParentheses => {
            let mut number_of_open_parentheses = 0;
            let mut parentheses_end_index = 0;

            for (index, character) in line.iter().enumerate().take(end_index).skip(start_index) {
                if matches!(character, Character::OpenParentheses) {
                    number_of_open_parentheses += 1;
                } else if matches!(character, Character::CloseParentheses) {
                    number_of_open_parentheses -= 1;

                    if number_of_open_parentheses == 0 {
                        parentheses_end_index = index;
                        break;
                    }
                }
            }

            // println!("Parentheses open at {start_index} and close at {parentheses_end_index}");
            (
                recursion_function(line, start_index + 1, parentheses_end_index),
                parentheses_end_index + 1,
            )
        }
        _ => panic!("At index {start_index} I should have a number or parentheses! ({line:?})"),
    }
}

fn recursion_is_fun(line: &Vec<Character>, start_index: usize, end_index: usize) -> u64 {
    // First character should always be an open parentheses, or a number.
    let (mut result, mut start_index) = get_number(line, start_index, end_index, recursion_is_fun);

    // Then we can only have an operation, which we use and connect to the next thing until we run out.
    while start_index != end_index {
        let (next_number, next_start_index) =
            get_number(line, start_index + 1, end_index, recursion_is_fun);

        match line[start_index] {
            Character::Summation => result += next_number,
            Character::Multiplication => result *= next_number,
            _ => panic!("a number can only be followed by a summation or a multiplication!"),
        };

        start_index = next_start_index;
    }

    result
}

pub fn part_1(file_path: String) -> u64 {
    let lines = parse_data(file_path);
    lines
        .iter()
        .map(|line| recursion_is_fun(line, 0, line.len()))
        .sum()
}

fn recursion_is_fun_2(line: &Vec<Character>, start_index: usize, end_index: usize) -> u64 {
    let mut factors = vec![];

    // First character should always be an open parentheses, or a number.
    let (mut result, mut start_index) =
        get_number(line, start_index, end_index, recursion_is_fun_2);

    // Take care of all pluses first, then multiply all of the multiplications...
    while start_index != end_index {
        let (next_number, next_start_index) =
            get_number(line, start_index + 1, end_index, recursion_is_fun_2);

        match line[start_index] {
            Character::Summation => result += next_number,
            Character::Multiplication => {
                factors.push(result);
                result = next_number;
            }
            _ => panic!("a number can only be followed by a summation or a multiplication!"),
        };

        start_index = next_start_index;
    }

    factors.push(result);

    // println!("Handling {temp_factors:?}");
    factors.iter().product()
}

pub fn part_2(file_path: String) -> u64 {
    let lines = parse_data(file_path);
    lines
        .iter()
        .map(|line| recursion_is_fun_2(line, 0, line.len()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case("1", 1)]
    #[case("2 * 5", 10)]
    #[case("(2 * 5)", 10)]
    #[case("2 * 3 + (4 * 5)", 26)]
    #[case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437)]
    #[case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240)]
    #[case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632)]
    fn test_random_cases_part_1(#[case] expression: String, #[case] expected: u64) {
        let as_str = expression.as_str();
        let as_vector = parse_line(&as_str);
        assert_eq!(expected, recursion_is_fun(&as_vector, 0, as_vector.len()));
    }

    #[rstest]
    #[case("1", 1)]
    #[case("2 * 5", 10)]
    #[case("(2 * 5)", 10)]
    #[case("1 + 2 * 3 + 4 * 5 + 6", 231)]
    #[case("2 * 3 + (4 * 5)", 46)]
    #[case("1 + (2 * 3) + (4 * (5 + 6))", 51)]
    #[case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445)]
    #[case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060)]
    #[case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340)]
    fn test_random_cases_part_2(#[case] expression: String, #[case] expected: u64) {
        let as_str = expression.as_str();
        let as_vector = parse_line(&as_str);
        assert_eq!(expected, recursion_is_fun_2(&as_vector, 0, as_vector.len()));
    }

    #[rstest]
    #[case(true, 71+51+26+437+12240+13632)]
    #[case(false, 11297104473091)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 18, None)));
    }

    #[rstest]
    #[case(true, 231+51+46+1445+669060+23340)]
    #[case(false, 185348874183674)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 18, None)));
    }
}
