use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::AoCResult;

pub fn solve_day3a() -> AoCResult<usize> {
    let reader = BufReader::new(File::open("data/input_day3a.txt")?);
    let maze: Vec<Vec<bool>> = reader
        .lines()
        .map(|line| -> Result<Vec<bool>, std::io::Error> {
            Ok(line?.chars().map(|c| c == '#').collect())
        })
        .collect::<Result<_, _>>()?;

    // let maze: Vec<Vec<bool>> = reader
    //     .lines()
    //     .map(|line| line.map(|l| l.chars().map(|c| c == '#').collect()))
    //     .collect::<Result<_, _>>()?;

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

pub fn solve_day3b() -> AoCResult<usize> {
    let mut result = 1;

    for i in [1, 3, 5, 7] {
        // result *= count_type_one(i).unwrap();
        result *= count_trees(i, 1).unwrap();
    }

    // result *= count_type_two().unwrap();
    result *= count_trees(1, 2).unwrap();

    Ok(result)
}

fn count_trees(right_shift: usize, down_shift: usize) -> AoCResult<usize> {
    let maze: Vec<Vec<bool>> = BufReader::new(File::open("data/input_day3a.txt")?)
        .lines()
        .map(|line| line.map(|l| l.chars().map(|c| c == '#').collect()))
        .collect::<Result<_, _>>()?;

    let width = maze[0].len();

    let count = maze
        .iter()
        .step_by(down_shift)
        .skip(1)
        .zip(1..) // turn into logical steps
        .filter(|(row, step)| {
            let col = (step * right_shift) % width;
            row[col]
        })
        .count();

    Ok(count)
}

fn count_type_two() -> AoCResult<usize> {
    let reader = BufReader::new(File::open("data/input_day3a.txt")?);

    let maze: Vec<Vec<bool>> = reader
        .lines()
        .map(|line| line.map(|l| l.chars().map(|c| c == '#').collect()))
        .collect::<Result<_, _>>()?;

    let width = maze[0].len();
    let count = maze
        .iter()
        .step_by(2)
        .skip(1)
        .enumerate()
        .filter(|(i, row)| {
            let col = (i + 1) % width;
            row[col]
        })
        .count();

    Ok(count)
}

fn count_type_one(right_shift: usize) -> AoCResult<usize> {
    let reader = BufReader::new(File::open("data/input_day3a.txt")?);

    let maze: Vec<Vec<bool>> = reader
        .lines()
        .map(|line| line.map(|l| l.chars().map(|c| c == '#').collect()))
        .collect::<Result<_, _>>()?;

    let width = maze[0].len();
    let count = maze
        .iter()
        .skip(1)
        .zip(1..)
        .filter(|(row, i)| {
            let col = (i * right_shift) % width;
            row[col]
        })
        .count();

    Ok(count)
}
