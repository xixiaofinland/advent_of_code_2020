use crate::AoCResult;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day9a() -> AoCResult<usize> {
    // let file = File::open("data/input_day9a_simple.txt")?;
    let file = File::open("data/input_day9a.txt")?;
    let reader = BufReader::new(file);

    let content: Vec<_> = reader
        .lines()
        .filter_map(|line_result| line_result.ok()?.parse::<usize>().ok())
        .collect();

    let preceding_size = 25;
    Ok(validate(preceding_size, &content))
}

fn validate(preceding_size: usize, content: &[usize]) -> usize {
    content
        .iter()
        .enumerate()
        .skip(preceding_size + 1)
        .find(|(i, item)| {
            let collection = build_collection(*i, preceding_size, content);
            !collection.contains(item)
        })
        .map(|(_, item)| item)
        .copied()
        .unwrap_or(0)
}

fn build_collection(index: usize, preceding_size: usize, content: &[usize]) -> HashSet<usize> {
    let mut collection = HashSet::new();

    for i in (index - preceding_size)..index {
        for j in (i + 1)..index {
            collection.insert(content[i] + content[j]);
        }
    }
    collection
}
