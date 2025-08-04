use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fs;
use crate::AoCResult;

pub fn solve_day22b() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day22a.txt")?;
    let (player1_data, player2_data) = file
        .split_once("\n\n")
        .ok_or_else(|| "cannot split_once correctly.")?;

    let mut player1: VecDeque<usize> = player1_data
        .lines()
        .skip(1)
        .map(|n| n.parse::<usize>())
        .collect::<Result<_, _>>()?;

    let mut player2: VecDeque<usize> = player2_data
        .lines()
        .skip(1)
        .map(|n| n.parse::<usize>())
        .collect::<Result<_, _>>()?;

    let (winner, winning_deck) = play(&mut player1, &mut player2);

    let sum: usize = winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &val)| (i + 1) * val)
        .sum();

    Ok(sum)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    One,
    Two,
}

fn play(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>) -> (Player, VecDeque<usize>) {
    let mut seen: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        // Check for previous state
        let state = (player1.iter().copied().collect::<Vec<_>>(),
                     player2.iter().copied().collect::<Vec<_>>());

        if seen.contains(&state) {
            return (Player::One, player1.clone());
        }
        seen.insert(state);

        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        let round_winner = if player1.len() >= card1 && player2.len() >= card2 {
            // Recurse with copies of the top N cards
            let mut sub_player1: VecDeque<usize> = player1.iter().take(card1).copied().collect();
            let mut sub_player2: VecDeque<usize> = player2.iter().take(card2).copied().collect();
            play(&mut sub_player1, &mut sub_player2).0
        } else {
            if card1 > card2 { Player::One } else { Player::Two }
        };

        match round_winner {
            Player::One => {
                player1.push_back(card1);
                player1.push_back(card2);
            }
            Player::Two => {
                player2.push_back(card2);
                player2.push_back(card1);
            }
        }
    }

    if player1.is_empty() {
        (Player::Two, player2.clone())
    } else {
        (Player::One, player1.clone())
    }
}
