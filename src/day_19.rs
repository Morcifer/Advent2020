use std::collections::{HashMap, VecDeque};

use crate::utilities::file_utilities::read_lines;

#[derive(Clone, Debug, Eq, PartialEq)]
enum RuleType {
    OneOption(Vec<usize>),
    TwoOptions(Vec<usize>, Vec<usize>),
    A,
    B,
}

type Rule = (usize, RuleType);
type Message = VecDeque<char>;

fn parse_sub_rule_tuple(sub_rule_str: &str) -> Vec<usize> {
    sub_rule_str
        .split(' ')
        .map(str::trim)
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn parse_line(line: &str) -> (Option<Rule>, Option<Message>) {
    if line.contains(':') {
        let split_by_colon: Vec<&str> = line.split(':').map(str::trim).collect();
        let id = split_by_colon[0].trim().parse::<usize>().unwrap();
        let other_part = split_by_colon[1].trim();

        return if other_part == "\"a\"" {
            (Some((id, RuleType::A)), None)
        } else if other_part == "\"b\"" {
            (Some((id, RuleType::B)), None)
        } else if other_part.contains('|') {
            let split_by_or: Vec<Vec<usize>> = other_part
                .split('|')
                .map(str::trim)
                .map(parse_sub_rule_tuple)
                .collect();
            (
                Some((
                    id,
                    RuleType::TwoOptions(split_by_or[0].clone(), split_by_or[1].clone()),
                )),
                None,
            )
        } else {
            (
                Some((id, RuleType::OneOption(parse_sub_rule_tuple(other_part)))),
                None,
            )
        };
    }

    (None, Some(line.chars().collect()))
}

fn parse_data(file_path: String) -> Vec<(Option<Rule>, Option<Message>)> {
    read_lines(file_path)
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .collect()
}

fn iterate_over_rules(
    message: &VecDeque<char>,
    rules: &[usize],
    all_rules: &HashMap<usize, RuleType>,
) -> Vec<VecDeque<char>> {
    let mut remainders = vec![message.clone()];

    for rule in rules.iter() {
        let mut new_remainders = vec![];

        for remainder in remainders.into_iter() {
            let new_remainder = rule_type_matched(&remainder, *rule, all_rules);
            new_remainders.extend(new_remainder);
        }

        remainders = new_remainders;
    }

    remainders
}

fn rule_type_matched(
    message: &VecDeque<char>,
    rule_id: usize,
    all_rules: &HashMap<usize, RuleType>,
) -> Vec<VecDeque<char>> {
    return match all_rules.get(&rule_id).unwrap() {
        RuleType::OneOption(rules) => iterate_over_rules(message, rules, all_rules),
        RuleType::TwoOptions(rules_1, rules_2) => {
            let remainders_1 = iterate_over_rules(message, rules_1, all_rules);
            let remainders_2 = iterate_over_rules(message, rules_2, all_rules);

            remainders_1
                .into_iter()
                .chain(remainders_2.into_iter())
                .collect()
        }
        RuleType::A => {
            if message.is_empty() || message[0] != 'a' {
                return vec![];
            }

            let message = message.clone();
            vec![message.into_iter().skip(1).collect()]
        }
        RuleType::B => {
            if message.is_empty() || message[0] != 'b' {
                return vec![];
            }

            let message = message.clone();
            vec![message.into_iter().skip(1).collect()]
        }
    };
}

pub fn part_1(file_path: String) -> i64 {
    let data = parse_data(file_path);

    let rules: HashMap<usize, RuleType> = data
        .iter()
        .cloned()
        .filter_map(|(rule, _)| rule)
        .map(|rule| (rule.0, rule.1))
        .collect();

    let messages: Vec<VecDeque<char>> = data
        .iter()
        .cloned()
        .filter_map(|(_, message)| message)
        .collect();

    messages
        .into_iter()
        .filter(|message| {
            let result = rule_type_matched(message, 0, &rules);
            return !result.is_empty() && result.iter().any(|r| r.is_empty());
        })
        .count() as i64
}

pub fn part_2(file_path: String) -> i64 {
    let data = parse_data(file_path);

    let mut rules: HashMap<usize, RuleType> = data
        .iter()
        .cloned()
        .filter_map(|(rule, _)| rule)
        .map(|rule| (rule.0, rule.1))
        .collect();

    let messages: Vec<VecDeque<char>> = data
        .iter()
        .cloned()
        .filter_map(|(_, message)| message)
        .collect();

    rules.insert(8_usize, RuleType::TwoOptions(vec![42], vec![42, 8]));
    rules.insert(
        11_usize,
        RuleType::TwoOptions(vec![42, 31], vec![42, 11, 31]),
    );

    messages
        .into_iter()
        .filter(|message| {
            let result = rule_type_matched(message, 0, &rules);
            return !result.is_empty() && result.iter().any(|r| r.is_empty());
        })
        .count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 3)]
    #[case(false, 104)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 19, None)));
    }

    #[rstest]
    #[case(true, 12)]
    #[case(false, 314)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 19, None)));
    }
}
