use crate::AoCResult;
use itertools::iproduct;
use std::collections::HashSet;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Coord = (i32, i32, i32, i32);

pub fn solve_day17b() -> AoCResult<usize> {
    let file = File::open("data/input_day17a.txt")?;
    let reader = BufReader::new(file);

    let mut active = HashSet::new();
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                active.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    for _ in 0..6 {
        active = simulate_cycle(&active);
    }

    Ok(active.len())
}

fn simulate_cycle(active: &HashSet<Coord>) -> HashSet<Coord> {
    let mut neighbor_counts = std::collections::HashMap::new();

    for &cube in active {
        for neighbor in get_neighbors_iter(cube) {
            *neighbor_counts.entry(neighbor).or_insert(0) += 1;
        }
    }

    neighbor_counts
        .into_iter()
        .filter_map(|(coord, count)| {
            if count == 3 || (count == 2 && active.contains(&coord)) {
                Some(coord)
            } else {
                None
            }
        })
        .collect()
}

fn get_neighbors_iter((x, y, z, w): Coord) -> Vec<Coord> {
    iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
        .filter(|&(dx, dy, dz, dw)| dx != 0 || dy != 0 || dz != 0 || dw != 0)
        .map(|(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
        .collect()
}
