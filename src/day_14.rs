use std::collections::HashMap;
use std::str;

use crate::utilities::file_utilities::read_lines;

const BITS: usize = 36;

#[derive(Clone, Debug, Eq, PartialEq)]
enum InputType {
    Mask([Option<usize>; BITS]),
    MemOverride(usize, i64),
}

fn parse_line(line: &str) -> InputType {
    let split_line: Vec<&str> = line.split('=').map(str::trim).collect();
    if split_line[0] == "mask" {
        let mut mask: [Option<usize>; BITS] = [None; BITS];
        for (index, char) in split_line[1].chars().rev().enumerate() {
            match char {
                'X' => mask[index] = None,
                '1' => mask[index] = Some(1),
                '0' => mask[index] = Some(0),
                _ => panic!(),
            }
        }

        return InputType::Mask(mask);
    }

    let length = split_line[0].len();
    let address = split_line[0][4..length - 1].parse::<usize>().unwrap();
    let value = split_line[1].parse::<i64>().unwrap();

    InputType::MemOverride(address, value)
}

fn parse_data(file_path: String) -> Vec<InputType> {
    read_lines(file_path)
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .collect()
}

pub fn part_1(file_path: String) -> i64 {
    let instructions = parse_data(file_path);
    let mut mask = [None; 36];
    let mut memory: HashMap<usize, i64> = HashMap::new();

    for instruction in instructions {
        match instruction {
            InputType::Mask(m) => mask = m,
            InputType::MemOverride(address, value) => {
                let mut masked_value = 0_i64;

                for (index, mask_bit) in mask.iter().enumerate() {
                    let bit = (value >> index) & 1;
                    let masked_bit = match *mask_bit {
                        None => bit,
                        Some(b) => b as i64,
                    };
                    masked_value += masked_bit * 2_i64.pow(index as u32);
                }

                memory.insert(address, masked_value);
            }
        }
    }

    memory.values().sum::<i64>()
}

pub fn part_2(file_path: String) -> i64 {
    let instructions = parse_data(file_path);
    let mut mask = [None; 36];
    let mut memory: HashMap<usize, i64> = HashMap::new();

    for instruction in instructions {
        match instruction {
            InputType::Mask(m) => mask = m,
            InputType::MemOverride(address, value) => {
                let mut masked_address = [None; 36];

                for (index, mask_bit) in mask.iter().enumerate() {
                    let bit = (address >> index) & 1;
                    masked_address[index] = match *mask_bit {
                        None => None,
                        Some(0) => Some(bit),
                        Some(1) => Some(1),
                        _ => panic!(),
                    };
                }

                let mut addresses = vec![0];

                for (index, masked_address_bit) in masked_address.iter().enumerate() {
                    addresses = match *masked_address_bit {
                        None => addresses
                            .into_iter()
                            .flat_map(|address| vec![address, address + 2_usize.pow(index as u32)])
                            .collect(),
                        Some(0) => addresses,
                        Some(1) => addresses
                            .into_iter()
                            .map(|address| address + 2_usize.pow(index as u32))
                            .collect(),
                        _ => panic!(),
                    }
                }

                for address in addresses {
                    memory.insert(address, value);
                }
            }
        }
    }

    memory.values().sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 51)]
    #[case(false, 9628746976360)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 14)));
    }

    #[rstest]
    #[case(true, 208)]
    #[case(false, 4574598714592)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 14)));
    }
}
