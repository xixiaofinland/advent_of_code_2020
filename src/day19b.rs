use crate::AoCResult;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug)]
enum Rule {
    Literal(char),
    Sequence(Vec<usize>),
    Alternative(Vec<Vec<usize>>),
}

pub fn solve_day19b() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day19b_simple.txt")?;
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

    let mut memo = HashMap::new();
    let rule42 = generate_matches(42, &rules_map, &mut memo);
    let rule31 = generate_matches(31, &rules_map, &mut memo);

    let valid_count = to_checkes
        .iter()
        .filter(|&&msg| is_valid_message(msg, &rule42, &rule31))
        .count();

    Ok(valid_count)
}

fn generate_matches<'a>(
    rule_id: usize,
    rules: &HashMap<usize, Rule>,
    memo: &mut HashMap<usize, HashSet<String>>,
) -> HashSet<String> {
    if let Some(cached) = memo.get(&rule_id) {
        return cached.clone();
    }

    let result = match &rules[&rule_id] {
        Rule::Literal(c) => HashSet::from([c.to_string()]),

        Rule::Sequence(seq) => {
            let mut sets = vec![HashSet::from(["".to_string()])];
            for &rule in seq {
                let next = generate_matches(rule, rules, memo);
                let mut combined = HashSet::new();
                for a in &sets[0] {
                    for b in &next {
                        combined.insert(format!("{a}{b}"));
                    }
                }
                sets[0] = combined;
            }
            sets.remove(0)
        }

        Rule::Alternative(alts) => alts
            .iter()
            .flat_map(|seq| {
                let mut sets = vec![HashSet::from(["".to_string()])];
                for &rule in seq {
                    let next = generate_matches(rule, rules, memo);
                    let mut combined = HashSet::new();
                    for a in &sets[0] {
                        for b in &next {
                            combined.insert(format!("{a}{b}"));
                        }
                    }
                    sets[0] = combined;
                }
                sets.pop().unwrap()
            })
            .collect(),
    };

    memo.insert(rule_id, result.clone());
    result
}

fn is_valid_message(message: &str, rule42: &HashSet<String>, rule31: &HashSet<String>) -> bool {
    let chunk_len = rule42.iter().next().unwrap().len();
    if message.len() % chunk_len != 0 {
        return false;
    }

    let chunks: Vec<&str> = message
        .as_bytes()
        .chunks(chunk_len)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();

    let mut count_42 = 0;
    let mut count_31 = 0;

    let mut i = 0;
    while i < chunks.len() && rule42.contains(chunks[i]) {
        count_42 += 1;
        i += 1;
    }

    while i < chunks.len() && rule31.contains(chunks[i]) {
        count_31 += 1;
        i += 1;
    }

    // Must use all chunks, and satisfy the rule
    i == chunks.len() && count_42 > count_31 && count_31 >= 1
}
