use crate::utilities::file_utilities::read_lines;

use itertools::iproduct;
use std::collections::HashSet;

fn get_cube_neighbours(cube: &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    let (x, y, z) = cube;
    iproduct!(-1..=1, -1..=1, -1..=1)
        .filter(|(dx, dy, dz)| *dx != 0 || *dy != 0 || *dz != 0)
        .map(|(dx, dy, dz)| (x + dx, y + dy, z + dz))
        .collect::<Vec<_>>()
}

fn get_hypercube_neighbours(
    cube: &(isize, isize, isize, isize),
) -> Vec<(isize, isize, isize, isize)> {
    let (x, y, z, t) = cube;
    iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
        .filter(|(dx, dy, dz, dt)| *dx != 0 || *dy != 0 || *dz != 0 || *dt != 0)
        .map(|(dx, dy, dz, dt)| (x + dx, y + dy, z + dz, t + dt))
        .collect::<Vec<_>>()
}

pub fn part_1(file_path: String) -> i64 {
    let input: Vec<String> = read_lines(file_path);

    let mut active_cubes: HashSet<(isize, isize, isize)> = (0..input.len())
        .flat_map(|y| {
            input[y]
                .chars()
                .enumerate()
                .filter_map(move |(x, character)| match character {
                    '#' => Some((x as isize, y as isize, 0)),
                    _ => None,
                })
        })
        .collect();

    for _cycle in 1..=6 {
        // println!("Cycle {cycle} starts with {} active cubes", active_cubes.len());

        let cubes_to_investigate: HashSet<(isize, isize, isize)> = active_cubes
            .iter()
            .flat_map(get_cube_neighbours)
            .chain(active_cubes.iter().cloned())
            .collect();

        // println!("Cycle {cycle} active_cubes: {active_cubes:?}");
        // println!("Cycle {cycle} cubes_to_investigate: {cubes_to_investigate:?}");

        let still_active_cubes: Vec<(isize, isize, isize)> = cubes_to_investigate
            .iter()
            .filter(|cube| active_cubes.contains(cube))
            .filter(|cube| {
                let active_neighbours_count = get_cube_neighbours(cube)
                    .iter()
                    .filter(|neighbour| active_cubes.contains(neighbour))
                    .count();
                (2..=3).contains(&active_neighbours_count)
            })
            .cloned()
            .collect();

        let new_active_cubes: Vec<(isize, isize, isize)> = cubes_to_investigate
            .iter()
            .filter(|cube| !active_cubes.contains(cube))
            .filter(|cube| {
                let active_neighbours_count = get_cube_neighbours(cube)
                    .iter()
                    .filter(|neighbour| active_cubes.contains(neighbour))
                    .count();
                active_neighbours_count == 3
            })
            .cloned()
            .collect();

        // println!("Cycle {cycle} still_active_cubes: {still_active_cubes:?}");
        // println!("Cycle {cycle} new_active_cubes: {new_active_cubes:?}");

        active_cubes = still_active_cubes
            .into_iter()
            .chain(new_active_cubes.into_iter())
            .collect();
    }

    // println!("{active_cubes:?}");
    // println!("{} active cubes", active_cubes.len());

    active_cubes.len() as i64
}

pub fn part_2(file_path: String) -> i64 {
    let input: Vec<String> = read_lines(file_path);

    let mut active_cubes: HashSet<(isize, isize, isize, isize)> = (0..input.len())
        .flat_map(|y| {
            input[y]
                .chars()
                .enumerate()
                .filter_map(move |(x, character)| match character {
                    '#' => Some((x as isize, y as isize, 0, 0)),
                    _ => None,
                })
        })
        .collect();

    for _ in 1..=6 {
        let cubes_to_investigate: HashSet<(isize, isize, isize, isize)> = active_cubes
            .iter()
            .flat_map(get_hypercube_neighbours)
            .chain(active_cubes.iter().cloned())
            .collect();

        let still_active_cubes: Vec<(isize, isize, isize, isize)> = cubes_to_investigate
            .iter()
            .filter(|cube| active_cubes.contains(cube))
            .filter(|cube| {
                let active_neighbours_count = get_hypercube_neighbours(cube)
                    .iter()
                    .filter(|neighbour| active_cubes.contains(neighbour))
                    .count();
                (2..=3).contains(&active_neighbours_count)
            })
            .cloned()
            .collect();

        let new_active_cubes: Vec<(isize, isize, isize, isize)> = cubes_to_investigate
            .iter()
            .filter(|cube| !active_cubes.contains(cube))
            .filter(|cube| {
                let active_neighbours_count = get_hypercube_neighbours(cube)
                    .iter()
                    .filter(|neighbour| active_cubes.contains(neighbour))
                    .count();
                active_neighbours_count == 3
            })
            .cloned()
            .collect();

        active_cubes = still_active_cubes
            .into_iter()
            .chain(new_active_cubes.into_iter())
            .collect();
    }

    active_cubes.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 112)]
    #[case(false, 267)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 17, None)));
    }

    #[rstest]
    #[case(true, 848)]
    #[case(false, 1812)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 17, None)));
    }
}
