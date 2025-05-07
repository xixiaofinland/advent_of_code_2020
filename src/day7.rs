use crate::AoCResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Bag {
    parent: String,
    children: Vec<(usize, String)>,
}

pub fn solve_day7a() -> AoCResult<usize> {
    let file = File::open("data/input_day7a_simple.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let graph = parse(&line);
    }
    Ok(0)
}

fn parse(line: &str) -> (String, Vec<(usize, String)>) {
    let (parent, children) = line
        .strip_suffix('.')
        .unwrap()
        .split_once(" contain ")
        .unwrap();
    let parent = parent.strip_suffix(" bags").unwrap().to_string();

    let children_vec = if children == "no other bags" {
        vec![]
    } else {
        children
            .split(", ")
            .map(|c| {
                let mut words = c.split_whitespace();
                let count = words.next().unwrap().parse::<usize>().unwrap();
                let color = words.take(2).collect::<Vec<_>>().join(" ");
                (count, color)
            })
            .collect()
    };
    (parent, children_vec)
}

pub fn solve_day7b() -> AoCResult<usize> {
    Ok(0)
}
