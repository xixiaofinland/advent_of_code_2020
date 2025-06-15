use itertools::Itertools;
use std::collections::VecDeque;

use crate::AoCResult;
use std::fs;

pub fn solve_day22a() -> AoCResult<usize> {
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

    play(&mut player1, &mut player2);

    let winner = if !player1.is_empty() {
        player1
    } else {
        player2
    };

    let sum: usize = winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &val)| (i + 1) * val)
        .sum();

    Ok(sum)
}

fn play(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>) {
    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else if card1 < card2 {
            player2.push_back(card2);
            player2.push_back(card1);
        } else {
            unreachable!()
        }
    }
}
