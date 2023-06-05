use std::collections::HashSet;
use std::str;

use crate::utilities::file_utilities::read_lines;

#[derive(Clone)]
enum Instruction {
    Accumulate(i32),
    Jump(i32),
    NoOp,
}

fn parse_line(line: &str) -> Instruction {
    let operation_number: Vec<&str> = line.split(' ').map(str::trim).collect();
    let value = operation_number[1].parse::<i32>().unwrap();
    match operation_number[0] {
        "acc" => Instruction::Accumulate(value),
        "jmp" => Instruction::Jump(value),
        "nop" => Instruction::NoOp,
        _ => panic!(),
    }
}

fn parse_data(file_path: String) -> Vec<Instruction> {
    read_lines(file_path)
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .collect()
}

fn simulate(instructions: Vec<Instruction>) -> (i32, bool) {
    let mut handled: HashSet<usize> = HashSet::new();

    let mut instruction_index = 0;
    let mut accumulator = 0;

    while instruction_index < instructions.len() {
        if handled.contains(&instruction_index) {
            return (accumulator, false);
        }

        handled.insert(instruction_index);

        match instructions[instruction_index] {
            Instruction::Accumulate(value) => {
                accumulator += value;
                instruction_index += 1;
            }
            Instruction::Jump(value) => {
                instruction_index = (instruction_index as i32 + value) as usize;
            }
            Instruction::NoOp => {
                instruction_index += 1;
            }
        }
    }

    (accumulator, true)
}

pub fn part_1(file_path: String) -> i32 {
    let instructions = parse_data(file_path);
    let (accumulator, _) = simulate(instructions);
    accumulator
}

pub fn part_2(file_path: String) -> i32 {
    let instructions = parse_data(file_path);

    // Try to replace a jmp with a nop:
    for instruction_index in 0..instructions.len() {
        if !matches!(instructions[instruction_index], Instruction::Jump(_)) {
            continue;
        }

        let mut new_instructions = instructions.clone();
        new_instructions[instruction_index] = Instruction::NoOp;

        let (accumulator, successful_termination) = simulate(new_instructions);

        if successful_termination {
            return accumulator;
        }
    }

    // Never did end up having to try to replace a nop with a jmp...

    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 5)]
    #[case(false, 1475)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 8)));
    }

    #[rstest]
    #[case(true, 8)]
    #[case(false, 1270)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 8)));
    }
}
