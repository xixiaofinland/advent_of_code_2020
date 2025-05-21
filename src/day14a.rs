use crate::AoCResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day14a() -> AoCResult<usize> {
    let file = File::open("data/input_day8a_simple.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        println!("{line}");
    }
    Ok(0)
}

fn apply_mask(mask: &str, value: u64) -> u64 {
    let mut or_mask = 0u64;
    let mut and_mask = !0u64; // All bits 1
    for (i, c) in mask.chars().rev().enumerate() {
        match c {
            '1' => or_mask |= 1 << i,
            '0' => and_mask &= !(1 << i),
            'X' => (), // leave unchanged
            _ => panic!("Invalid mask character"),
        }
    }
    (value & and_mask) | or_mask
}
