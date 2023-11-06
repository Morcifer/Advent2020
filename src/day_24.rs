use crate::utilities::file_utilities::read_lines;

use num::complex::Complex32;
use rustc_hash::FxHashMap;
use std::hash::{Hash, Hasher};
use std::ops;

#[derive(Copy, Clone, Debug)]
struct HexComplex(Complex32);

impl HexComplex {
    pub fn new(real: i32, imaginary: i32) -> Self {
        Self(Complex32::new(real as f32, imaginary as f32))
    }
}

#[allow(clippy::derived_hash_with_manual_eq)]
impl Hash for HexComplex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.0.re.round() as i32);
        state.write_i32(self.0.im.round() as i32);
    }
}

impl PartialEq for HexComplex {
    fn eq(&self, other: &Self) -> bool {
        self.0.re.round() == other.0.re.round() && self.0.im.round() == other.0.im.round()
    }
}

impl Eq for HexComplex {}

impl ops::Add<Self> for HexComplex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Debug, Hash, PartialEq)]
enum Neighbor {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn get_neighbour_complex(direction: Neighbor) -> HexComplex {
    match direction {
        Neighbor::East => HexComplex::new(1, 0),
        Neighbor::SouthEast => HexComplex::new(1, -1),
        Neighbor::SouthWest => HexComplex::new(0, -1),
        Neighbor::West => HexComplex::new(-1, 0),
        Neighbor::NorthWest => HexComplex::new(-1, 1),
        Neighbor::NorthEast => HexComplex::new(0, 1),
    }
}

fn get_direction_enum(direction: &str) -> Option<Neighbor> {
    match direction {
        "e" => Some(Neighbor::East),
        "se" => Some(Neighbor::SouthEast),
        "sw" => Some(Neighbor::SouthWest),
        "w" => Some(Neighbor::West),
        "nw" => Some(Neighbor::NorthWest),
        "ne" => Some(Neighbor::NorthEast),
        _ => None,
    }
}

pub fn part_1(file_path: String) -> i64 {
    let flip_directions: Vec<String> = read_lines(file_path);
    let mut tiles: FxHashMap<HexComplex, Color> = FxHashMap::default();

    tiles.insert(HexComplex::new(0, 0), Color::White);

    for flip_direction in flip_directions.clone().into_iter() {
        println!("Working for {flip_direction}");
        let mut instruction_index = 0;
        let mut current_tile = HexComplex::new(0, 0);

        while instruction_index < flip_direction.len() {
            println!("At tile {current_tile:?}");

            let new_direction = if let Some(direction) =
                get_direction_enum(&flip_direction[instruction_index..=instruction_index])
            {
                instruction_index += 1;
                direction
            } else {
                let direction =
                    get_direction_enum(&flip_direction[instruction_index..=instruction_index + 1])
                        .unwrap();
                instruction_index += 2;
                direction
            };

            current_tile = current_tile + get_neighbour_complex(new_direction);
            tiles.entry(current_tile).or_insert(Color::White);
        }

        let color = tiles[&current_tile].clone();
        match color {
            Color::White => tiles.insert(current_tile, Color::Black),
            Color::Black => tiles.insert(current_tile, Color::White),
        };
    }

    tiles.values().filter(|v| **v == Color::Black).count() as i64
}

pub fn part_2(file_path: String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 10)]
    #[case(false, 282)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 24, None)));
    }

    #[rstest]
    #[case(true, 0)]
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 24, None)));
    }
}
