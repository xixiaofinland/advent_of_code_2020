use crate::AoCResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Schedule {
    num: usize,
    time_offset: usize,
}
pub fn solve_day13b() -> AoCResult<usize> {
    let file = File::open("data/input_day13a_simple.txt")?;
    let reader = BufReader::new(file);

    let mut line_iter = reader.lines().flatten();
    let earliest: usize = line_iter.next().unwrap().parse().unwrap();
    let line2 = line_iter.next().unwrap();

    // let schedules: Vec<(usize, usize)> = line2.split(',').map(|id| ).collect();
    let schedules: Vec<Schedule> = line2
        .split(',')
        .enumerate()
        .filter_map(|(i, id)| {
            if id == "x" {
                None
            } else {
                Some(Schedule {
                    num: id.parse().unwrap(),
                    time_offset: i,
                })
            }
        })
        .collect();

    eprintln!("gopro[378]: day13b.rs:21: schedules={:#?}", schedules);

    // let (timestamp, bus_id) = (earliest..)
    //     .find_map(|timestamp| {
    //         schedules
    //             .iter()
    //             .find(|&&s| timestamp % s == 0)
    //             .map(|&s| (timestamp, s))
    //     })
    //     .unwrap();
    //
    // let result = (timestamp - earliest) * bus_id;
    Ok(0)
}
