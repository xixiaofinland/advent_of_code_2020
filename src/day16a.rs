use std::fs;

use crate::AoCResult;

pub fn solve_day16a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day16a_simple.txt")?;
    let mut chunks = file.split("\n\n");

    for line in chunks.next().unwrap().split("\n") {
        let mut nums = line
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(" or ")
            .skip(1)
            .next()
            .unwrap()
            .split("-");
        println!("{}", nums.next().unwrap());
        println!("{}", nums.next().unwrap());
        println!();
    }

    for line in chunks.skip(1).next().unwrap().split("\n") {
        let nearby_nums: Vec<_> = line.split(",").collect();
        eprintln!("gopro[388]: day16a.rs:26: nearby_nums={:#?}", nearby_nums);
    }

    Ok(0)
}
