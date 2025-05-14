use crate::AoCResult;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day10a() -> AoCResult<usize> {
    let file = File::open("data/input_day10a.txt")?;
    let reader = BufReader::new(file);

    let mut content: Vec<_> = reader
        .lines()
        .map(|line| -> AoCResult<usize> { Ok(line?.parse()?) })
        .collect::<Result<_, _>>()?;
    content.sort();

    let mut map: HashMap<usize, usize> = HashMap::new();
    for (i, c) in content.iter().copied().enumerate() {
        if i == 0 {
            *map.entry(c).or_default() += 1;
        } else {
            *map.entry(c - content[i - 1]).or_default() += 1;
        }
    }

    *map.entry(3).or_default() += 1;

    Ok(map.get(&1).unwrap() * map.get(&3).unwrap())
}
