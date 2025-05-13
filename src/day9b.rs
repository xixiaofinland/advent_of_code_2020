use crate::AoCResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day9b() -> AoCResult<usize> {
    const NUM: usize = 675280050;

    let file = File::open("data/input_day9a.txt")?;
    let reader = BufReader::new(file);

    let content: Vec<_> = reader
        .lines()
        .map(|line| -> AoCResult<usize> { Ok(line?.parse()?) })
        .collect::<Result<_, _>>()?;

    let mut start = 0;
    let mut end = 0;
    let mut sum = 0;

    while end < content.len() {
        if sum < NUM {
            sum += content[end];
            end += 1;
        } else if sum > NUM {
            sum -= content[start];
            start += 1;
        } else {
            let range = &content[start..end];
            let min = range.iter().min().unwrap();
            let max = range.iter().max().unwrap();
            return Ok(min + max);
        }
    }

    Err("not found".into())
}
