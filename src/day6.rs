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

    let mut answers: Vec<String> = Vec::new();
    let mut count = 0;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            count += calculate(&answers);
            answers.clear();
        } else {
            answers.push(line);
        }
    }

    if !answers.is_empty() {
        count += calculate(&answers);
    }

    Ok(count)
}

fn calculate(answers: &[String]) -> usize {
    let mut answers_iter = answers.iter();

    let mut exists_in_all = HashSet::new();

    if let Some(first_answer) = answers_iter.next() {
        exists_in_all.extend(first_answer.chars());
    }

    for answer in answers_iter {
        for el in exists_in_all.clone() {
            if !answer.contains(el) {
                exists_in_all.remove(&el);
            }
        }
    }
    let result = exists_in_all.len();

    result
}
