use itertools::Itertools;

use crate::utilities::file_utilities::read_lines;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::day_20::Pixel::Off;

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
    FlipColumnWise,
    FlipRowWise,
}

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

fn get_edge(tile: &Tile, edge: &Edge, rotation: &Rotation) -> [Pixel; TILE_SIZE] {
    let (filter_row, filter_column, flip) = match (edge, rotation) {
        (Edge::Top, Rotation::Zero) => (Some(0), None, false),
        (Edge::Top, Rotation::Ninety) => (None, Some(0), true),
        (Edge::Top, Rotation::OneEighty) => (Some(9), None, true),
        (Edge::Top, Rotation::TwoSeventy) => (None, Some(9), false),
        (Edge::Top, Rotation::FlipRowWise) => (Some(9), None, false),
        (Edge::Top, Rotation::FlipColumnWise) => (Some(0), None, true),
        (Edge::Bottom, Rotation::Zero) => (Some(9), None, false),
        (Edge::Bottom, Rotation::Ninety) => (None, Some(9), true),
        (Edge::Bottom, Rotation::OneEighty) => (Some(0), None, true),
        (Edge::Bottom, Rotation::TwoSeventy) => (None, Some(0), false),
        (Edge::Bottom, Rotation::FlipRowWise) => (Some(0), None, false),
        (Edge::Bottom, Rotation::FlipColumnWise) => (Some(9), None, true),
        (Edge::Right, Rotation::Zero) => (None, Some(0), false),
        (Edge::Right, Rotation::Ninety) => (Some(0), None, false),
        (Edge::Right, Rotation::OneEighty) => (None, Some(9), true),
        (Edge::Right, Rotation::TwoSeventy) => (Some(9), None, true),
        (Edge::Right, Rotation::FlipRowWise) => (None, Some(0), true),
        (Edge::Right, Rotation::FlipColumnWise) => (None, Some(9), false),
        (Edge::Left, Rotation::Zero) => (None, Some(9), false),
        (Edge::Left, Rotation::Ninety) => (Some(9), None, false),
        (Edge::Left, Rotation::OneEighty) => (None, Some(0), true),
        (Edge::Left, Rotation::TwoSeventy) => (Some(0), None, true),
        (Edge::Left, Rotation::FlipRowWise) => (None, Some(9), true),
        (Edge::Left, Rotation::FlipColumnWise) => (None, Some(0), false),
    };

    let non_flipped: Vec<Pixel> = match (filter_row, filter_column) {
        (Some(filter_row), None) => tile.1[filter_row].to_vec(),
        (None, Some(filter_column)) => tile.1.iter().map(|row| row[filter_column]).collect(),
        _ => panic!("How did you manage to do that?"),
    };

    if flip {
        non_flipped
            .into_iter()
            .rev()
            .collect::<Vec<Pixel>>()
            .as_slice()
            .try_into()
            .unwrap()
    } else {
        non_flipped
            .into_iter()
            .collect::<Vec<Pixel>>()
            .as_slice()
            .try_into()
            .unwrap()
    }
}

