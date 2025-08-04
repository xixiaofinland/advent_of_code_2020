use crate::AoCResult;
use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Rule {
    Literal(char),
    Sequence(Vec<usize>),
    Alternative(Vec<Vec<usize>>),
}

pub fn solve_day19a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day19a.txt")?;
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

    let valid_count = to_checkes
        .iter()
        .filter(|&&message| match_rule(0, &rules_map, message).contains(&""))
        .count();

    Ok(valid_count)
}

fn match_rule_imperative<'a>(
    rule_id: usize,
    rules: &HashMap<usize, Rule>,
    message: &'a str,
) -> Vec<&'a str> {
    match &rules[&rule_id] {
        Rule::Literal(c) => {
            if let Some(remainder) = message.strip_prefix(*c) {
                vec![remainder]
            } else {
                vec![]
            }
        }

        Rule::Sequence(seq) => {
            let mut remainders = vec![message];

            for &rule in seq {
                let mut next_remainders = Vec::new();

                for msg in &remainders {
                    let results = match_rule(rule, rules, msg);
                    next_remainders.extend(results);
                }

                remainders = next_remainders;

                // Early exit if nothing matched
                if remainders.is_empty() {
                    break;
                }
            }

            remainders
        }

        Rule::Alternative(alternatives) => {
            let mut all_remainders = Vec::new();

            for seq in alternatives {
                let mut remainders = vec![message];

                for &rule in seq {
                    let mut next_remainders = Vec::new();

                    for msg in &remainders {
                        let results = match_rule(rule, rules, msg);
                        next_remainders.extend(results);
                    }

                    remainders = next_remainders;

                    if remainders.is_empty() {
                        break;
                    }
                }

                all_remainders.extend(remainders);
            }

            all_remainders
        }
    }
}

fn match_rule<'a>(rule_id: usize, rules: &HashMap<usize, Rule>, message: &'a str) -> Vec<&'a str> {
    match &rules[&rule_id] {
        Rule::Literal(c) => message
            .strip_prefix(*c)
            .map_or(vec![], |remainder| vec![remainder]),

        Rule::Sequence(seq) => seq.iter().fold(vec![message], |acc, &rule| {
            acc.into_iter()
                .flat_map(|msg| match_rule(rule, rules, msg))
                .collect()
        }),

        Rule::Alternative(alternatives) => alternatives
            .iter()
            .flat_map(|seq| {
                seq.iter().fold(vec![message], |acc, &rule| {
                    acc.into_iter()
                        .flat_map(|msg| match_rule(rule, rules, msg))
                        .collect()
                })
            })
            .collect(),
    }
}
