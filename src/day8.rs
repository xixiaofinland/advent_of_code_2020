use crate::AoCResult;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}

pub fn solve_day8a() -> AoCResult<i32> {
    let file = File::open("data/input_day8a.txt")?;
    let reader = BufReader::new(file);
    let program = reader
        .lines()
        .map(|line_result| {
            let line = line_result?;
            parse_instruction(&line).ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid instruction: {}", line),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(run_until_loop(&program))
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let (op, num) = line.split_once(' ')?;
    let value = num.parse::<i32>().ok()?;
    match op {
        "nop" => Some(Instruction::Nop),
        "acc" => Some(Instruction::Acc(value)),
        "jmp" => Some(Instruction::Jmp(value)),
        _ => None,
    }
}

fn run_until_loop(program: &[Instruction]) -> i32 {
    let mut acc = 0;
    let mut visited = HashSet::new();
    let mut index: i32 = 0;

    while index >= 0 && (index as usize) < program.len() {
        if !visited.insert(index) {
            break;
        }

        match program[index as usize] {
            Instruction::Nop => index += 1,
            Instruction::Acc(v) => {
                acc += v;
                index += 1;
            }
            Instruction::Jmp(v) => index += v,
        }
    }

    acc
}

pub fn solve_day8a_with_error_handling() -> AoCResult<usize> {
    let file = File::open("data/input_day8a_simple.txt")?;
    let reader = BufReader::new(file);

    let parsed_lines: Vec<(String, String)> = reader
        .lines()
        .map(|line_result| -> AoCResult<_> {
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
