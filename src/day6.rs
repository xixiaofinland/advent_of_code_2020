use crate::AoCResult;
use std::{fs::File, io::BufReader};

pub fn solve_day6a() -> AoCResult<usize> {
    let file = File::open("data/input_day2a.txt")?;
    let reader = BufReader::new(file);
    Ok(0)
}

// pub fn solve_day6b() -> AoCResult<Option<usize>> {}
