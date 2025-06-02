use crate::AoCResult;
use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Rule {
    Literal(char),
    Sequence(Vec<usize>),
    Alternative(Vec<Vec<usize>>),
}

pub fn solve_day19a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day19a_simple.txt")?;
    let (rules, combinations) = file.split_once("\n\n").ok_or_else(|| "can't split")?;

    let rules_vec = rules
        .lines()
        .map(|line| -> AoCResult<_> {
            let (first, second) = line
                .split_once(": ")
                .ok_or_else(|| "split 'first: second' fails")?;

            let num = first.parse::<usize>()?;
            let rule = if let Some((rule1, rule2)) = second.split_once(" | ") {
                Rule::Alternative(vec![
                    rule1
                        .split_whitespace()
                        .map(str::parse::<usize>)
                        .collect::<Result<_, _>>()?,
                    rule2
                        .split_whitespace()
                        .map(str::parse::<usize>)
                        .collect::<Result<_, _>>()?,
                ])
            } else if second.starts_with('"') && second.ends_with('"') {
                let c = second.chars().nth(1).ok_or("missing char")?;
                Rule::Literal(c)
            } else {
                Rule::Sequence(
                    second
                        .split(" ")
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect(),
                )
            };

            Ok((num, rule))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let rules_map: HashMap<usize, Rule> = rules_vec.into_iter().collect();
    let to_checkes: Vec<&str> = combinations.lines().collect();

    Ok(0)
}