fn tiles_match(
    tile_1: &Tile,
    tile_1_rotation: &Rotation,
    tile_2: &Tile,
) -> Option<(Edge, Edge, Rotation)> {
    for tile_1_edge in [Edge::Top, Edge::Bottom, Edge::Right, Edge::Left] {
        let tile_1_edge_indices = get_edge(tile_1, &tile_1_edge, tile_1_rotation);

        for tile_2_edge in [Edge::Top, Edge::Bottom, Edge::Right, Edge::Left] {
            for tile_2_rotation in [
                Rotation::Zero,
                Rotation::Ninety,
                Rotation::OneEighty,
                Rotation::TwoSeventy,
                Rotation::FlipColumnWise,
                Rotation::FlipRowWise,
            ] {
                let tile_2_edge_indices = get_edge(tile_2, &tile_2_edge, &tile_2_rotation);

                if tile_1_edge_indices == tile_2_edge_indices {
                    return Some((tile_1_edge, tile_2_edge, tile_2_rotation));
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
            .filter_map(|tile_2| tiles_match(tile_1, &Rotation::Zero, tile_2).map(|_| tile_2.0))
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
    // Now we get to the good part.
    let tiles = parse_data(file_path);
    let tiles_by_id: FxHashMap<isize, &Tile> = tiles.iter().map(|tile| (tile.0, tile)).collect();

    let puzzle_pieces = categorize_tiles(&tiles);

    let puzzle_size = (tiles.len() as f64).sqrt().round() as usize;
    // println!("{puzzle_size:?}");

    let mut pieces: Vec<Vec<isize>> = vec![vec![Default::default(); puzzle_size]; puzzle_size];
    let mut rotations: Vec<Vec<Rotation>> =
        vec![vec![Default::default(); puzzle_size]; puzzle_size];

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
    println!("{all_corners:?}");

    let (first_corner, first_corner_rotation) = all_corners
        .iter()
        .flat_map(|corner| {
            let friends = puzzle_pieces.corners.get(corner).unwrap().to_vec();
            [
                Rotation::Zero,
                Rotation::Ninety,
                Rotation::OneEighty,
                Rotation::TwoSeventy,
                Rotation::FlipColumnWise,
                Rotation::FlipRowWise,
            ]
            .iter()
            .map(|rotation| {
                let matching_edges = friends
                    .iter()
                    .map(|friend| {
                        tiles_match(
                            tiles_by_id.get(corner).unwrap(),
                            rotation,
                            tiles_by_id.get(friend).unwrap(),
                        )
                        .unwrap()
                        .0
                    })
                    .collect::<Vec<_>>();

                // println!("Corner {corner} with rotation {rotation:?} - {matching_edges:?}");
                (**corner, rotation, matching_edges)
            })
            .collect::<Vec<_>>()
        })
        .filter(|(_, _, matching_edges)| {
            *matching_edges == [Edge::Bottom, Edge::Right]
                || *matching_edges == [Edge::Right, Edge::Bottom]
        })
        .map(|(corner, rotation, _)| (corner, *rotation))
        .next()
        .unwrap();

    let first_corner_friends = puzzle_pieces.corners.get(&first_corner).unwrap().to_vec();
    let first_first_corner_friend = first_corner_friends[0];

    pieces[0][0] = first_corner;
    rotations[0][0] = first_corner_rotation;
    pieces[0][1] = first_first_corner_friend;
    rotations[0][1] = tiles_match(
        tiles_by_id.get(&first_corner).unwrap(),
        &rotations[0][0],
        tiles_by_id.get(&pieces[0][1]).unwrap(),
    )
    .unwrap()
    .2;

    let mut handled: FxHashSet<isize> = vec![first_corner, first_first_corner_friend]
        .into_iter()
        .collect();

    // Fill up the puzzle from top to bottom, from left to right.
    for row in 0..puzzle_size {
        for column in 0..puzzle_size {
            if row == 0 && column < 2 {
                continue;
            }

            let (neighbour_tile_id, neighbour_tile_rotation) = if row == 0 {
                (pieces[0][column - 1], rotations[0][column - 1])
            } else {
                (pieces[row - 1][column], rotations[row - 1][column])
            };

            let important_subset = if row == 0 {
                &all_edges_and_corners_pieces
            } else {
                &all_neighbours
            };

            let next_piece = all_neighbours
                .get(&neighbour_tile_id)
                .unwrap()
                .iter()
                .find(|n| important_subset.contains_key(n) && !handled.contains(n))
                .unwrap();

            pieces[row][column] = *next_piece;
            rotations[row][column] = tiles_match(
                tiles_by_id.get(&neighbour_tile_id).unwrap(),
                &neighbour_tile_rotation,
                tiles_by_id.get(&pieces[row][column]).unwrap(),
            )
            .unwrap()
            .2;

            handled.insert(*next_piece);
        }
    }

    // Now look for rotation and construct image.
    let whole_picture_size = puzzle_size * (TILE_SIZE - 2);
    let mut picture_pixels: Vec<Vec<Pixel>> =
        vec![vec![Default::default(); whole_picture_size]; whole_picture_size];

    println!("{pieces:?}");
    println!("{rotations:?}");

    let inner_tile_size = TILE_SIZE - 2;

    for tile_row in 0..puzzle_size {
        for tile_column in 0..puzzle_size {
            let tile_id = pieces[tile_row][tile_column];
            let tile_rotation = rotations[tile_row][tile_column];
            let tile_pixes = tiles_by_id.get(&tile_id).unwrap();

            for pixel_row in 0..inner_tile_size {
                for pixel_column in 0..inner_tile_size {
                    let (tile_pixel_row, tile_pixel_column) = (pixel_row + 1, pixel_column + 1);

                    let (tile_pixel_row, tile_pixel_column) = match tile_rotation {
                        Rotation::Zero => (tile_pixel_row, tile_pixel_column),
                        Rotation::Ninety => (TILE_SIZE - tile_pixel_column - 1, tile_pixel_row),
                        Rotation::OneEighty => (
                            TILE_SIZE - tile_pixel_row - 1,
                            TILE_SIZE - tile_pixel_column - 1,
                        ),
                        Rotation::TwoSeventy => (tile_pixel_column, TILE_SIZE - tile_pixel_row - 1),
                        Rotation::FlipColumnWise => (tile_pixel_row, TILE_SIZE - tile_pixel_column - 1),
                        Rotation::FlipRowWise => (TILE_SIZE - tile_pixel_row - 1, tile_pixel_column),
                    };

                    // println!("Tile: {tile_row}, {tile_column}");
                    // println!("Pixel: {pixel_row}, {pixel_column}");
                    // println!("Tile Pixel: {tile_pixel_row}, {tile_pixel_column}");
                    // println!("Location in total: {}, {}", tile_row*inner_tile_size + pixel_row, tile_column*inner_tile_size + pixel_column);

                    picture_pixels[tile_row * inner_tile_size + pixel_row]
                        [tile_column * inner_tile_size + pixel_column] =
                        tile_pixes.1[tile_pixel_row][tile_pixel_column];
                }
            }
        }
    }

    // println!("{picture_pixels:?}");

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

    let mut monsters = 0;
    let monster_offsets = [
        (0, 18),
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

    for row_index in 0..whole_picture_size-3 {
        for column_index in 0..whole_picture_size-20 {
            let fits_monster = monster_offsets
                .iter()
                .all(|(row_offset, column_offset)| {
                    picture_pixels[row_offset+row_index][column_offset+column_index] == Pixel::On
                });

            if fits_monster {
                monsters += 1;
            }
        }
    }

    monsters
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
