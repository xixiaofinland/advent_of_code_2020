use itertools::Itertools;

use crate::AoCResult;
use std::fs;

pub fn solve_day22a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day22a.txt")?;
    let (player1_data, player2_data) = file
        .split_once("\n\n")
        .ok_or_else(|| "cannot split_once correctly.")?;

    let mut player1: Vec<usize> = player1_data
        .lines()
        .skip(1)
        .map(|n| n.parse::<usize>())
        .collect::<Result<_, _>>()?;

    let mut player2: Vec<usize> = player2_data
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

fn play(player1: &mut Vec<usize>, player2: &mut Vec<usize>) {
    while !player1.is_empty() && !player2.is_empty() {
        if player1[0] > player2[0] {
            let first = player1.remove(0);
            player1.push(first);
            player1.push(player2.remove(0));
        } else if player1[0] < player2[0] {
            let first = player2.remove(0);
            player2.push(first);
            player2.push(player1.remove(0));
        } else {
            unreachable!()
        }
    }
}
