use crate::AoCResult;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day6a() -> AoCResult<usize> {
    let file = File::open("data/input_day6a.txt")?;

    let mut answers = HashSet::new();
    let mut count = 0;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            count += answers.len();
            answers.clear();
        } else {
            answers.extend(line.chars());
        }
    }

    count += answers.len();

    Ok(count)
}

// pub fn solve_day6b() -> AoCResult<Option<usize>> {}
