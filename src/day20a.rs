use crate::AoCResult;
use std::fs;

pub fn solve_day20a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day20a_simple.txt")?;
    let data: Vec<(usize, &str)> = file
        .split("\n\n")
        .map(|chunk| -> AoCResult<_> {
            let (tile_line, tile_data) = chunk.split_once("\n").ok_or("data chunk can't split")?;

            let (_, tile_num) = tile_line
                .strip_suffix(":")
                .ok_or("missing colon suffix")?
                .split_once(" ")
                .ok_or("missing space in title line")?;

            let tile_num = tile_num.parse::<usize>()?;

            Ok((tile_num, tile_data))
        })
        .collect::<Result<_, _>>()?;
    eprintln!("gopro[417]: day20a.rs:6: data={:#?}", data);
    Ok(0)
}
