use core::num;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::AoCResult;

pub fn solve_day1a() -> AoCResult<Option<usize>> {
    let file = File::open("data/input_day1a.txt")?;
    let reader = BufReader::new(file);

    let numbers: Vec<usize> = reader
        .lines()
        .filter_map(|line| line.ok()?.trim().parse().ok())
        .collect();

    let mut seen = HashSet::new();

    for num in numbers {
        let complement = 2020_usize.checked_sub(num);
        if let Some(c) = complement {
            let has_seen = seen.contains(&c);
            if has_seen {
                return Ok(Some(num * c));
            }
        }
        seen.insert(num);
    }

    Ok(None)
}

pub fn solve_day1b() -> AoCResult<Option<usize>> {
    let file = File::open("data/input_day1a.txt")?;
    let reader = BufReader::new(file);

    let numbers: Vec<usize> = reader
        .lines()
        .filter_map(|line| line.ok()?.trim().parse().ok())
        .collect();

    for (i, &a) in numbers.iter().enumerate() {
        let target: usize = 2020 - a;

        let mut seen = HashSet::new();
        for &b in &numbers[i + 1..] {
            let complement = target.checked_sub(b);
            if let Some(c) = complement {
                let has_seen = seen.contains(&c);
                if has_seen {
                    return Ok(Some(a * b * c));
                }
            }
            seen.insert(b);
        }
    }

    Ok(None)
}
