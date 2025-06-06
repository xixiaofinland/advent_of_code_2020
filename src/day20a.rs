use crate::AoCResult;
use std::fs;

pub fn solve_day20a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day20a_simple.txt")?;
    let data: Vec<(usize, &str)> = file
        .split("\n\n")
        .map(|chunk| -> AoCResult<_> {
            let tile_line = chunk.lines().next().ok_or("missing title line")?;

            let (_, tile_num) = tile_line
                .strip_suffix(":")
                .ok_or("missing colon suffix")?
                .split_once(" ")
                .ok_or("missing space in title line")?;

            let tile_num = tile_num.parse::<usize>()?;

            Ok((tile_num, chunk.lines().next().ok_or("no tile data")?))
        })
        .collect::<Result<_, _>>()?;
    Ok(0)
}
