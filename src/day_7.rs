use std::collections::{HashMap, HashSet, VecDeque};
use std::str;

use crate::utilities::file_utilities::read_lines;

struct BagRequirement {
    // ... (contains) 2 shiny gold bags, 9 faded blue bags.
    bag_color: String,
    bag_number: usize,
}

fn parse_line(line: &str) -> (String, Vec<BagRequirement>) {
    // muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    let contain_split: Vec<&str> = line.split("contain").map(str::trim).collect();

    let bag_color = contain_split[0]
        .split("bags")
        .map(str::trim)
        .next()
        .unwrap();

    let bag_requirements = contain_split[1]
        .split(',')
        .map(str::trim)
        .map(|requirement| {
            let substrings = requirement.split(' ').map(str::trim).collect::<Vec<_>>();
            let number = substrings[0].parse::<usize>().unwrap_or(0);
            let color = substrings[1..substrings.len() - 1].join(" ");

            BagRequirement {
                bag_color: color,
                bag_number: number,
            }
        })
        .collect::<Vec<_>>();

    (String::from(bag_color), bag_requirements)
}

fn parse_data(file_path: String) -> Vec<(String, Vec<BagRequirement>)> {
    read_lines(file_path)
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .collect()
}

pub fn part_1(file_path: String) -> i32 {
    let bag_policies: HashMap<_, Vec<_>> = parse_data(file_path).into_iter().collect();

    let all_bag_colors: HashSet<String> = bag_policies
        .keys()
        .cloned()
        .chain(
            bag_policies
                .values()
                .flat_map(|value| value.iter().map(|r| r.bag_color.clone())),
        )
        .collect();

    all_bag_colors
        .into_iter()
        .filter(|bag_color| {
            let mut queue: VecDeque<String> = [bag_color.clone()].into();
            let mut explored: HashSet<String> = HashSet::new();

            while let Some(to_explore) = queue.pop_back() {
                if to_explore == *"shiny gold" {
                    return true;
                }

                explored.insert(to_explore.clone());

                for policy in bag_policies.get(&to_explore).unwrap_or(&vec![]).iter() {
                    if !explored.contains(&policy.bag_color) {
                        queue.push_front(policy.bag_color.clone());
                    }
                }
            }

            false
        })
        .count() as i32
        - 1
}

pub fn part_2(file_path: String) -> i32 {
    let bag_policies: HashMap<_, Vec<_>> = parse_data(file_path).into_iter().collect();

    let mut number_of_bags = 0;

    // TODO: Is there a way to cache this, or combine entries?
    let mut queue: VecDeque<(String, usize)> = [(String::from("shiny gold"), 1)].into();

    while let Some((bag_color, multiplier)) = queue.pop_back() {
        number_of_bags += multiplier;

        for policy in bag_policies.get(&bag_color).unwrap_or(&vec![]).iter() {
            queue.push_front((policy.bag_color.clone(), multiplier * policy.bag_number));
        }
    }

    number_of_bags as i32 - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 4)]
    #[case(false, 248)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 7, None)));
    }

    #[rstest]
    #[case(true, 32)]
    #[case(false, 57281)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 7, None)));
    }
}
