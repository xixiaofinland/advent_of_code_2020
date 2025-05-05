use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::AoCResult;

pub fn solve_day5a() -> AoCResult<usize> {
    let reader = BufReader::new(File::open("data/input_day5a_simple.txt")?);
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(0)
}
