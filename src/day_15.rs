use std::collections::HashMap;

pub fn do_math(numbers: &Vec<usize>, turns: usize) -> i64 {
    let mut memory = HashMap::new();
    let mut last_number = numbers[0];

    for turn in 0..turns {
        let new_number = if turn < numbers.len() {
            numbers[turn]
        } else if let Some(last_number_turn) = memory.get(&last_number) {
            turn - *last_number_turn - 1
        } else {
            0
        };

        // println!("{new_number} in turn {}", turn + 1);

        if turn > 0 {
            memory.insert(last_number, turn - 1);
        }

        last_number = new_number;
    }

    last_number as i64
}

pub fn part_1(numbers: &Vec<usize>) -> i64 {
    do_math(numbers, 2020)
}

pub fn part_2(numbers: &Vec<usize>) -> i64 {
    do_math(numbers, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![0, 3, 6], 436)]
    #[case(vec![1, 3, 2], 1)]
    #[case(vec![2, 1, 3], 10)]
    #[case(vec![1, 2, 3], 27)]
    #[case(vec![2, 3, 1], 78)]
    #[case(vec![3, 2, 1], 438)]
    #[case(vec![3, 1, 2], 1836)]
    #[case(vec![6, 4, 12, 1, 20, 0, 16], 475)]
    fn test_part_1(#[case] input: Vec<usize>, #[case] expected: i64) {
        assert_eq!(expected, part_1(&input));
    }

    #[rstest]
    #[case(vec![0, 3, 6], 175594)]
    #[case(vec![6, 4, 12, 1, 20, 0, 16], 11261)]
    fn test_part_2(#[case] input: Vec<usize>, #[case] expected: i64) {
        assert_eq!(expected, part_2(&input));
    }
}
