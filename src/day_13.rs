use std::str;

use crate::utilities::file_utilities::read_lines;

fn parse_line(line: &str) -> Vec<Option<usize>> {
    line.split(',')
        .map(str::trim)
        .map(|x| match x.parse::<usize>() {
            Ok(number) => Some(number),
            Err(_) => None,
        })
        .collect()
}

fn parse_data(file_path: String) -> Vec<Vec<Option<usize>>> {
    read_lines(file_path)
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .collect()
}

pub fn part_1(file_path: String) -> usize {
    let numbers = parse_data(file_path);

    let arrival_time = numbers[0][0].unwrap();
    let busses = numbers[1].clone().into_iter().flatten().collect::<Vec<_>>();

    let mut min_bus = usize::MAX;
    let mut min_wait = usize::MAX;

    for bus in busses.into_iter() {
        let wait = bus - arrival_time % bus;
        if wait < min_wait {
            min_wait = wait;
            min_bus = bus;
        }
    }

    min_wait * min_bus
}

pub fn part_2(file_path: String) -> isize {
    let numbers = parse_data(file_path);

    let mut time = 0_isize;
    let mut required_modulo = 0_isize;
    let mut jump = 1_isize;

    for bus in numbers[1].iter() {
        if let Some(bus) = bus {
            let bus = *bus as isize;
            while (time + required_modulo) % bus != 0 {
                time += jump;
            }
            println!(
                "At time {time}, bus {bus} has modulo {required_modulo} (after jumps of {jump})"
            );
            required_modulo += 1;
            jump *= bus;
        } else {
            // println!("I don't care about modulo {required_modulo} time {time}");
            required_modulo += 1;
        }
    }

    println!("{time}, {required_modulo}, {jump}");
    time
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 295)]
    #[case(false, 1835)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_1(get_file_path(is_test, 13)));
    }

    #[rstest]
    #[case(true, 1068781)]
    #[case(false, 247086664214628)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: isize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 13)));
    }
}
