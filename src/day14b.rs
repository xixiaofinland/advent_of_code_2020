use crate::AoCResult;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day14b() -> AoCResult<usize> {
    let file = File::open("data/input_day14a_simple.txt")?;
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
            apply_mask(addr, &current_mask, value, &mut memory);
        }
    }

    let sum: u64 = memory.values().sum();
    Ok(sum as usize)
}

fn apply_mask(addr: u64, mask: &str, value: u64, memory: &mut HashMap<u64, u64>) {
    let mut base_addr = addr;
    let mut floating_bits = vec![];

    for (i, c) in mask.chars().rev().enumerate() {
        match c {
            '0' => {}                     // leave unchanged
            '1' => base_addr |= 1 << i,   // force to 1
            'X' => floating_bits.push(i), // floating bit
            _ => panic!("Invalid mask character"),
        }
    }

    let combinations = 1 << floating_bits.len(); // 2^n combinations
    for n in 0..combinations {
        let mut floating_addr = base_addr;
        for (bit_idx, &bit_pos) in floating_bits.iter().enumerate() {
            let bit_val = (n >> bit_idx) & 1;
            if bit_val == 1 {
                floating_addr |= 1 << bit_pos;
            } else {
                floating_addr &= !(1 << bit_pos);
            }
        }
        memory.insert(floating_addr, value);
    }
}
