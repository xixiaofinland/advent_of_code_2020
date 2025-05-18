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

pub fn solve_day11b() -> AoCResult<usize> {
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

    // display_grid(&grid);

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
    } else if grid[row][col] == '#' && five_or_more_adjacent_occupied(row, col, grid) {
        next_grid[row][col] = 'L';
        *has_grid_updated = true;
    } else {
        next_grid[row][col] = grid[row][col];
    }
}

fn has_occupied_adjacent(row: usize, col: usize, grid: &[Vec<char>]) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let row = row as isize;
    let col = col as isize;

    DIRECTIONS.iter().any(|(dr, dc)| {
        let mut nr = row + dr;
        let mut nc = col + dc;

        while nr >= 0 && nr < rows && nc >= 0 && nc < cols {
            match grid[nr as usize][nc as usize] {
                '#' => return true,
                'L' => break,
                '.' => {
                    nr += dr;
                    nc += dc;
                }
                _ => break,
            }
        }
        false
    })
}

fn five_or_more_adjacent_occupied(row: usize, col: usize, grid: &[Vec<char>]) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let row = row as isize;
    let col = col as isize;

    let mut count = 0;

    for (dr, dc) in DIRECTIONS.iter() {
        let mut nr = row + dr;
        let mut nc = col + dc;

        while nr >= 0 && nr < rows && nc >= 0 && nc < cols {
            match grid[nr as usize][nc as usize] {
                '#' => {
                    count += 1;
                    break;
                }
                'L' => break,
                '.' => {
                    nr += dr;
                    nc += dc;
                }
                _ => break,
            }
        }

        if count >= 5 {
            return true;
        }
    }

    false
}

fn display_grid(grid: &[Vec<char>]) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
