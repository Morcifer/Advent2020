use itertools::Itertools;

use crate::utilities::file_utilities::read_lines;

struct Passport {
    full: String,
}

fn parse_line(line: String) -> Passport {
    Passport { full: line }
}

fn parse_data(file_path: String) -> Vec<Passport> {
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
    let passports = parse_data(file_path);

    let required_fields = vec![
        "byr", // (Birth Year)
        "iyr", // (Issue Year)
        "eyr", // (Expiration Year)
        "hgt", // (Height)
        "hcl", // (Hair Color)
        "ecl", // (Eye Color)
        "pid", // (Passport ID)
    ];

    // let optional_fields = vec![
    //     "cid", // (Country ID)
    // ];

    return passports
        .iter()
        .filter(|passport| {
            for required_field in &required_fields {
                if !passport.full.contains(required_field) {
                    return false;
                }
            }

            true
        })
        .count() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 2)]
    #[case(false, 216)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 4)));
    }
}
