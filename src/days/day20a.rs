use crate::AoCResult;
use std::{collections::HashMap, fs};

pub fn solve_day20a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day20a.txt")?;
    let data: Vec<(usize, Vec<Vec<u8>>)> = file
        .split("\n\n")
        .map(|chunk| -> AoCResult<_> {
            let (tile_line, tile_data) = chunk.split_once("\n").ok_or("data chunk can't split")?;

            let (_, tile_num) = tile_line
                .strip_suffix(":")
                .ok_or("missing colon suffix")?
                .split_once(" ")
                .ok_or("missing space in title line")?;

            let tile_num = tile_num.parse::<usize>()?;

            let tile_data: Vec<Vec<u8>> = tile_data
                .lines()
                .map(|line| line.chars().map(|c| if c == '.' { 0 } else { 1 }).collect())
                .collect::<Vec<Vec<_>>>();
            Ok((tile_num, tile_data))
        })
        .collect::<Result<_, _>>()?;

    let get_edges = |tile: &[Vec<u8>]| -> Vec<Vec<u8>> {
        let top = tile[0].clone();
        let bottom = tile[tile.len() - 1].clone();
        let left: Vec<_> = tile.iter().map(|row| row[0]).collect();
        let right: Vec<_> = tile.iter().map(|row| row[row.len() - 1]).collect();
        vec![
            top.clone(),
            bottom.clone(),
            left.clone(),
            right.clone(),
            top.into_iter().rev().collect(),
            bottom.into_iter().rev().collect(),
            left.into_iter().rev().collect(),
            right.into_iter().rev().collect(),
        ]
    };

    // Map from tile ID to its 4 edges (non-reversed)
    let mut tile_to_edges: HashMap<usize, Vec<Vec<u8>>> = HashMap::new();
    let mut edge_freq: HashMap<Vec<u8>, usize> = HashMap::new();

    for (tile_id, tile_data) in data {
        let edges = get_edges(&tile_data);
        tile_to_edges.insert(tile_id, edges[..4].to_vec()); // only store non-reversed

        // count all 8 forms of edges (including reversed) to find matches
        for edge in edges {
            *edge_freq.entry(edge).or_insert(0) += 1;
        }
    }

    // Corner tiles have exactly 2 edges that appear only once
    let corner_tiles: Vec<usize> = tile_to_edges
        .iter()
        .filter(|(_, edges)| {
            edges
                .iter()
                .filter(|e| edge_freq.get(*e).copied().unwrap_or(0) == 1)
                .count()
                == 2
        })
        .map(|(&id, _)| id)
        .collect();

    let product = corner_tiles.iter().product();
    Ok(product)
}
