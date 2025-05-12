use crate::AoCResult;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day9b() -> AoCResult<usize> {
    const NUM: usize = 675280050;

    let file = File::open("data/input_day9a.txt")?;
    let reader = BufReader::new(file);

    let content: Vec<_> = reader
        .lines()
        .filter_map(|line_result| line_result.ok()?.parse::<usize>().ok())
        .collect();

    let preceding_size = 25;
    Ok(validate(preceding_size, &content))
}
