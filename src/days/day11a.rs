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
    let file = File::open("data/input_day11a.txt")?;
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
        // next_grid = grid.clone();

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                calculate(i, j, &grid, &mut next_grid, &mut changed);
            }
        }

        std::mem::swap(&mut grid, &mut next_grid);
    }

    display_grid(&grid);

    Ok(grid
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum())
}

fn calculate(
    row: usize,
    col: usize,
    grid: &[Vec<char>],
    next_grid: &mut [Vec<char>],
    has_grid_updated: &mut bool,
) {
    if grid[row][col] == 'L' && !has_occupied_adjacent(row, col, grid) {
        next_grid[row][col] = '#';
        *has_grid_updated = true;
    } else if grid[row][col] == '#' && four_or_more_adjacent_occupied(row, col, grid) {
        next_grid[row][col] = 'L';
        *has_grid_updated = true;
    } else {
        // important! As we do std::mem:swap() without clone() before each for loop step, this is
        // needed to avoid stale values
        next_grid[row][col] = grid[row][col];
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

fn display_grid(grid: &[Vec<char>]) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
