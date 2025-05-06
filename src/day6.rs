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

pub fn solve_day6b() -> AoCResult<usize> {
    let file = File::open("data/input_day6a.txt")?;

    let mut group_answers: Vec<String> = Vec::new();
    let mut total = 0;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            total += intersection_size(&group_answers);
            group_answers.clear();
        } else {
            group_answers.push(line);
        }
    }

    if !group_answers.is_empty() {
        total += intersection_size(&group_answers);
    }

    Ok(total)
}

fn intersection_size(group: &[String]) -> usize {
    let mut iter = group.iter().map(|s| s.chars().collect::<HashSet<_>>());
    if let Some(first) = iter.next() {
        iter.fold(first, |acc, set| &acc & &set).len()
    } else {
        0
    }
}

pub fn solve_day6b_ff() -> AoCResult<usize> {
    let input = std::fs::read_to_string("data/input_day6a.txt")?;

    let total = input
        .split("\n\n") // group by blank lines
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|a, b| &a & &b) // set intersection
                .map_or(0, |set| set.len()) // default to 0 if group is empty
        })
        .sum();

    Ok(total)
}
