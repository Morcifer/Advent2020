use crate::utilities::file_utilities::read_lines;

fn tree_is_in_position(inputs: &Vec<String>, spot_row: usize, spot_column: usize) -> bool {
    let spot_row = spot_row;
    let spot_column = spot_column % inputs[0].len();

    return inputs[spot_row][spot_column..=spot_column].eq("#");
}

fn solve(trees: &Vec<String>, dx: usize, dy: usize) -> i64 {
    let height = trees.len();

    let mut trees_in_slope = 0;

    let mut row = 0;
    let mut column = 0;

    while row + dy < height {
        column += dx;
        row += dy;

        if tree_is_in_position(&trees, row, column) {
            trees_in_slope += 1;
        }
    }

    trees_in_slope
}

pub fn part_1(file_path: String) -> i64 {
    let trees: Vec<String> = read_lines(file_path);

    solve(&trees, 3, 1)
}

pub fn part_2(file_path: String) -> i64 {
    let trees: Vec<String> = read_lines(file_path);

    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(dx, dy)| solve(&trees, dx, dy))
        .product::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 7)]
    #[case(false, 214)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 3, None)));
    }

    #[rstest]
    #[case(true, 336)]
    #[case(false, 8336352024)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 3, None)));
    }
}
