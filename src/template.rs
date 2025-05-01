use crate::AoCResult;
use std::{fs::File, io::BufReader};

pub fn solve_day2a() -> AoCResult<Option<usize>> {
    let file = File::open("data/input_day2a.txt")?;
    let reader = BufReader::new(file);
}

pub fn solve_day2b() -> AoCResult<Option<usize>> {}
