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
                line.split_once(" ").and_then(|(first, second)| {
                    second.parse::<i32>().ok().map(|v| (first.to_string(), v))
                })
            })
        })
        .collect::<Vec<(String, i32)>>();

    Ok(0)
}

pub fn solve_day8a_with_error_handling() -> AoCResult<usize> {
    let file = File::open("data/input_day8a_simple.txt")?;
    let reader = BufReader::new(file);

    let parsed_lines: Vec<(String, String)> = reader
        .lines()
        .map(|line_result| -> AoCResult<(String, String)> {
            let line = line_result?; // Propagate IO errors

            // The simplest error option with Rust 2021+
            // as String has implemented std::error::Error
            let (first, second) = line
                .split_once(" ")
                .ok_or(format!("Line missing space: '{}'", line))?;

            Ok((first.to_string(), second.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?; // Collect and propagate any errors

    // Continue processing with parsed_lines
    println!("Successfully parsed {} lines", parsed_lines.len());

    Ok(0) // Replace with actual result calculation
}
