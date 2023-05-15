use crate::utilities::file_utilities::read_lines;
use std::collections::HashSet;

const NUM_ROWS: i64 = 128;
const NUM_COLUMNS: i64 = 8;

fn get_seat_number(boarding_pass: &String) -> i64 {
    let mut minimal_row = 0;
    let mut maximal_row = NUM_ROWS;

    let mut minimal_column = 0;
    let mut maximal_column = NUM_COLUMNS;

    for p in boarding_pass.chars() {
        let rows_left = (maximal_row - minimal_row) / 2;
        let columns_left = (maximal_column - minimal_column) / 2;

        match p {
            'F' => maximal_row = minimal_row + rows_left,
            'B' => minimal_row = maximal_row - rows_left,
            'L' => maximal_column = minimal_column + columns_left,
            'R' => minimal_column = maximal_column - columns_left,
            _ => todo!(),
        }
    }

    minimal_row * 8 + minimal_column
}

pub fn part_1(file_path: String) -> i64 {
    let boarding_passes: Vec<String> = read_lines(file_path);

    boarding_passes.iter().map(get_seat_number).max().unwrap()
}

pub fn part_2(file_path: String) -> i64 {
    let boarding_passes: HashSet<i64> = read_lines(file_path).iter().map(get_seat_number).collect();

    for seat_id in 1..(NUM_ROWS * 8 + NUM_COLUMNS - 1) {
        let below = seat_id - 1;
        let above = seat_id + 1;

        if boarding_passes.contains(&below)
            && !boarding_passes.contains(&seat_id)
            && boarding_passes.contains(&above)
        {
            return seat_id;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 820)]
    #[case(false, 888)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 5)));
    }

    #[rstest]
    #[case(true, -1)]
    #[case(false, 522)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 5)));
    }
}
