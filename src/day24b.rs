use crate::AoCResult;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hex(i32, i32); // (q, r)

#[derive(Debug, Clone, Copy)]
enum Direction {
    E, W, NE, NW, SE, SW
}

impl Direction {
    fn delta(self) -> (i32, i32) {
        match self {
            Direction::E  => (1, 0),
            Direction::W  => (-1, 0),
            Direction::NE => (1, -1),
            Direction::NW => (0, -1),
            Direction::SE => (0, 1),
            Direction::SW => (-1, 1),
        }
    }
}

fn neighbors(tile: Hex) -> Vec<Hex> {
    use Direction::*;
    [E, W, NE, NW, SE, SW]
        .iter()
        .map(|&dir| {
            let (dq, dr) = dir.delta();
            Hex(tile.0 + dq, tile.1 + dr)
        })
        .collect()
}

pub fn solve_day24b() -> AoCResult<usize> {
    let input = fs::read_to_string("data/input_day24a.txt")?;
    let mut black_tiles = HashSet::new();

    for line in input.lines() {
        let dirs = parse_directions(line);
        let tile = walk_path(&dirs);
        if !black_tiles.insert(tile) {
            black_tiles.remove(&tile); // flip back to white
        }
    }

    for _ in 0..100 {
        let mut neighbor_black_count = HashMap::new();
        let mut new_black_tiles = HashSet::new();

        for &tile in &black_tiles {
            let mut black_neighbor_count = 0;
            for neighbor in neighbors(tile) {
                if black_tiles.contains(&neighbor) {
                    black_neighbor_count += 1;
                } else {
                    *neighbor_black_count.entry(neighbor).or_insert(0) += 1;
                }
            }
            if black_neighbor_count == 1 || black_neighbor_count == 2 {
                new_black_tiles.insert(tile);
            }
        }

        for (tile, count) in neighbor_black_count {
            if count == 2 {
                new_black_tiles.insert(tile);
            }
        }

        black_tiles = new_black_tiles;
    }

    Ok(black_tiles.len())
}

fn parse_directions(line: &str) -> Vec<Direction> {
    let mut chars = line.chars();
    let mut directions = Vec::new();

    while let Some(c) = chars.next() {
        let dir = match c {
            'e' => Direction::E,
            'w' => Direction::W,
            'n' => {
                let next = chars.next().unwrap();
                match next {
                    'e' => Direction::NE,
                    'w' => Direction::NW,
                    _ => panic!("invalid direction"),
                }
            }
            's' => {
                let next = chars.next().unwrap();
                match next {
                    'e' => Direction::SE,
                    'w' => Direction::SW,
                    _ => panic!("invalid direction"),
                }
            }
            _ => panic!("invalid char"),
        };
        directions.push(dir);
    }

    directions
}

fn walk_path(directions: &[Direction]) -> Hex {
    let mut q = 0;
    let mut r = 0;
    for dir in directions {
        let (dq, dr) = dir.delta();
        q += dq;
        r += dr;
    }
    Hex(q, r)
}

