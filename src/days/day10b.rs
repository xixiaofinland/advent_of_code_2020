use crate::AoCResult;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day10b() -> AoCResult<usize> {
    let file = File::open("data/input_day10a.txt")?;
    let reader = BufReader::new(file);

    let mut adapters: Vec<_> = reader
        .lines()
        .map(|line| -> AoCResult<usize> { Ok(line?.parse()?) })
        .collect::<Result<_, _>>()?;

    adapters.sort_unstable();

    let mut ways = HashMap::from([(0, 1_usize)]);

    for &adapter in &adapters {
        let c_sum = (1..=3)
            .filter_map(|diff| adapter.checked_sub(diff))
            .filter_map(|prev| ways.get(&prev))
            .sum();

        ways.insert(adapter, c_sum);
    }

    adapters
        .last()
        .and_then(|jolt| ways.get(jolt).copied())
        .ok_or_else(|| "No adapters found".into())

    // adapters
    //     .last()
    //     .map(|&jolt| ways[&jolt])
    //     .ok_or_else(|| "No adapters found".into())
}
