use crate::AoCResult;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn solve_day11a() -> AoCResult<usize> {
    let file = File::open("data/input_day11a_simple.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| -> AoCResult<Vec<char>> {
            let l = line?;
            Ok(l.chars().collect())
        })
        .collect::<Result<_, _>>()?;

    let mut changed = true;
    let mut next_grid = grid.clone();

    while changed {
        changed = false;
        next_grid = grid.clone();

        for (i, row) in grid.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                calculate(i, j, &grid, &mut next_grid, &mut changed);
            }
        }

        grid = next_grid.clone();
    }

    for row in &next_grid {
        println!("{}", row.iter().collect::<String>());
    }

    Ok(next_grid
        .iter()
        .map(|row| row.iter().filter(|&&c| c == 'L').count())
        .sum())
}

fn calculate(
    row: usize,
    col: usize,
    grid: &[Vec<char>],
    result: &mut [Vec<char>],
    has_grid_updated: &mut bool,
) {
    if matches!(grid[row][col], 'L') && !has_occupied_adjacent(row, col, grid) {
        result[row][col] = '#';
        *has_grid_updated = true;
    } else if matches!(grid[row][col], '#') && four_or_more_adjacent_occupied(row, col, grid) {
        result[row][col] = 'L';
        *has_grid_updated = true;
    }

    // let cell_value = grid[row][col];
    // match cell_value {
    //     'L' => {
    //         if !has_occupied_adjacent(row, col, grid) {
    //             result[row][col] = '#';
    //             *has_grid_updated = true;
    //         }
    //     }
    //     '#' => {
    //         if four_or_more_adjacent_occupied(row, col, grid) {
    //             result[row][col] = 'L';
    //             *has_grid_updated = true;
    //         }
    //     }
    //     _ => {}
    // }
}

fn has_occupied_adjacent(row: usize, col: usize, grid: &[Vec<char>]) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let row = row as isize;
    let col = col as isize;

    DIRECTIONS.iter().any(|(dr, dc)| {
        let nr = row + dr;
        let nc = col + dc;

        if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
            grid[nr as usize][nc as usize] == '#'
        } else {
            false
        }
    })
}

fn four_or_more_adjacent_occupied(row: usize, col: usize, grid: &[Vec<char>]) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let row = row as isize;
    let col = col as isize;

    let count = DIRECTIONS
        .iter()
        .filter(|&&(dr, dc)| {
            let nr = row + dr;
            let nc = col + dc;

            nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr as usize][nc as usize] == '#'
        })
        .count();

    count >= 4

    // let rows = grid.len() as isize;
    // let cols = grid[0].len() as isize;
    // let row = row as isize;
    // let col = col as isize;
    //
    // let count = directions.iter().fold(0, |acc, (dr, dc)| {
    //     let nr = row + dr;
    //     let nc = col + dc;
    //
    //     if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
    //         if grid[nr as usize][nc as usize] == '#' {
    //             acc + 1
    //         } else {
    //             acc
    //         }
    //     } else {
    //         acc
    //     }
    // });
    //
    // count >= 4
}
