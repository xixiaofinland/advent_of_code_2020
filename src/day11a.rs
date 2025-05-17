use crate::AoCResult;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_day11a() -> AoCResult<usize> {
    let file = File::open("data/input_day11a_simple.txt")?;
    let reader = BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| -> AoCResult<Vec<char>> {
            let l = line?;
            Ok(l.chars().collect())
        })
        .collect::<Result<_, _>>()?;

    Ok(0)
}

fn calculate(index: (usize, usize), grid: &mut Vec<Vec<char>>) {}
