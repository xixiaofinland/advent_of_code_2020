use crate::AoCResult;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    let mut has_grid_updated = true;
    let mut result = grid.clone();

    while has_grid_updated {
        has_grid_updated = false;

        for (i, row) in grid.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                calculate(i, j, &grid, &mut result, &mut has_grid_updated);
            }
        }

        grid = result.clone();
    }

    result.iter().for_each(|row| println!("{:#?}", row));

    Ok(result
        .iter()
        .map(|row| row.iter().filter(|&&c| c == 'L').count())
        .sum())
}

fn calculate(
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>,
    result: &mut Vec<Vec<char>>,
    has_grid_updated: &mut bool,
) {
    let cell_value = grid[row][col];
    match cell_value {
        'L' => {
            if !has_occupied_adjacent(row, col, grid) {
                result[row][col] = '#';
                *has_grid_updated = true;
            }
        }
        '#' => {
            if four_or_more_adjacent_occupied(row, col, grid) {
                result[row][col] = 'L';
                *has_grid_updated = true;
            }
        }
        _ => {}
    }
}

fn has_occupied_adjacent(row: usize, col: usize, grid: &Vec<Vec<char>>) -> bool {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let row = row as isize;
    let col = col as isize;

    directions.iter().any(|(dr, dc)| {
        let nr = row + dr;
        let nc = col + dc;

        if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
            grid[nr as usize][nc as usize] == '#'
        } else {
            false
        }
    })
}

fn four_or_more_adjacent_occupied(row: usize, col: usize, grid: &Vec<Vec<char>>) -> bool {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let row = row as isize;
    let col = col as isize;

    let count = directions
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
