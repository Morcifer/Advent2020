use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::VecDeque;

use crate::utilities::file_utilities::read_lines;
use rustc_hash::FxHashSet;

fn parse_cluster(line: String) -> Vec<usize> {
    let split_by_colon: Vec<&str> = line.split(':').map(str::trim).collect();
    split_by_colon[1]
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn parse_data(file_path: String) -> Vec<Vec<usize>> {
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

pub fn part_1(file_path: String) -> i64 {
    let decks = parse_data(file_path);
    let mut player_1_deck: VecDeque<usize> = decks[0].iter().cloned().collect();
    let mut player_2_deck: VecDeque<usize> = decks[1].iter().cloned().collect();

    let mut round = 0;

    while !player_1_deck.is_empty() && !player_2_deck.is_empty() {
        round += 1;
        // println!("Round {round}: Player 1 deck is {player_1_deck:?}");
        // println!("Round {round}: Player 2 deck is {player_2_deck:?}");

        let Some(player_1_card) = player_1_deck.pop_front() else {
            panic!("Player 1 pop")
        };
        let Some(player_2_card) = player_2_deck.pop_front() else {
            panic!("Player 2 pop")
        };

        match player_1_card.cmp(&player_2_card) {
            Ordering::Greater => {
                player_1_deck.push_back(player_1_card);
                player_1_deck.push_back(player_2_card);
            }
            Ordering::Less => {
                player_2_deck.push_back(player_2_card);
                player_2_deck.push_back(player_1_card);
            }
            Ordering::Equal => panic!("Round {round}: This should not happen!"),
        };
    }

    let winning_deck = if player_1_deck.is_empty() {
        player_2_deck
    } else {
        player_1_deck
    };

    // println!("Winning deck is {winning_deck:?}");

    winning_deck
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (card * (i + 1)) as i64)
        .sum()
}

fn recursive_game(
    player_1_deck: &mut VecDeque<usize>,
    player_2_deck: &mut VecDeque<usize>,
    _game_level: usize,
    // history?
) -> bool {
    // Player 1 wins
    let mut history: FxHashSet<(String, String)> = FxHashSet::default();
    let mut _round = 0;

    // println!("=== Game {game_level} ===");

    while !player_1_deck.is_empty() && !player_2_deck.is_empty() {
        _round += 1;

        // println!("-- Round {round} (Game {game_level}) --");
        // println!("Player 1's deck: {player_1_deck:?}");
        // println!("Player 2's deck: {player_2_deck:?}");

        // This prevents infinite games of Recursive Combat, which everyone agrees is a bad idea.
        let key = (
            player_1_deck.iter().join("-"),
            player_2_deck.iter().join("-"),
        );
        if history.contains(&key) {
            // println!("History repeats itself, player 1 wins game {game_level}");
            return true;
        }

        history.insert(key);

        let Some(player_1_card) = player_1_deck.pop_front() else {
            panic!("Player 1 pop")
        };
        let Some(player_2_card) = player_2_deck.pop_front() else {
            panic!("Player 2 pop")
        };

        // println!("Player 1 plays: {player_1_card}");
        // println!("Player 2 plays: {player_2_card}");

        let player_1_win =
            if player_1_deck.len() >= player_1_card && player_2_deck.len() >= player_2_card {
                // println!("Playing a sub-game to determine the winner...");
                let mut player_1_clone: VecDeque<usize> =
                    player_1_deck.iter().cloned().take(player_1_card).collect();
                let mut player_2_clone: VecDeque<usize> =
                    player_2_deck.iter().cloned().take(player_2_card).collect();
                recursive_game(&mut player_1_clone, &mut player_2_clone, _game_level + 1)
            } else {
                player_1_card > player_2_card
            };

        if player_1_win {
            // println!("Player 1 wins round {round} of game {game_level}!");
            player_1_deck.push_back(player_1_card);
            player_1_deck.push_back(player_2_card);
        } else {
            // println!("Player 2 wins round {round} of game {game_level}!");
            player_2_deck.push_back(player_2_card);
            player_2_deck.push_back(player_1_card);
        };
    }

    !player_1_deck.is_empty()
}

pub fn part_2(file_path: String) -> i64 {
    let decks = parse_data(file_path);
    let mut player_1_deck: VecDeque<usize> = decks[0].iter().cloned().collect();
    let mut player_2_deck: VecDeque<usize> = decks[1].iter().cloned().collect();

    let winner = recursive_game(&mut player_1_deck, &mut player_2_deck, 1);

    let winning_deck = if winner { player_1_deck } else { player_2_deck };

    // println!("Winning deck is {winning_deck:?}");

    winning_deck
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (card * (i + 1)) as i64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 306)]
    #[case(false, 32856)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 22, None)));
    }

    #[rstest]
    #[case(true, 291)]
    #[case(false, 33805)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 22, None)));
    }
}
