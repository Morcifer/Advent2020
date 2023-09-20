use itertools::Itertools;

use crate::utilities::file_utilities::read_lines;
use std::collections::HashSet;

type Tile = (isize, Vec<(isize, isize)>);

enum Edge {
    Top,
    Bottom,
    Right,
    Left,
}

enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

fn parse_cluster(line: String) -> Tile {
    let cluster: Vec<&str> = line.split(' ').collect();
    let tile_id = cluster[1][0..4].parse::<isize>().unwrap();
    let tile_data = &cluster[2..];
    // println!("{tile_data:?}");

    let mut on_pixels: Vec<(isize, isize)> = vec![];
    for (i, row) in tile_data.iter().enumerate() {
        for (j, char) in row.chars().enumerate() {
            if char == '#' {
                on_pixels.push((i as isize, j as isize));
            }
        }
    }
    // println!("{on_pixels:?}");

    (tile_id, on_pixels)
}

fn parse_data(file_path: String) -> Vec<Tile> {
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
        .map(parse_cluster)
        .collect()
}

fn get_edge(tile: &Tile, edge: &Edge, rotation: &Rotation) -> HashSet<isize> {
    let (filter_row, filter_column, flip) = match (edge, rotation) {
        (Edge::Top, Rotation::Zero) => (Some(0), None, false),
        (Edge::Top, Rotation::Ninety) => (None, Some(0), true),
        (Edge::Top, Rotation::OneEighty) => (Some(9), None, true),
        (Edge::Top, Rotation::TwoSeventy) => (None, Some(9), false),
        (Edge::Bottom, Rotation::Zero) => (Some(9), None, false),
        (Edge::Bottom, Rotation::Ninety) => (None, Some(9), true),
        (Edge::Bottom, Rotation::OneEighty) => (Some(0), None, true),
        (Edge::Bottom, Rotation::TwoSeventy) => (None, Some(0), false),
        (Edge::Right, Rotation::Zero) => (None, Some(0), false),
        (Edge::Right, Rotation::Ninety) => (Some(0), None, false),
        (Edge::Right, Rotation::OneEighty) => (None, Some(9), true),
        (Edge::Right, Rotation::TwoSeventy) => (Some(9), None, true),
        (Edge::Left, Rotation::Zero) => (None, Some(9), false),
        (Edge::Left, Rotation::Ninety) => (Some(9), None, false),
        (Edge::Left, Rotation::OneEighty) => (None, Some(0), true),
        (Edge::Left, Rotation::TwoSeventy) => (Some(0), None, true),
    };

    let non_flipped = match (filter_row, filter_column) {
        (Some(filter_row), None) => tile
            .1
            .iter()
            .filter_map(|(row_index, column_index)| {
                if *row_index == filter_row {
                    Some(column_index)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>(),
        (None, Some(filter_column)) => tile
            .1
            .iter()
            .filter_map(|(row_index, column_index)| {
                if *column_index == filter_column {
                    Some(row_index)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>(),
        _ => panic!("How did you manage to do that?"),
    };

    if flip {
        non_flipped.into_iter().map(|i| 9 - i).collect()
    } else {
        non_flipped.into_iter().copied().collect()
    }
}

pub fn tiles_match(tile_1: &Tile, tile_2: &Tile) -> bool {
    for tile_1_edge in [Edge::Top, Edge::Bottom, Edge::Right, Edge::Left] {
        let tile_1_edge = get_edge(tile_1, &tile_1_edge, &Rotation::Zero);

        for tile_2_edge in [Edge::Top, Edge::Bottom, Edge::Right, Edge::Left] {
            for tile_2_rotation in [
                Rotation::Zero,
                Rotation::Ninety,
                Rotation::OneEighty,
                Rotation::TwoSeventy,
            ] {
                let tile_2_edge = get_edge(tile_2, &tile_2_edge, &tile_2_rotation);

                if tile_1_edge == tile_2_edge {
                    return true;
                }
            }
        }
    }

    false
}

pub fn part_1(file_path: String) -> i64 {
    let tiles = parse_data(file_path);
    println!("{tiles:?}");

    let frame_size = (tiles.len() as f64).sqrt().round() as i32;
    println!("{frame_size:?}");

    let mut corner_pieces_maybe = vec![];

    for tile_1 in tiles.iter() {
        let matching_tiles = tiles
            .iter()
            .filter(|tile_2| tiles_match(tile_1, tile_2))
            .count();

        println!("Tile {} has {matching_tiles} matching tiles", tile_1.0);

        if matching_tiles == 3 {
            corner_pieces_maybe.push(tile_1.0)
        }
    }

    if corner_pieces_maybe.len() != 4 {
        panic!("Your algorithm is faulty, you're making wrong assumptions.");
    }

    corner_pieces_maybe.into_iter().map(|i| i as i64).product()
}

pub fn part_2(file_path: String) -> i64 {
    // Now we get to the good part.
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 20899048083289)]
    #[case(false, 8425574315321)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 20, None)));
    }

    #[rstest]
    #[case(true, 0)]
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 20, None)));
    }
}
