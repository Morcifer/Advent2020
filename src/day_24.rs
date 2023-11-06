use crate::utilities::file_utilities::read_lines;

use num::complex::Complex32;
use rustc_hash::{FxHashMap, FxHashSet};
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

fn get_tile_configuration(flip_directions: Vec<String>) -> FxHashMap<HexComplex, Color> {
    let mut tiles: FxHashMap<HexComplex, Color> = FxHashMap::default();

    tiles.insert(HexComplex::new(0, 0), Color::White);

    for flip_direction in flip_directions.clone().into_iter() {
        let mut index = 0;
        let mut current_tile = HexComplex::new(0, 0);

        while index < flip_direction.len() {
            let new_direction =
                if let Some(direction) = get_direction_enum(&flip_direction[index..=index]) {
                    index += 1;
                    direction
                } else {
                    let direction = get_direction_enum(&flip_direction[index..=index + 1]).unwrap();
                    index += 2;
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
    tiles
}

pub fn part_1(file_path: String) -> i64 {
    let flip_directions: Vec<String> = read_lines(file_path);

    let tile_configuration = get_tile_configuration(flip_directions);

    tile_configuration
        .values()
        .filter(|v| **v == Color::Black)
        .count() as i64
}

fn get_neighbours(tile: &HexComplex) -> Vec<HexComplex> {
    [
        Neighbor::East,
        Neighbor::SouthEast,
        Neighbor::SouthWest,
        Neighbor::West,
        Neighbor::NorthWest,
        Neighbor::NorthEast,
    ]
    .into_iter()
    .map(|n| *tile + get_neighbour_complex(n))
    .collect::<Vec<_>>()
}

pub fn part_2(file_path: String) -> i64 {
    let flip_directions: Vec<String> = read_lines(file_path);
    let tile_configuration = get_tile_configuration(flip_directions);

    let mut black_tiles: FxHashSet<HexComplex> = tile_configuration
        .into_iter()
        .filter(|(_, v)| *v == Color::Black)
        .map(|(m, _)| m)
        .collect();

    for _day in 0..100 {
        // println!("Day {_day}: {}", black_tiles.len());

        let tile_to_investigate: FxHashSet<HexComplex> = black_tiles
            .iter()
            .flat_map(get_neighbours)
            .chain(black_tiles.iter().cloned())
            .collect();

        let still_black_tiles: Vec<_> = tile_to_investigate
            .iter()
            .filter(|tile| black_tiles.contains(tile))
            .filter(|tile| {
                let active_neighbours_count = get_neighbours(tile)
                    .iter()
                    .filter(|neighbour| black_tiles.contains(neighbour))
                    .count();
                (1..=2).contains(&active_neighbours_count)
            })
            .cloned()
            .collect();

        let new_black_tiles: Vec<_> = tile_to_investigate
            .iter()
            .filter(|tile| !black_tiles.contains(tile))
            .filter(|tile| {
                let active_neighbours_count = get_neighbours(tile)
                    .iter()
                    .filter(|neighbour| black_tiles.contains(neighbour))
                    .count();
                active_neighbours_count == 2
            })
            .cloned()
            .collect();

        black_tiles = still_black_tiles
            .into_iter()
            .chain(new_black_tiles.into_iter())
            .collect();
    }

    black_tiles.len() as i64
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
    #[case(true, 2208)]
    #[case(false, 3445)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 24, None)));
    }
}
