use crate::utilities::file_utilities::read_lines;

type Rule = (String, (usize, usize), (usize, usize));
type Ticket = Vec<usize>;

fn parse_line(line: &str) -> (Option<Rule>, Option<Ticket>) {
    if line.contains(':') {
        let split_by_colon: Vec<&str> = line.split(':').map(str::trim).collect();
        let field = split_by_colon[0].to_string();
        let two_rules: Vec<&str> = split_by_colon[1].split(' ').map(str::trim).collect();
        let rule_1_string: Vec<&str> = two_rules[0].split('-').map(str::trim).collect();
        let rule_2_string: Vec<&str> = two_rules[2].split('-').map(str::trim).collect();

        (
            Some((
                field,
                (
                    rule_1_string[0].parse::<usize>().unwrap(),
                    rule_1_string[1].parse::<usize>().unwrap(),
                ),
                (
                    rule_2_string[0].parse::<usize>().unwrap(),
                    rule_2_string[1].parse::<usize>().unwrap(),
                ),
            )),
            None,
        )
    } else {
        (
            None,
            Some(
                line.split(',')
                    .map(str::trim)
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect(),
            ),
        )
    }
}

fn parse_data(file_path: String) -> Vec<(Option<Rule>, Option<Ticket>)> {
    read_lines(file_path)
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .collect()
}

pub fn part_1(file_path: String) -> i64 {
    let data = parse_data(file_path);

    let rules: Vec<(usize, usize)> = data
        .iter()
        .cloned()
        .filter_map(|(rule, _)| rule)
        .flat_map(|rule| vec![rule.1, rule.2])
        .collect();

    let fields: Vec<usize> = data
        .iter()
        .cloned()
        .filter_map(|(_, ticket)| ticket)
        .flatten()
        .collect();

    fields
        .into_iter()
        .skip(rules.len() / 2)
        .filter(|field| {
            return rules
                .iter()
                .all(|(start, end)| !(start..=end).contains(&field));
        })
        .sum::<usize>() as i64
}

pub fn part_2(file_path: String) -> i64 {
    let data = parse_data(file_path);

    let rules: Vec<Rule> = data.iter().cloned().filter_map(|(rule, _)| rule).collect();

    let valid_tickets: Vec<Ticket> = data
        .iter()
        .cloned()
        .filter_map(|(_, ticket)| ticket)
        .filter(|ticket| {
            ticket.iter().all(|field| {
                rules.iter().any(|rule| {
                    (rule.1 .0..=rule.1 .1).contains(field)
                        || (rule.2 .0..=rule.2 .1).contains(field)
                })
            })
        })
        .collect();

    let mut validity_matrix = vec![vec![false; rules.len()]; rules.len()];

    for field_index in 0..rules.len() {
        for (rule_index, rule) in rules.iter().enumerate() {
            validity_matrix[field_index][rule_index] = valid_tickets
                .iter()
                .map(|ticket| ticket[field_index])
                .all(|field| {
                    (rule.1 .0..=rule.1 .1).contains(&field)
                        || (rule.2 .0..=rule.2 .1).contains(&field)
                });
        }
    }

    let mut known_field_indices = vec![];
    let mut known_rule_field_indices = vec![42; rules.len()];

    while known_field_indices.len() < rules.len() {
        for field_index in 0..rules.len() {
            if known_field_indices.contains(&field_index) {
                continue;
            }

            let matches = validity_matrix[field_index]
                .iter()
                .enumerate()
                .filter(|(rule_index, _)| !known_rule_field_indices.contains(rule_index))
                .filter_map(
                    |(rule_index, is_valid)| {
                        if *is_valid {
                            Some(rule_index)
                        } else {
                            None
                        }
                    },
                )
                .collect::<Vec<_>>();

            let first_match = matches.first().unwrap();
            let last_match = matches.last().unwrap();

            if first_match == last_match {
                known_field_indices.push(field_index);
                known_rule_field_indices[field_index] = *first_match;
                break;
            }
        }
    }

    let mut result = 1;
    for (field_index, rule_index) in known_rule_field_indices.into_iter().enumerate() {
        // println!("{}: {}", rules[rule_index].0, valid_tickets[0][field_index]);

        if rules[rule_index].0.contains("departure") {
            result *= valid_tickets[0][field_index];
        }
    }

    result as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case("-a", 71)]
    #[case("-b", 0)]
    fn test_part_1(#[case] suffix: &str, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(true, 16, Some(suffix))));
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(20058, part_1(get_file_path(false, 16, None)));
    }

    #[test]
    fn test_part_2_real() {
        assert_eq!(366871907221, part_2(get_file_path(false, 16, None)));
    }
}
