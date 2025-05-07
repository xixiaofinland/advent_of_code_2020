use crate::AoCResult;
use std::{fs::File, io::BufReader};

pub fn solve_day7a() -> AoCResult<usize> {
    let file = File::open("data/input_day7a_simple.txt")?;
    let reader = BufReader::new(file);
    Ok(0)
}

pub fn solve_day7b() -> AoCResult<usize> {
    Ok(0)
}
