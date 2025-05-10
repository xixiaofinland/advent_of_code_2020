use crate::AoCResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day8a() -> AoCResult<usize> {
    let file = File::open("data/input_day8a_simple.txt")?;
    let reader = BufReader::new(file);
    let content = reader
        .lines()
        .filter_map(|line_result| {
            line_result.ok().and_then(|line| {
                line.split_once(" ")
                    .map(|(first, second)| (first.to_string(), second.to_string()))
            })
        })
        .collect::<Vec<(String, String)>>();

    // reader
    //     .lines()
    //     .map(|l| l.map(|line| line.split_once(" ").unwrap()));

    Ok(0)
}
