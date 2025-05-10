use crate::AoCResult;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day8a() -> AoCResult<i32> {
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

    let mut value: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    let result = calculate(0, &content, &mut value, &mut visited);
    Ok(result)
}

fn calculate(
    index: i32,
    content: &Vec<(String, i32)>,
    value: &mut i32,
    visited: &mut HashSet<i32>,
) -> i32 {
    eprintln!();
    if !visited.insert(index) {
        println!("Done index: {}", index);
        println!("Done value: {}", value);
        return *value;
    }

    let (op, num) = content.get(usize::try_from(index).unwrap()).unwrap();
    match op.as_str() {
        "nop" => calculate(index + 1, content, value, visited),
        "acc" => {
            *value += num;
            calculate(index + 1, content, value, visited)
        }
        "jmp" => calculate(index + num, content, value, visited),
        _ => unreachable!(),
    }
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
