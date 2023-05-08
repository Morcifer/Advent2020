use itertools::Itertools;
use std::collections;

use crate::utilities::file_utilities::read_lines;

struct Passport {
    fields: collections::HashMap<String, String>,
}

fn parse_line(line: String) -> Passport {
    let fields: collections::HashMap<String, String> = line
        .split(' ')
        .map(str::trim)
        .map(|field| field.split(":").map(str::trim).collect::<Vec<&str>>())
        .map(|key_value| (String::from(key_value[0]), String::from(key_value[1])))
        .collect();

    Passport { fields }
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

    return passports
        .iter()
        .filter(|passport| {
            for required_field in &required_fields {
                if !passport.fields.contains_key(&String::from(*required_field)) {
                    return false;
                }
            }

            true
        })
        .count() as i64;
}

pub fn part_2(file_path: String) -> i64 {
    let passports = parse_data(file_path);

    let required_fields = vec![
        "byr", // (Birth Year) - four digits; at least 1920 and at most 2002.
        "iyr", // (Issue Year) - four digits; at least 2010 and at most 2020.
        "eyr", // (Expiration Year) - four digits; at least 2020 and at most 2030.
        "hgt", // (Height) - a number followed by either cm or in. 150 <= cm <= 193; 59 <= in <= 76
        "hcl", // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        "ecl", // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        "pid", // (Passport ID) - a nine-digit number, including leading zeroes.
    ];

    return passports
        .iter()
        .filter(|passport| {
            for required_field in &required_fields {
                if !passport.fields.contains_key(&String::from(*required_field)) {
                    return false;
                }
            }

            true
        })
        .filter(|passport| {
            if let Ok(byr) = passport
                .fields
                .get(&String::from("byr"))
                .unwrap()
                .parse::<i32>()
            {
                // (Birth Year) - four digits; at least 1920 and at most 2002.
                if !(1920..=2002).contains(&byr) {
                    return false;
                }
            } else {
                return false;
            }

            if let Ok(iyr) = passport
                .fields
                .get(&String::from("iyr"))
                .unwrap()
                .parse::<i32>()
            {
                // (Issue Year) - four digits; at least 2010 and at most 2020.
                if !(2010..=2020).contains(&iyr) {
                    return false;
                }
            } else {
                return false;
            }

            if let Ok(eyr) = passport
                .fields
                .get(&String::from("eyr"))
                .unwrap()
                .parse::<i32>()
            {
                // (Issue Year) - four digits; at least 2010 and at most 2020.
                if !(2020..=2030).contains(&eyr) {
                    return false;
                }
            } else {
                return false;
            }

            let hgt = passport.fields.get(&String::from("hgt")).unwrap();
            let (hgt_value, hgt_type) = hgt.split_at(hgt.len() - 2);

            if let Ok(hgt_number) = hgt_value.parse::<i32>() {
                // (Height) - a number followed by either cm or in. 150 <= cm <= 193; 59 <= in <= 76
                if hgt_type == "cm" && !(150..=193).contains(&hgt_number) {
                    return false;
                } else if hgt_type == "in" && !(59..=76).contains(&hgt_number) {
                    return false;
                } else if hgt_type != "cm" && hgt_type != "in" {
                    return false;
                }
            } else {
                return false;
            }

            // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            let hcl = passport.fields.get(&String::from("hcl")).unwrap();
            let (hcl_hashtag, hcl_value) = hcl.split_at(1);
            if hcl_hashtag != "#" || hcl_value.len() != 6 {
                return false;
            }

            let valid_numbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
            let valid_letters = ["a", "b", "c", "d", "e", "f"];
            for hcl_char in hcl_value.chars() {
                if !valid_numbers.contains(&&*hcl_char.to_string())
                    && !valid_letters.contains(&&*hcl_char.to_string())
                {
                    return false;
                }
            }

            // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            let ecl = passport.fields.get("ecl").unwrap();
            let valid_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            if !valid_colors.contains(&ecl.as_str()) {
                return false;
            }

            // (Passport ID) - a nine-digit number, including leading zeroes.
            let pid = passport.fields.get(&String::from("pid")).unwrap();
            if pid.len() != 9 {
                return false;
            }

            for pid_char in pid.chars() {
                if !valid_numbers.contains(&&*pid_char.to_string()) {
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

    #[rstest]
    #[case(true, 2)]
    #[case(false, 150)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 4)));
    }
}
