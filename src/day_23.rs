use itertools::Itertools;

pub fn part_1(cups_data: String) -> String {
    let mut cups: Vec<usize> = cups_data
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|i| i as usize)
        .collect();

    let mut current_cup_index = 0;

    for _move in 1..=100 {
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

    let one_index = cups.iter().position(|c| *c == 1).unwrap();
    cups.iter()
        .skip(one_index)
        .chain(cups.iter().take(one_index))
        .skip(1)
        .join("")
}

pub fn part_2(cups_data: String) -> i64 {
    let mut cups: Vec<usize> = cups_data
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|i| i as usize)
        .collect();

    cups.extend(10..1000000);

    let mut current_cup_index = 0;

    for _move in 1..=10000000 {
        // println!("-- move {_move} --");

        let current_cup = cups[current_cup_index];

        let picked_up: Vec<usize> = (0..3)
            .map(|_| {
                let mut index_to_remove = current_cup_index + 1;
                if index_to_remove >= cups.len() {
                    index_to_remove = 0;
                }
                cups.remove(index_to_remove)
            })
            .collect();

        let mut destination_cup = if current_cup != 1 { current_cup - 1 } else { 9 };
        while picked_up.contains(&destination_cup) {
            destination_cup = if destination_cup != 1 {
                destination_cup - 1
            } else {
                9
            };
        }
        let destination_cup_index = cups.iter().position(|c| *c == destination_cup).unwrap();

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

    let one_index = cups.iter().position(|c| *c == 1).unwrap();
    (cups[one_index - 1] as i64) * (cups[one_index - 2] as i64)
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
