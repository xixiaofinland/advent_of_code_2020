use crate::AoCResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day13a() -> AoCResult<usize> {
    let file = File::open("data/input_day13a.txt")?;
    let reader = BufReader::new(file);

    let mut line_iter = reader.lines().flatten();
    let earliest: usize = line_iter.next().unwrap().parse().unwrap();
    let line2 = line_iter.next().unwrap();

    // let mut line_iter = reader.lines();
    // let earliest: usize = line_iter.next().unwrap().unwrap().parse().unwrap();
    // let line2 = line_iter.next().unwrap().unwrap();

    let schedules: Vec<usize> = line2.split(',').filter_map(|id| id.parse().ok()).collect();

    // let schedules: Vec<usize> = line2
    //     .split(',')
    //     .filter(|&id| id != "x")
    //     .map(|id| id.parse().unwrap())
    //     .collect();

    // let result = (minutes..)
    //     .into_iter()
    //     .find(|timestamp| {
    //         for s in &schedules {
    //             if timestamp % s == 0 {
    //                 return true;
    //             }
    //         }
    //         false
    //     })
    //     .unwrap();

    let (timestamp, bus_id) = (earliest..)
        .find_map(|timestamp| {
            schedules
                .iter()
                .find(|&&s| timestamp % s == 0)
                .map(|&s| (timestamp, s))
        })
        .unwrap();

    let result = (timestamp - earliest) * bus_id;
    Ok(result)
}
