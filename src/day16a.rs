use std::{fs, ops::RangeInclusive};

use crate::AoCResult;

pub fn solve_day16a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day16a.txt")?;
    let mut chunks = file.split("\n\n");

    let ranges: Vec<RangeInclusive<usize>> = chunks
        .next()
        .unwrap()
        .lines()
        .flat_map(|line| {
            let (_, rest) = line.split_once(": ").unwrap();
            rest.split(" or").map(|part| {
                let (start, end) = part.trim().split_once('-').unwrap();
                start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
            })
        })
        .collect();

    let nearby_values = chunks
        .skip(1)
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .flat_map(|line| line.split(',').filter_map(|n| n.parse::<usize>().ok()))
        .collect::<Vec<_>>();

    let result = nearby_values
        .iter()
        .filter(|&v| !ranges.iter().any(|r| r.contains(v)))
        .sum();

    Ok(result)
}

pub fn solve_day16a_my() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day16a.txt")?;
    let mut chunks = file.split("\n\n");

    let mut ranges: Vec<std::ops::RangeInclusive<usize>> = Vec::new();
    for line in chunks.next().unwrap().split("\n") {
        let mut nums_iter = line.split(": ").skip(1).next().unwrap().split(" or ");

        let mut nums = nums_iter.next().unwrap().split("-");
        ranges.push(nums.next().unwrap().parse()?..=nums.next().unwrap().parse()?);

        let mut nums = nums_iter.next().unwrap().split("-");
        ranges.push(nums.next().unwrap().parse()?..=nums.next().unwrap().parse()?);
    }

    let mut combined = vec![];
    for line in chunks.skip(1).next().unwrap().split("\n").skip(1) {
        let nearby_nums: Vec<usize> = line
            .split(",")
            .filter(|line| !line.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        combined.extend(nearby_nums);
    }

    let result = combined
        .iter()
        .filter(|&c| !ranges.iter().any(|r| r.contains(c)))
        .sum();

    Ok(result)
}
