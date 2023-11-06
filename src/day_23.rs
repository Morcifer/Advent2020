use itertools::Itertools;


fn simulate_moves(cups: &Vec<usize>, moves_count: usize) -> Vec<usize> {
    let mut cups = cups.clone();

    let mut current_cup_index = 0;

    for _move in 1..=moves_count {
        // println!("-- move {_move} --");
        // println!("cups: {}", cups.iter().join(""));

        let current_cup = cups[current_cup_index];
        // println!("current cup: {current_cup} at index {current_cup_index}");

        let picked_up: Vec<usize> = (0..3)
            .map(|_| {
                let mut index_to_remove = current_cup_index + 1;
                if index_to_remove >= cups.len() {
                    index_to_remove = 0;
                }
                cups.remove(index_to_remove)
            })
            .collect();

        // println!("picked up: {picked_up:?}");

        let mut destination_cup = if current_cup != 1 { current_cup - 1 } else { 9 };
        while picked_up.contains(&destination_cup) {
            destination_cup = if destination_cup != 1 {
                destination_cup - 1
            } else {
                9
            };
        }
        let destination_cup_index = cups.iter().position(|c| *c == destination_cup).unwrap();

        // println!("destination cup: {destination_cup} at index {destination_cup_index}");

        for cup in picked_up.into_iter().rev() {
            cups.insert(destination_cup_index + 1, cup)
        }

        current_cup_index = cups.iter().position(|c| *c == current_cup).unwrap();
        current_cup_index = if current_cup_index != 8 {
            current_cup_index + 1
        } else {
            0
        };
    }

    cups
}

pub fn part_1(cups_data: String) -> String {
    let cups: Vec<usize> = cups_data
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|i| i as usize)
        .collect();

    let moved_cups = simulate_moves(&cups, 100);

    let one_index = moved_cups.iter().position(|c| *c == 1).unwrap();
    moved_cups.iter()
        .skip(one_index)
        .chain(moved_cups.iter().take(one_index))
        .skip(1)
        .join("")
}

pub fn part_2(cups_data: String) -> i64 {
    let mut cups: Vec<usize> = cups_data
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|i| i as usize)
        .collect();

    cups.extend(10..1_000_000);

    let moved_cups = simulate_moves(&cups, 10_000_000);

    let one_index = moved_cups.iter().position(|c| *c == 1).unwrap();
    (moved_cups[one_index - 1] as i64) * (moved_cups[one_index - 2] as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case("389125467", "67384529")]
    #[case("157623984", "58427369")]
    fn test_part_1(#[case] input: String, #[case] expected: String) {
        assert_eq!(expected, part_1(input));
    }

    #[rstest]
    #[case("389125467", 149245887792)]
    #[case("157623984", 149245887792)]
    fn test_part_2(#[case] input: String, #[case] expected: i64) {
        assert_eq!(expected, part_2(input));
    }
}
