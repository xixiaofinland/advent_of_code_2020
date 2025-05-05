use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::AoCResult;

pub fn solve_day5a() -> AoCResult<usize> {
    let reader = BufReader::new(File::open("data/input_day5a.txt")?);
    let max_id = reader
        .lines()
        .map(|line| line.map(|l| parse_ticket(&l)))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .max()
        .unwrap_or(0);

    Ok(max_id)
}

fn parse_ticket(pass: &str) -> usize {
    let (a, b) = pass.split_at(pass.len().saturating_sub(3));

    let mut seat_range = (0, 127);

    for c in a.chars() {
        seat_range = match c {
            'F' => (
                seat_range.0,
                seat_range.0 + (seat_range.1 - seat_range.0) / 2,
            ),
            'B' => (
                seat_range.0 + (seat_range.1 - seat_range.0 + 1) / 2,
                seat_range.1,
            ),
            _ => panic!(),
        };
    }

    let mut column = (0, 7);
    for d in b.chars() {
        column = match d {
            'L' => (column.0, column.0 + (column.1 - column.0) / 2),
            'R' => (column.0 + (column.1 - column.0 + 1) / 2, column.1),
            _ => panic!(),
        };
    }

    seat_range.0 * 8 + column.0
}
