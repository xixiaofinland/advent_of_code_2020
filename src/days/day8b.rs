use crate::AoCResult;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

pub fn solve_day8b() -> AoCResult<i32> {
    let file = File::open("data/input_day8a.txt")?;
    let reader = BufReader::new(file);
    let mut program = reader
        .lines()
        .map(|line_result| -> AoCResult<_> {
            let line = line_result?;
            parse_instruction(&line).ok_or(format!("Invalid instruction: {}", line).into())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let path = find_termination_path(&program);

    for &i in &path {
        let original = program[i];

        program[i] = match original {
            Instruction::Jmp(v) => Instruction::Nop(v),
            Instruction::Nop(v) => Instruction::Jmp(v),
            Instruction::Acc(_) => continue,
        };

        let (acc, success) = run_until_loop(&program);
        if success {
            return Ok(acc);
        }

        program[i] = original; // revert change
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "No fix found that terminates the program",
    )
    .into())
}

fn find_termination_path(program: &[Instruction]) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut path = Vec::new();
    let mut index: i32 = 0;

    while index >= 0 && (index as usize) < program.len() {
        if !visited.insert(index) {
            break;
        }
        path.push(index as usize);

        match program[index as usize] {
            Instruction::Nop(_) | Instruction::Acc(_) => index += 1,
            Instruction::Jmp(v) => index += v,
        }
    }

    path
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let (op, num) = line.split_once(' ')?;
    let value = num.parse::<i32>().ok()?;
    match op {
        "nop" => Some(Instruction::Nop(value)),
        "acc" => Some(Instruction::Acc(value)),
        "jmp" => Some(Instruction::Jmp(value)),
        _ => None,
    }
}

fn run_until_loop(program: &[Instruction]) -> (i32, bool) {
    let mut acc = 0;
    let mut visited = HashSet::new();
    let mut index: i32 = 0;

    while index >= 0 && (index as usize) < program.len() {
        if !visited.insert(index) {
            return (acc, false); // loop detected
        }

        match program[index as usize] {
            Instruction::Nop(_) => index += 1,
            Instruction::Acc(v) => {
                acc += v;
                index += 1;
            }
            Instruction::Jmp(v) => index += v,
        }
    }

    (acc, true) // terminated normally
}

