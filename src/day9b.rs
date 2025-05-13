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
        .filter_map(|line_result| line_result.ok()?.parse::<usize>().ok())
        .collect();

    let mut start: usize = 0;
    let mut end: usize = 1;
    let mut sum = content[0] + content[1];

    while end < content.len() {
        if sum < NUM {
            end += 1;
            sum += content[end];
        } else if sum > NUM {
            sum -= content[start];
            start += 1;
        } else {
            let min = content[start..=end].iter().min().unwrap();
            let max = content[start..=end].iter().max().unwrap();
            return Ok(min + max);
        }
    }

    return Err("not found".into());
}
