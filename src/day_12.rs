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

pub fn part_1(file_path: String) -> i32 {
    let instructions = parse_data(file_path);
    let mut ship_direction = Direction::East;
    let mut ship_point = (0, 0);

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Direction(direction, amount) => {
                ship_point = simulate_move_in_direction(&ship_point, direction, *amount);
            }
            Instruction::Forward(amount) => {
                ship_point = simulate_move_in_direction(&ship_point, &ship_direction, *amount);
            }
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

                let old_index = rotations.iter().position(|&d| d == ship_direction).unwrap();
                let new_index = (((*amount as f64 / 90.0).round() as usize) + old_index) % 4;

                ship_direction = rotations[new_index];
            }
        }
    }

    ship_point.0.abs() + ship_point.1.abs()
}

pub fn part_2(file_path: String) -> i32 {
    let instructions = parse_data(file_path);
    let mut ship = (0, 0);
    let mut waypoint = (10, 1);

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Direction(direction, amount) => {
                waypoint = simulate_move_in_direction(&waypoint, direction, *amount);
            }
            Instruction::Forward(amount) => {
                ship = (ship.0 + amount * waypoint.0, ship.1 + amount * waypoint.1);
            }
            Instruction::Left(amount) | Instruction::Right(amount) => {
                let theta = match instruction {
                    Instruction::Left(_) => (*amount as f64).to_radians(),
                    Instruction::Right(_) => -(*amount as f64).to_radians(),
                    _ => panic!(),
                };

                let waypoint_x = (theta.cos().round() as i32 * waypoint.0)
                    - (theta.sin().round() as i32 * waypoint.1);
                let waypoint_y = (theta.sin().round() as i32 * waypoint.0)
                    + (theta.cos().round() as i32 * waypoint.1);

                waypoint = (waypoint_x, waypoint_y);
            }
        }
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
        assert_eq!(expected, part_1(get_file_path(is_test, 12, None)));
    }

    #[rstest]
    #[case(true, 286)]
    #[case(false, 18747)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 12, None)));
    }
}
