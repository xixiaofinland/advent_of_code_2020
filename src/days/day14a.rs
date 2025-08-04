use crate::AoCResult;
use std::{
    collections::HashMap, fs::File, io::{BufRead, BufReader}
};

pub fn solve_day14a() -> AoCResult<usize> {
    let file = File::open("data/input_day14a.txt")?;
    let reader = BufReader::new(file);

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("mask") {
            current_mask = line.split(" = ").nth(1).unwrap().to_string();
        } else if line.starts_with("mem") {
            let left_bracket = line.find('[').unwrap();
            let right_bracket = line.find(']').unwrap();
            let addr: u64 = line[left_bracket + 1..right_bracket].parse().unwrap();
            let value_str = line.split(" = ").nth(1).unwrap();
            let value: u64 = value_str.parse().unwrap();
            let masked_value = apply_mask(&current_mask, value);
            memory.insert(addr, masked_value);
        }
    }

    let sum: u64 = memory.values().sum();
    Ok(sum as usize)
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
