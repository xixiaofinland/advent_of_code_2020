use crate::AoCResult;
use std::{fs::File, io::{BufRead, BufReader}};

pub fn solve_day8a() -> AoCResult<usize> {
    let file = File::open("data/input_day8a_simple.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line = line?;
        println!("{line}");
    }
    Ok(0)
}
