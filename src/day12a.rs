use crate::AoCResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Instruct {
    action: Action,
}

#[derive(Debug)]
enum Action {
    N(usize),
    S(usize),
    W(usize),
    E(usize),
    F(usize),
    L(usize),
    R(usize),
}

impl From<String> for Instruct {
    fn from(s: String) -> Self {
        let (action, num) = s.split_at(1);
        let num = num.parse::<usize>().expect("Invalid number");

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

pub fn solve_day12a() -> AoCResult<usize> {
    let file = File::open("data/input_day12a_simple.txt")?;
    let reader = BufReader::new(file);

    let instructions = reader
        .lines()
        .map(|line_result| -> AoCResult<Instruct> {
            let line = line_result?;
            Ok(Instruct::from(line))
        })
        .collect::<Result<Vec<_>, _>>()?;

    eprintln!("gopro[369]: day12a.rs:47: instructions={:#?}", instructions);
    Ok(0)
}
