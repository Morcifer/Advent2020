const MODULO: u64 = 20201227;

fn transform_once(value: u64, subject_number: u64) -> u64 {
    (value * subject_number) % MODULO
}

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;

    for _ in 0..loop_size {
        value = transform_once(value, subject_number);
    }

    value
}

fn hack_loop_size(public_key: u64) -> u64 {
    let mut transformation = 1;
    let mut loop_size = 1;

    loop {
        transformation = transform_once(transformation, 7);

        if transformation == public_key {
            // println!("Found loop size {loop_size} for key {public_key}");
            return loop_size;
        }

        loop_size += 1;
    }
}

pub fn part_1(input: (u64, u64)) -> u64 {
    let card_public_key = input.0;
    let door_public_key = input.1;

    let card_loop_size = hack_loop_size(card_public_key);
    let door_loop_size = hack_loop_size(door_public_key);

    let card_encryption_key = transform(door_public_key, card_loop_size);
    let door_encryption_key = transform(card_public_key, door_loop_size);

    if card_encryption_key != door_encryption_key {
        panic!("I think something is off with your keys");
    }

    card_encryption_key
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((5764801, 17807724), 14897079)]
    #[case((9717666, 20089533), 19924389)]
    fn test_part_1(#[case] input: (u64, u64), #[case] expected: u64) {
        assert_eq!(expected, part_1(input));
    }
}
