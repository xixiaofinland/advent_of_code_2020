use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::AoCResult;

pub fn solve_day2a() -> AoCResult<usize> {
    let reader = BufReader::new(File::open("data/input_day2a.txt")?);

    let mut count = 0;

    for line in reader.lines() {
        let line = line?;

        let (range, char_part, password) = match line.split_whitespace().collect::<Vec<_>>()[..] {
            [range, char_part, password] => (range, char_part, password),
            _ => return Err("Invalid line format".into()),
        };

        let (min, max) = {
            let (min, max) = range.split_once('-').ok_or("Invalid range format")?;
            (min.parse::<usize>()?, max.parse::<usize>()?)
        };

        let target_char = char_part.chars().next().ok_or("Empty character part")?;

        let count_match = password.chars().filter(|&c| c == target_char).count();

        if (min..=max).contains(&count_match) {
            count += 1;
        }
    }

    Ok(count)
}

pub fn solve_day2b() -> AoCResult<usize> {
    let reader = BufReader::new(File::open("data/input_day2a.txt")?);

    let mut count = 0;

    for line in reader.lines() {
        let line = line?;

        let (range, char_part, password) = match line.split_whitespace().collect::<Vec<_>>()[..] {
            [range, char_part, password] => (range, char_part, password),
            _ => return Err("Invalid line format".into()),
        };

        let (first, second) = {
            let (first, second) = range.split_once('-').ok_or("Invalid range format")?;
            (first.parse::<usize>()? - 1, second.parse::<usize>()? - 1)
        };

        let target_char = char_part.chars().next().ok_or("Empty character part")?;
        let password_vec: Vec<_> = password.chars().collect();

        let first_check = password_vec
            .get(first)
            .ok_or("no item on this index")?
            .eq(&target_char);
        let second_check = password_vec
            .get(second)
            .ok_or("no item on this index")?
            .eq(&target_char);

        if first_check ^ second_check {
            count += 1;
        }
    }

    Ok(count)
}

