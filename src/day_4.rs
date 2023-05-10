use itertools::Itertools;

use crate::utilities::file_utilities::read_lines;

enum PassportField {
    BirthYear(String),
    IssueYear(String),
    ExpirationYear(String),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportID(String),
}

fn is_passport_field_valid(passport_field: &PassportField) -> bool {
    match passport_field {
        PassportField::BirthYear(year) => match year.parse::<i32>() {
            Ok(year) => (1920..=2002).contains(&year),
            Err(_) => false,
        },
        PassportField::IssueYear(year) => match year.parse::<i32>() {
            Ok(year) => (2010..=2020).contains(&year),
            Err(_) => false,
        },
        PassportField::ExpirationYear(year) => match year.parse::<i32>() {
            Ok(year) => (2020..=2030).contains(&year),
            Err(_) => false,
        },
        PassportField::Height(height) => {
            let (height_value, height_type) = height.split_at(height.len() - 2);
            match (height_type, height_value.parse::<i32>()) {
                ("cm", Ok(height)) => (150..=193).contains(&height),
                ("in", Ok(height)) => (59..=76).contains(&height),
                _ => false,
            }
        }
        PassportField::HairColor(color) => {
            let (hcl_hashtag, hair_color) = color.split_at(1);
            return hcl_hashtag == "#"
                && hair_color.len() == 6
                && hair_color.chars().all(|c| c.is_ascii_hexdigit());
        }
        PassportField::EyeColor(color) => {
            vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&color.as_str())
        }
        PassportField::PassportID(passport_id) => {
            passport_id.len() == 9 && passport_id.chars().all(|c| c.is_ascii_digit())
        }
    }
}

struct Passport {
    #[allow(dead_code)]
    passport_id: String,
    keys: Vec<String>,
    fields: Vec<PassportField>,
}

fn required_fields() -> Vec<String> {
    vec![
        "byr", // (Birth Year) - four digits; at least 1920 and at most 2002.
        "iyr", // (Issue Year) - four digits; at least 2010 and at most 2020.
        "eyr", // (Expiration Year) - four digits; at least 2020 and at most 2030.
        "hgt", // (Height) - a number followed by either cm or in. 150 <= cm <= 193; 59 <= in <= 76
        "hcl", // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        "ecl", // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        "pid", // (Passport ID) - a nine-digit number, including leading zeroes.
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

fn parse_line(line: String) -> Passport {
    let mut keys = vec![];
    let mut passport_id = String::from("");

    let fields: Vec<PassportField> = line
        .split(' ')
        .map(str::trim)
        .map(|field| field.split(':').map(str::trim).collect::<Vec<&str>>())
        .filter_map(|key_value| {
            let key = String::from(key_value[0]);
            let value = String::from(key_value[1]);

            keys.push(key.clone());
            match key.as_str() {
                "byr" => Some(PassportField::BirthYear(value)),
                "iyr" => Some(PassportField::IssueYear(value)),
                "eyr" => Some(PassportField::ExpirationYear(value)),
                "hgt" => Some(PassportField::Height(value)),
                "hcl" => Some(PassportField::HairColor(value)),
                "ecl" => Some(PassportField::EyeColor(value)),
                "pid" => {
                    passport_id = value.clone();
                    Some(PassportField::PassportID(value))
                }
                "cid" => None,
                &_ => None, // Should fail here?
            }
        })
        .collect();

    Passport {
        passport_id,
        keys,
        fields,
    }
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

    return passports
        .iter()
        .filter(|passport| {
            for required_field in &required_fields() {
                if !passport.keys.contains(required_field) {
                    return false;
                }
            }

            true
        })
        .count() as i64;
}

pub fn part_2(file_path: String) -> i64 {
    let passports = parse_data(file_path);

    return passports
        .iter()
        .filter(|passport| {
            for required_field in &required_fields() {
                if !passport.keys.contains(required_field) {
                    return false;
                }
            }

            true
        })
        .filter(|passport| passport.fields.iter().all(is_passport_field_valid))
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

    #[rstest]
    #[case(true, 2)]
    #[case(false, 150)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 4)));
    }
}
