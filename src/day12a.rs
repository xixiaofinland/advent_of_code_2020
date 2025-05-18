use crate::AoCResult;
use std::{fs::File, io::{BufRead, BufReader}};

pub struct Instruct{
    direct: Direct,
    num: usize,
}

enum Direct{
    N,
    S,
    W,
    E,
}

// F10
// N3
// F7
// R90
// F11

impl From<String> for Instruct {
    fn from(s: String) -> Self {
        let (direct, num) = s.split_at(1);
        let direct = match direct {
            "N" => Direct::N,
            "S" => Direct::S,
            "W" => Direct::W,
            "E" => Direct::E,
            _ => panic!("Invalid direction"),
        };
        let num = num.parse::<usize>().expect("Invalid number");
        Instruct { direct, num }
    }
}

pub fn solve_day12a() -> AoCResult<usize> {
    let file = File::open("data/input_day12a_simple.txt")?;
    let reader = BufReader::new(file);

    let instructions = reader.lines().map(|line_result| -> AoCResult<Instruct>{
        let line = line_result?;

    })
    Ok(0)
}

