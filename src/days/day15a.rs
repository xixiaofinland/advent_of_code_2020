use crate::AoCResult;
use std::collections::HashMap;

pub fn solve_day15a() -> AoCResult<usize> {
    let starting_numbers = [0, 20, 7, 16, 1, 18, 15];
    let mut last_seen = HashMap::new();

    let mut iter = starting_numbers.iter().copied().enumerate();
    for (turn, num) in iter.by_ref().take(starting_numbers.len() - 1) {
        last_seen.insert(num, turn + 1);
    }

    let (_, mut current) = iter.next().unwrap(); // last starting number

    for turn in starting_numbers.len()..30000000 {
        let next = match last_seen.insert(current, turn) {
            Some(prev_turn) => turn - prev_turn,
            None => 0,
        };
        current = next;
    }

    Ok(current)
}

pub fn solve_day15a_my() -> AoCResult<usize> {
    let mut nums = HashMap::new();

    let init_inputs = [0, 20, 7, 16, 1, 18];

    init_inputs.iter().enumerate().for_each(|(i, &n)| {
        nums.insert(n, i + 1);
    });

    let mut num = 15;
    let mut index = init_inputs.len() + 1;

    while index <= 2020 {
        if let Some(v) = nums.insert(num, index) {
            num = index - v;
        } else {
            num = 0;
        }
        index += 1;
    }

    Ok(num)
}
