use crate::AoCResult;
use std::{
    collections::HashSet,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day9a() -> AoCResult<usize> {
    // let file = File::open("data/input_day9a_simple.txt")?;
    let file = File::open("data/input_day9a.txt")?;
    let reader = BufReader::new(file);

    let content: Vec<_> = reader
        .lines()
        .filter_map(|line_result| line_result.ok()?.parse::<usize>().ok())
        .collect();

    let queue_size = 25;
    let mut queue = VecDeque::from(content[0..queue_size].to_vec());

    for num in content[queue_size..].iter().copied() {
        if validate(&queue, num) {
            queue.pop_front();
            queue.push_back(num);
        } else {
            return Ok(num);
        }
    }

    Ok(0)
}

fn validate(queue: &VecDeque<usize>, num: usize) -> bool {
    let mut seen = HashSet::new();
    queue.iter().any(|&x| {
        let complement = num.saturating_sub(x);
        if seen.contains(&complement) {
            true
        } else {
            seen.insert(x);
            false
        }
    })
}
