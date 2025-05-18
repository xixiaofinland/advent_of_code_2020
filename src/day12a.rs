use crate::AoCResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Instruct {
    action: Action,
}

#[derive(Debug)]
enum Action {
    N(i32),
    S(i32),
    W(i32),
    E(i32),
    F(i32),
    L(i32),
    R(i32),
}

impl From<String> for Instruct {
    fn from(s: String) -> Self {
        let (action, num) = s.split_at(1);
        let num = num.parse::<i32>().expect("Invalid number");

        let action = match action {
            "N" => Action::N(num),
            "S" => Action::S(num),
            "W" => Action::W(num),
            "E" => Action::E(num),

            "F" => Action::F(num),

            "L" => Action::L(num),
            "R" => Action::R(num),

            _ => panic!("Invalid action: {}", action),
        };

        Instruct { action }
    }
}

// clock-wise: 0 = North, 1 = East, 2 = South, 3 = West
pub fn solve_day12a() -> AoCResult<usize> {
    let file = File::open("data/input_day12a.txt")?;
    let reader = BufReader::new(file);

    let instructions = reader
        .lines()
        .map(|line_result| -> AoCResult<Instruct> {
            let line = line_result?;
            Ok(Instruct::from(line))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut state: (i32, (i32, i32)) = (1, (0, 0));

    for instruct in instructions {
        match instruct.action {
            Action::N(n) => state.1.1 -= n,
            Action::E(n) => state.1.0 += n,
            Action::S(n) => state.1.1 += n,
            Action::W(n) => state.1.0 -= n,

            Action::L(n) => {
                state.0 = turn(state.0, 'L', n);
            }
            Action::R(n) => {
                state.0 = turn(state.0, 'R', n);
            }

            Action::F(n) => match state.0 {
                0 => state.1.1 -= n,
                1 => state.1.0 += n,
                2 => state.1.1 += n,
                3 => state.1.0 -= n,
                _ => unreachable!(),
            },
        }
    }

    let result = state.1.0 + state.1.1;
    Ok(result.try_into().unwrap())
}

fn turn(current: i32, turn: char, degrees: i32) -> i32 {
    let steps = degrees / 90;
    match turn {
        'L' => (current + 4 - steps) % 4,
        'R' => (current + steps) % 4,
        _ => current,
    }
}
