use itertools::Itertools;

use crate::utilities::file_utilities::read_lines;
use rustc_hash::{FxHashMap, FxHashSet};

type Tile = (isize, Vec<Vec<Pixel>>);

const TILE_SIZE: usize = 10;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Pixel {
    On,
    #[default]
    Off,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Edge {
    Top,
    Bottom,
    Right,
    Left,
}

#[derive(Clone, Copy, Debug, Default)]
enum Rotation {
    #[default]
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[derive(Clone, Copy, Debug, Default)]
enum Flip {
    #[default]
    No,
    ColumnWise,
    RowWise,
}

type Orientation = (Rotation, Flip);

fn parse_cluster(line: String) -> Tile {
    let cluster: Vec<&str> = line.split(' ').collect();
    let tile_id = cluster[1][0..4].parse::<isize>().unwrap();
    let tile_data = &cluster[2..];
    // println!("{tile_data:?}");

    let mut pixels: Vec<Vec<Pixel>> = vec![vec![Default::default(); TILE_SIZE]; TILE_SIZE];

    for (i, row) in tile_data.iter().enumerate() {
        for (j, char) in row.chars().enumerate() {
            let pixel = match char {
                '#' => Pixel::On,
                '.' => Pixel::Off,
                _ => panic!("Something's wrong with this pixel"),
            };
            pixels[i][j] = pixel;
        }
    }
    // println!("{on_pixels:?}");

    (tile_id, pixels)
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

fn rotated_row_column_by_ninety(row: usize, column: usize) -> (usize, usize) {
    (TILE_SIZE - 1 - column, row)
}

fn get_pixel_on_tile(tile: &Tile, orientation: &Orientation, row: usize, column: usize) -> Pixel {
    let (rotated_row, rotated_column) = match orientation.0 {
        Rotation::Zero => (row, column),
        Rotation::Ninety => rotated_row_column_by_ninety(row, column),
        Rotation::OneEighty => {
            let (new_row, new_column) = rotated_row_column_by_ninety(row, column);
            rotated_row_column_by_ninety(new_row, new_column)
        }
        Rotation::TwoSeventy => {
            let (new_row, new_column) = rotated_row_column_by_ninety(row, column);
            let (new_new_row, new_new_column) = rotated_row_column_by_ninety(new_row, new_column);
            rotated_row_column_by_ninety(new_new_row, new_new_column)
        }
    };

    let (flipped_row, flipped_column) = match orientation.1 {
        Flip::No => (rotated_row, rotated_column),
        Flip::ColumnWise => (rotated_row, TILE_SIZE - 1 - rotated_column),
        Flip::RowWise => (TILE_SIZE - 1 - rotated_row, rotated_column),
    };

    tile.1[flipped_row][flipped_column]
}

fn get_edge(tile: &Tile, edge: &Edge, orientation: &Orientation) -> [Pixel; TILE_SIZE] {
    let pixels = match edge {
        Edge::Top => (0..TILE_SIZE)
            .map(|column| get_pixel_on_tile(tile, orientation, 0, column))
            .collect::<Vec<_>>(),
        Edge::Bottom => (0..TILE_SIZE)
            .map(|column| get_pixel_on_tile(tile, orientation, TILE_SIZE - 1, column))
            .collect::<Vec<_>>(),
        Edge::Left => (0..TILE_SIZE)
            .map(|row| get_pixel_on_tile(tile, orientation, row, 0))
            .collect::<Vec<_>>(),
        Edge::Right => (0..TILE_SIZE)
            .map(|row| get_pixel_on_tile(tile, orientation, row, TILE_SIZE - 1))
            .collect::<Vec<_>>(),
    };

    pixels.as_slice().try_into().unwrap()
}

fn tiles_match(
    tile_1: &Tile,
    tile_1_orientation: &Orientation,
    tile_2: &Tile,
    preferred_tile_1_edge: Option<&Edge>,
    preferred_tile_2_edge: Option<&Edge>,
) -> Option<(Edge, Edge, Orientation)> {
    for tile_1_edge in [Edge::Top, Edge::Bottom, Edge::Right, Edge::Left] {
        let tile_1_edge = preferred_tile_1_edge.unwrap_or(&tile_1_edge);
        let tile_1_edge_indices = get_edge(tile_1, tile_1_edge, tile_1_orientation);

        for tile_2_edge in [Edge::Top, Edge::Bottom, Edge::Right, Edge::Left] {
            let tile_2_edge = preferred_tile_2_edge.unwrap_or(&tile_2_edge);

            let rotations = [
                Rotation::Zero,
                Rotation::Ninety,
                Rotation::OneEighty,
                Rotation::TwoSeventy,
            ];
            let flips = [Flip::No, Flip::RowWise, Flip::ColumnWise];

            for tile_2_orientation in rotations.iter().cartesian_product(flips.iter()) {
                let tile_2_orientation = (*tile_2_orientation.0, *tile_2_orientation.1);
                let tile_2_edge_indices = get_edge(tile_2, tile_2_edge, &tile_2_orientation);

                // Am I choosing the easy way out, and should actually get multiple of these?
                if tile_1_edge_indices == tile_2_edge_indices {
                    return Some((*tile_1_edge, *tile_2_edge, tile_2_orientation));
                }
            }
        }
    }

    None
}

struct PuzzlePieces {
    corners: FxHashMap<isize, Vec<isize>>,
    edges: FxHashMap<isize, Vec<isize>>,
    insides: FxHashMap<isize, Vec<isize>>,
}

fn categorize_tiles(tiles: &[Tile]) -> PuzzlePieces {
    let mut corners: FxHashMap<isize, Vec<isize>> = FxHashMap::default();
    let mut edges: FxHashMap<isize, Vec<isize>> = FxHashMap::default();
    let mut insides: FxHashMap<isize, Vec<isize>> = FxHashMap::default();

    for tile_1 in tiles.iter() {
        let matching_tiles: Vec<isize> = tiles
            .iter()
            .filter(|tile_2| tile_1.0 != tile_2.0)
            .filter_map(|tile_2| {
                tiles_match(tile_1, &(Rotation::Zero, Flip::No), tile_2, None, None)
                    .map(|_| tile_2.0)
            })
            .collect();

        // println!(
        //     "Tile {} has {} matching tiles",
        //     tile_1.0,
        //     matching_tiles.len()
        // );

        match matching_tiles.len() {
            2 => corners.insert(tile_1.0, matching_tiles),
            3 => edges.insert(tile_1.0, matching_tiles),
            4 => insides.insert(tile_1.0, matching_tiles),
            _ => panic!("The data or the algorithm is faulty."),
        };
    }

    PuzzlePieces {
        corners,
        edges,
        insides,
    }
}

pub fn part_1(file_path: String) -> i64 {
    let tiles = parse_data(file_path);

    let puzzle_pieces = categorize_tiles(&tiles);

    if puzzle_pieces.corners.len() != 4 {
        panic!("Your algorithm is faulty, you're making wrong assumptions.");
    }

    puzzle_pieces.corners.keys().map(|i| *i as i64).product()
}

pub fn part_2(file_path: String) -> i64 {
    let tiles = parse_data(file_path);
    let tiles_by_id: FxHashMap<isize, &Tile> = tiles.iter().map(|tile| (tile.0, tile)).collect();

    let puzzle_pieces = categorize_tiles(&tiles);
    let puzzle_size = (tiles.len() as f64).sqrt().round() as usize;

    let all_edges_and_corners_pieces = puzzle_pieces
        .corners
        .iter()
        .chain(puzzle_pieces.edges.iter())
        .collect::<FxHashMap<_, _>>();

    let all_neighbours = puzzle_pieces
        .corners
        .iter()
        .chain(puzzle_pieces.edges.iter())
        .chain(puzzle_pieces.insides.iter())
        .collect::<FxHashMap<_, _>>();

    // println!("{all_neighbours:?}");

    // Start with the top left corner -
    // the one that has some rotation in which it has a bottom edge and right edge neighbour...
    let all_corners = puzzle_pieces.corners.keys().collect::<Vec<_>>();
    // println!("{all_corners:?}");

    for first_corner in all_corners.iter() {
        let rotations = [
            Rotation::Zero,
            Rotation::Ninety,
            Rotation::OneEighty,
            Rotation::TwoSeventy,
        ];
        let flips = [Flip::No, Flip::RowWise, Flip::ColumnWise];

        for first_corner_rotation in rotations.iter().cartesian_product(flips.iter()) {
            let mut pieces: Vec<Vec<isize>> =
                vec![vec![Default::default(); puzzle_size]; puzzle_size];
            let mut orientations: Vec<Vec<Orientation>> =
                vec![vec![Default::default(); puzzle_size]; puzzle_size];

            // println!("Trying {first_corner} as first corner with rotation {first_corner_rotation:?}");
            let first_corner_rotation = (*first_corner_rotation.0, *first_corner_rotation.1);
            let first_corner_friends = puzzle_pieces.corners.get(first_corner).unwrap().to_vec();

            let temp = first_corner_friends
                .iter()
                .filter_map(|friend| {
                    let matching = tiles_match(
                        tiles_by_id.get(first_corner).unwrap(),
                        &first_corner_rotation,
                        tiles_by_id.get(friend).unwrap(),
                        Some(&Edge::Right),
                        Some(&Edge::Left),
                    );

                    matching.map(|matching| (friend, matching.0, matching.1, matching.2))
                })
                .next();

            if temp.is_none() {
                continue;
            }

            let (first_first_corner_friend, _, _, first_first_corner_friend_rotation) =
                temp.unwrap();

            pieces[0][0] = **first_corner;
            orientations[0][0] = first_corner_rotation;

            pieces[0][1] = *first_first_corner_friend;
            orientations[0][1] = first_first_corner_friend_rotation;

            let mut handled: FxHashSet<isize> = vec![**first_corner, *first_first_corner_friend]
                .into_iter()
                .collect();

            // Fill up the puzzle from top to bottom, from left to right.
            let mut stop_solving = false;

            for row in 0..puzzle_size {
                if stop_solving {
                    break;
                }

                for column in 0..puzzle_size {
                    if stop_solving {
                        break;
                    }

                    if row == 0 && column < 2 {
                        continue;
                    }

                    let (neighbour_tile_id, neighbour_tile_rotation, tile_edge, neighbour_edge) =
                        if row == 0 {
                            (
                                pieces[0][column - 1],
                                orientations[0][column - 1],
                                Some(&Edge::Right),
                                Some(&Edge::Left),
                            )
                        } else {
                            (
                                pieces[row - 1][column],
                                orientations[row - 1][column],
                                Some(&Edge::Bottom),
                                Some(&Edge::Top),
                            )
                        };

                    let important_subset = if row == 0 {
                        &all_edges_and_corners_pieces
                    } else {
                        &all_neighbours
                    };

                    let result = all_neighbours
                        .get(&neighbour_tile_id)
                        .unwrap()
                        .iter()
                        .filter(|n| important_subset.contains_key(n) && !handled.contains(n))
                        .filter_map(|piece| {
                            let tile_match = tiles_match(
                                tiles_by_id.get(&neighbour_tile_id).unwrap(),
                                &neighbour_tile_rotation,
                                tiles_by_id.get(piece).unwrap(),
                                tile_edge,
                                neighbour_edge,
                            );

                            tile_match.map(|tile_match| (piece, tile_match.2))
                        })
                        .next();

                    if result.is_none() {
                        stop_solving = true;
                        break;
                    }

                    let (next_piece, next_piece_orientation) = result.unwrap();

                    pieces[row][column] = *next_piece;
                    orientations[row][column] = next_piece_orientation;

                    handled.insert(*next_piece);
                }
            }

            if stop_solving {
                // Dead end
                continue;
            }

            // Now look for rotation and construct image.
            let whole_picture_size = puzzle_size * (TILE_SIZE - 2);
            let mut picture_pixels: Vec<Vec<Pixel>> =
                vec![vec![Default::default(); whole_picture_size]; whole_picture_size];

            let inner_tile_size = TILE_SIZE - 2;

            for tile_row in 0..puzzle_size {
                for tile_column in 0..puzzle_size {
                    let tile_id = pieces[tile_row][tile_column];
                    let tile_orientation = orientations[tile_row][tile_column];
                    let tile_pixes = tiles_by_id.get(&tile_id).unwrap();

                    for pixel_row in 0..inner_tile_size {
                        for pixel_column in 0..inner_tile_size {
                            let pixel = get_pixel_on_tile(
                                tile_pixes,
                                &tile_orientation,
                                pixel_row + 1,
                                pixel_column + 1,
                            );

                            // println!("Tile: {tile_row}, {tile_column}");
                            // println!("Pixel: {pixel_row}, {pixel_column}");
                            // println!("Tile Pixel: {tile_pixel_row}, {tile_pixel_column}");
                            // println!("Location in total: {}, {}", tile_row*inner_tile_size + pixel_row, tile_column*inner_tile_size + pixel_column);

                            picture_pixels[tile_row * inner_tile_size + pixel_row]
                                [tile_column * inner_tile_size + pixel_column] = pixel;
                        }
                    }
                }
            }

            // println!("{picture_pixels:?}");
            // if first_corner == 1951 {
            //     for row in picture_pixels.iter() {
            //         let print = row
            //             .iter()
            //             .map(|pixel| {
            //                 if *pixel == Pixel::Off {
            //                     ".".to_string()
            //                 } else {
            //                     "#".to_string()
            //                 }
            //             })
            //             .join("");
            //         println!("{print:?}");
            //     }
            // }

            let mut monsters = 0;
            let mut monster_pixels = FxHashSet::default();
            let monster_offsets = [
                (0, 18),
                (1, 0),
                (1, 5),
                (1, 6),
                (1, 11),
                (1, 12),
                (1, 17),
                (1, 18),
                (1, 19),
                (2, 1),
                (2, 4),
                (2, 7),
                (2, 10),
                (2, 13),
                (2, 16),
            ];

            for row_index in 0..whole_picture_size {
                for column_index in 0..whole_picture_size {
                    let fits_monster = monster_offsets.iter().all(|(row_offset, column_offset)| {
                        let row_in_picture = row_offset + row_index;
                        let column_in_picture = column_offset + column_index;

                        if !(0..whole_picture_size).contains(&row_in_picture)
                            || !(0..whole_picture_size).contains(&column_in_picture)
                        {
                            return false;
                        }

                        picture_pixels[row_offset + row_index][column_offset + column_index]
                            == Pixel::On
                    });

                    if fits_monster {
                        println!("Monster at ({row_index}, {column_index})");
                        monsters += 1;

                        monster_pixels.extend(monster_offsets.iter().map(
                            |(row_offset, column_offset)| {
                                (row_offset + row_index, column_offset + column_index)
                            },
                        ));
                    }
                }
            }

            if monsters > 0 {
                println!("Found monsters in this map:");

                println!("{pieces:?}");
                println!("{orientations:?}");

                for row in picture_pixels.iter() {
                    let print = row
                        .iter()
                        .map(|pixel| {
                            if *pixel == Pixel::Off {
                                ".".to_string()
                            } else {
                                "#".to_string()
                            }
                        })
                        .join("");
                    println!("{print:?}");
                }

                // Don't figure out monsters. Figure out #'s without monsters, you idiot!!
                let hashtag_total: usize = picture_pixels
                    .iter()
                    .map(|row| row.iter().filter(|pixel| **pixel == Pixel::On).count())
                    .sum();
                let hashtag_monster = monster_pixels.len();
                return (hashtag_total - hashtag_monster) as i64;
            }
        }
    }

    println!("I failed...");
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
    #[case(true, 273)]
    #[case(false, 1841)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 20, None)));
    }
}
