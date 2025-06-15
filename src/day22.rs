use crate::AoCResult;
use std::fs;

pub fn solve_day22a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day22a_simple.txt")?;
    let (player1, player2) = file
        .split_once("\n\n")
        .ok_or_else(|| "cannot split_once correctly.")?;
    let palyer1_stack: Vec<usize> = player1
        .lines()
        .skip(1)
        .map(|n| n.parse::<usize>())
        .collect::<Result<_,_>>()?;

    let palyer2_stack: Vec<usize> = player2
        .lines()
        .skip(1)
        .map(|n| n.parse::<usize>())
        .collect::<Result<_,_>>()?;

    Ok(0)
}
