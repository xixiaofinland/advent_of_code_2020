use std::fs;

use crate::AoCResult;

pub fn solve_day16a() -> AoCResult<usize> {
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
        .filter(|&c| {
            let check_result = !ranges.iter().any(|r| r.contains(c));
            check_result
        })
        .sum();

    Ok(result)
}
