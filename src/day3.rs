use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::AoCResult;

pub fn solve_day3a() -> AoCResult<usize> {
    let reader = BufReader::new(File::open("data/input_day3a.txt")?);

    let maze: Vec<Vec<bool>> = reader
        .lines()
        .map(|line| line.map(|l| l.chars().map(|c| c == '#').collect()))
        .collect::<Result<_, _>>()?;

    let width = maze[0].len();
    let count = maze
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(i, row)| {
            let col = (i * 3) % width;
            row[col]
        })
        .count();

    Ok(count)
}
