use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::utilities::file_utilities::read_lines;

fn get_all_seats(inputs: &Vec<String>) -> (FxHashSet<(isize, isize)>, isize, isize) {
    let height = inputs.len();
    let width = inputs[0].len();
    let mut seats = FxHashSet::default();

    for (row, column) in (0..height).cartesian_product(0..width) {
        if inputs[row][column..=column].eq("L") {
            seats.insert((row as isize, column as isize));
        }
    }

    (seats, height as isize, width as isize)
}

pub fn part_1(file_path: String) -> usize {
    let map: Vec<String> = read_lines(file_path);
    let (seats, height, width) = get_all_seats(&map);

    let mut filled_seats: FxHashSet<(isize, isize)> = FxHashSet::default();

    loop {
        // println!("New iteration with {} filled seats out of {}!", filled_seats.len(), seats.len());
        let mut new_filled_seats: FxHashSet<(isize, isize)> = FxHashSet::default();
        let mut new_emptied_seats: FxHashSet<(isize, isize)> = FxHashSet::default();

        for seat in seats.iter() {
            let (row, column) = *seat;

            // (-1..=1).cartesian_product(-1..=1)
            let adjacent_occupied = [
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ]
            .into_iter()
            .map(|(delta_row, delta_column)| (row + delta_row, column + delta_column))
            .filter(|(new_row, new_column)| {
                (0..height).contains(new_row) && (0..width).contains(new_column)
            })
            .filter(|spot| filled_seats.contains(spot))
            .count();

            if adjacent_occupied == 0 && !filled_seats.contains(seat) {
                new_filled_seats.insert(*seat);
            } else if adjacent_occupied >= 4 && filled_seats.contains(seat) {
                new_emptied_seats.insert(*seat);
            }
        }

        if new_filled_seats.is_empty() && new_emptied_seats.is_empty() {
            break;
        }

        // println!("Original filled: {filled_seats:?}");
        // println!("New emptied: {new_emptied_seats:?}");
        // println!("New filled: {new_filled_seats:?}");

        filled_seats = filled_seats
            .difference(&new_emptied_seats)
            .cloned()
            .collect();
        filled_seats = filled_seats.union(&new_filled_seats).cloned().collect();
    }

    filled_seats.len()
}

pub fn part_2(file_path: String) -> usize {
    let map: Vec<String> = read_lines(file_path);
    let (seats, height, width) = get_all_seats(&map);
    let visible_seats: FxHashMap<(isize, isize), Vec<(isize, isize)>> = seats
        .iter()
        .map(|seat| {
            let (row, column) = *seat;

            (
                *seat,
                [
                    (0, 1),
                    (0, -1),
                    (1, 0),
                    (-1, 0),
                    (1, 1),
                    (-1, -1),
                    (1, -1),
                    (-1, 1),
                ]
                .into_iter()
                .filter_map(|(delta_row, delta_column)| {
                    let mut new_row = row + delta_row;
                    let mut new_column = column + delta_column;

                    while (0..height).contains(&new_row) && (0..width).contains(&new_column) {
                        // Stop at first.
                        if seats.contains(&(new_row, new_column)) {
                            return Some((new_row, new_column));
                        }

                        new_row += delta_row;
                        new_column += delta_column;
                    }

                    None
                })
                .collect::<Vec<(isize, isize)>>(),
            )
        })
        .collect();

    let mut filled_seats: FxHashSet<(isize, isize)> = FxHashSet::default();

    loop {
        // println!("New iteration with {} filled seats out of {}!", filled_seats.len(), seats.len());
        let mut new_filled_seats: FxHashSet<(isize, isize)> = FxHashSet::default();
        let mut new_emptied_seats: FxHashSet<(isize, isize)> = FxHashSet::default();

        for seat in seats.iter() {
            // (-1..=1).cartesian_product(-1..=1)
            let adjacent_occupied = visible_seats
                .get(seat)
                .unwrap()
                .iter()
                .filter(|spot| filled_seats.contains(spot))
                .count();

            if adjacent_occupied == 0 && !filled_seats.contains(seat) {
                new_filled_seats.insert(*seat);
            } else if adjacent_occupied >= 5 && filled_seats.contains(seat) {
                new_emptied_seats.insert(*seat);
            }
        }

        if new_filled_seats.is_empty() && new_emptied_seats.is_empty() {
            break;
        }

        // println!("Original filled: {filled_seats:?}");
        // println!("New emptied: {new_emptied_seats:?}");
        // println!("New filled: {new_filled_seats:?}");

        filled_seats = filled_seats
            .difference(&new_emptied_seats)
            .cloned()
            .collect();
        filled_seats = filled_seats.union(&new_filled_seats).cloned().collect();
    }

    filled_seats.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 37)]
    #[case(false, 2481)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_1(get_file_path(is_test, 11)));
    }

    #[rstest]
    #[case(true, 26)]
    #[case(false, 2227)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 11)));
    }
}
