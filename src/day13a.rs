use crate::AoCResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day13a() -> AoCResult<usize> {
    let file = File::open("data/input_day13a.txt")?;
    let reader = BufReader::new(file);

    let mut line_iter = reader.lines();
    let minutes: usize = line_iter.next().unwrap().unwrap().parse().unwrap();
    let line2 = line_iter.next().unwrap().unwrap();
    let schedules: Vec<usize> = line2
        .split(',')
        .filter(|&id| id != "x")
        .map(|id| id.parse().unwrap())
        .collect();

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

    let (wait_time, s) = (minutes..)
        .find_map(|timestamp| {
            schedules
                .iter()
                .find(|&&s| timestamp % s == 0)
                .map(|&s| (timestamp, s))
        })
        .unwrap();

    let result = (wait_time - minutes) * s;
    Ok(result)
}
