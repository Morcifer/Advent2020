use itertools::Itertools;

fn get_resulting_cups(right_neighbour: &[usize], max_cup: usize) -> Vec<usize> {
    let mut current_cup = right_neighbour[1];

    (0..max_cup)
        .map(|_| {
            let output = current_cup;
            current_cup = right_neighbour[current_cup];
            output
        })
        .collect()
}

fn simulate_moves(cups: &[usize], max_cup: usize, moves_count: usize) -> Vec<usize> {
    let mut right_neighbour: Vec<usize> = (0..=max_cup).map(|_| 0).collect();

    for (prev_cup, current_cup) in cups.iter().tuple_windows() {
        right_neighbour[*prev_cup] = *current_cup;
    }
    right_neighbour[*cups.last().unwrap()] = *cups.first().unwrap(); // Circular

    let mut current_cup = cups[0];

    for _move in 1..=moves_count {
        // println!("-- move {_move} --");
        // println!("right_neighbour: {:?}", right_neighbour);
        // println!("cups: {}", get_resulting_cups(&right_neighbour, max_cup).iter().join(" "));

        // println!("current cup: {current_cup}");
        let mut current_for_pickup = current_cup;
        let picked_up: Vec<usize> = (0..3)
            .map(|_| {
                let clockwise_cup = right_neighbour[current_for_pickup];
                current_for_pickup = clockwise_cup;
                clockwise_cup
            })
            .collect();

        // println!("picked up: {picked_up:?}");

        let mut destination_cup = if current_cup != 1 {
            current_cup - 1
        } else {
            max_cup
        };

        while picked_up.contains(&destination_cup) {
            destination_cup = if destination_cup != 1 {
                destination_cup - 1
            } else {
                max_cup
            };
        }

        // println!("destination cup: {destination_cup}");

        let destination_cup_right_neighbour = right_neighbour[destination_cup];
        right_neighbour[destination_cup] = picked_up[0];
        right_neighbour[current_cup] = right_neighbour[picked_up[2]];
        right_neighbour[picked_up[2]] = destination_cup_right_neighbour;

        current_cup = right_neighbour[current_cup];
    }

    get_resulting_cups(&right_neighbour, max_cup)
}

pub fn part_1(cups_data: String) -> String {
    let cups: Vec<usize> = cups_data
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|i| i as usize)
        .collect();

    let moved_cups = simulate_moves(&cups, 9, 100);
    // println!("The cups are: {moved_cups:?}");

    let one_index = moved_cups.iter().position(|c| *c == 1).unwrap();
    moved_cups
        .iter()
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

    cups.extend(10..=1_000_000);
    println!(
        "I have {} cups and the max cup is {}",
        cups.len(),
        cups.iter().max().unwrap()
    );
    println!(
        "The cups are: {:?}",
        cups.iter().take(20).cloned().collect::<Vec<usize>>()
    );

    let moved_cups = simulate_moves(&cups, 1_000_000, 10_000_000);

    let one_index = moved_cups.iter().position(|c| *c == 1).unwrap();
    println!("Index of 1 is {one_index}");
    println!(
        "Cups in that region are {:?}",
        moved_cups
            .iter()
            .skip(one_index - 20)
            .take(21)
            .cloned()
            .collect::<Vec<usize>>()
    );

    (moved_cups[one_index - 1] as i64) * (moved_cups[one_index - 2] as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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
