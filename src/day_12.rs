use std::str;

use crate::utilities::file_utilities::read_lines;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug)]
enum Instruction {
    Direction(Direction, i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

fn parse_line(line: &str) -> Instruction {
    let (type_str, value_str) = line.split_at(1);
    let value = value_str.parse::<i32>().unwrap();
    match type_str {
        "N" => Instruction::Direction(Direction::North, value),
        "S" => Instruction::Direction(Direction::South, value),
        "E" => Instruction::Direction(Direction::East, value),
        "W" => Instruction::Direction(Direction::West, value),
        "L" => Instruction::Left(value),
        "R" => Instruction::Right(value),
        "F" => Instruction::Forward(value),
        _ => panic!(),
    }
}

fn parse_data(file_path: String) -> Vec<Instruction> {
    read_lines(file_path)
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .collect()
}

fn simulate_move_in_direction(
    state: &(i32, i32),
    direction: &Direction,
    amount: i32,
) -> (i32, i32) {
    match direction {
        Direction::North => (state.0, state.1 + amount),
        Direction::South => (state.0, state.1 - amount),
        Direction::East => (state.0 + amount, state.1),
        Direction::West => (state.0 - amount, state.1),
    }
}

fn simulate_single_instruction(
    state: &(Direction, (i32, i32)),
    instruction: &Instruction,
) -> (Direction, (i32, i32)) {
    match instruction {
        Instruction::Direction(direction, amount) => (
            state.0,
            simulate_move_in_direction(&state.1, direction, *amount),
        ),
        Instruction::Forward(amount) => (
            state.0,
            simulate_move_in_direction(&state.1, &state.0, *amount),
        ),
        Instruction::Left(amount) | Instruction::Right(amount) => {
            let rotations = match instruction {
                Instruction::Left(_) => vec![
                    Direction::East,
                    Direction::North,
                    Direction::West,
                    Direction::South,
                ],
                Instruction::Right(_) => vec![
                    Direction::East,
                    Direction::South,
                    Direction::West,
                    Direction::North,
                ],
                _ => panic!(),
            };

            let mut current_index = rotations.iter().position(|&d| d == state.0).unwrap();
            let mut degrees = *amount;

            while degrees > 0 {
                current_index = (current_index + 1) % 4;
                degrees -= 90;
            }
            (rotations[current_index], state.1)
        }
    }
}

pub fn part_1(file_path: String) -> i32 {
    let instructions = parse_data(file_path);
    let mut state = (Direction::East, (0, 0));

    for instruction in instructions.iter() {
        state = simulate_single_instruction(&state, instruction);
    }

    state.1 .0.abs() + state.1 .1.abs()
}

fn simulate_single_instruction_2(
    ship: &(i32, i32),
    waypoint: &(i32, i32),
    instruction: &Instruction,
) -> ((i32, i32), (i32, i32)) {
    match instruction {
        Instruction::Direction(direction, amount) => (
            *ship,
            simulate_move_in_direction(waypoint, direction, *amount),
        ),
        Instruction::Forward(amount) => (
            (ship.0 + amount * waypoint.0, ship.1 + amount * waypoint.1),
            *waypoint,
        ),
        Instruction::Left(amount) | Instruction::Right(amount) => {
            let theta = match instruction {
                Instruction::Left(_) => (*amount as f64).to_radians(),
                Instruction::Right(_) => -(*amount as f64).to_radians(),
                _ => panic!(),
            };

            let waypoint_x = (theta.cos() * waypoint.0 as f64) - (theta.sin() * waypoint.1 as f64);
            let waypoint_y = (theta.sin() * waypoint.0 as f64) + (theta.cos() * waypoint.1 as f64);

            (
                *ship,
                (waypoint_x.round() as i32, waypoint_y.round() as i32),
            )
        }
    }
}

pub fn part_2(file_path: String) -> i32 {
    let instructions = parse_data(file_path);
    let mut ship = (0, 0);
    let mut waypoint = (10, 1);
    for instruction in instructions.iter() {
        (ship, waypoint) = simulate_single_instruction_2(&ship, &waypoint, instruction);
    }

    ship.0.abs() + ship.1.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 25)]
    #[case(false, 904)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 12)));
    }

    #[rstest]
    #[case(true, 286)]
    #[case(false, 18747)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 12)));
    }
}
