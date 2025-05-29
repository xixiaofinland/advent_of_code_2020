use crate::AoCResult;
use std::collections::HashSet;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Coord = (i32, i32, i32);

pub fn solve_day17a() -> AoCResult<usize> {
    let file = File::open("data/input_day17a_simple.txt")?;
    let reader = BufReader::new(file);

    let mut active = HashSet::new();
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                active.insert((x as i32, y as i32, 0)); // z=0 for initial state
            }
        }
    }

    for _ in 0..6 {
        active = simulate_cycle(&active);
    }

    Ok(active.len())
}

fn get_neighbors(coord: Coord) -> Vec<Coord> {
    let mut neighbors = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if dx != 0 || dy != 0 || dz != 0 {
                    neighbors.push((coord.0 + dx, coord.1 + dy, coord.2 + dz));
                }
            }
        }
    }
    neighbors
}

fn simulate_cycle(active: &HashSet<Coord>) -> HashSet<Coord> {
    let mut neighbor_counts = std::collections::HashMap::new();

    for &cube in active {
        for neighbor in get_neighbors(cube) {
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
