use std::{collections::HashMap, fs, ops::RangeInclusive};

use crate::AoCResult;

pub fn solve_day16b() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day16a.txt")?;
    let mut chunks = file.split("\n\n");

    let ranges: Vec<Vec<RangeInclusive<usize>>> = chunks
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(": ").unwrap();
            rest.split(" or")
                .map(|part| {
                    let (start, end) = part.trim().split_once('-').unwrap();
                    start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
                })
                .collect()
        })
        .collect();

    let my_ticket: Vec<usize> = chunks
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let tickets: Vec<Vec<usize>> = chunks
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(|line| {
            let nums: Vec<usize> = line.split(',').filter_map(|n| n.parse().ok()).collect();
            let valid = nums
                .iter()
                .all(|n| ranges.iter().flatten().any(|r| r.contains(n)));
            valid.then_some(nums)
        })
        .collect();

    let mut possible_fields: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..tickets[0].len() {
        let column_values: Vec<usize> = tickets.iter().map(|v| v[i]).collect();
        for (rule_idx, rule_ranges) in ranges.iter().enumerate() {
            if column_values
                .iter()
                .all(|v| rule_ranges.iter().any(|r| r.contains(v)))
            {
                possible_fields.entry(i).or_default().push(rule_idx);
            }
        }
    }

    // Sort fields by number of matching rules (smallest first)
    let mut sorted_fields: Vec<_> = possible_fields.into_iter().collect();
    sorted_fields.sort_by_key(|(_, v)| v.len());

    let mut resolved = Vec::new();
    while let Some((col_idx, candidates)) = sorted_fields.first() {
        if candidates.len() != 1 {
            panic!("Ambiguous field mapping");
        }

        let rule_idx = candidates[0];
        resolved.push((*col_idx, rule_idx));
        sorted_fields.remove(0);
        for (_, v) in &mut sorted_fields {
            v.retain(|&x| x != rule_idx);
        }
        sorted_fields.sort_by_key(|(_, v)| v.len());
    }

    resolved.sort_by_key(|&(_, rule_idx)| rule_idx);

    // Multiply first 6 values (assume "departure" fields are first 6 rules)
    let result = resolved
        .iter()
        .take(6)
        .map(|&(col, _)| my_ticket[col])
        .product();

    Ok(result)
}

pub fn solve_day16b_my() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day16a.txt")?;
    let mut chunks = file.split("\n\n");

    let ranges: Vec<Vec<RangeInclusive<usize>>> = chunks
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(": ").unwrap();
            rest.split(" or")
                .map(|part| {
                    let (start, end) = part.trim().split_once('-').unwrap();
                    start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
                })
                .collect()
        })
        .collect();

    let my_ticket: Vec<_> = chunks
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let tickets = chunks
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(|line| {
            let nums_in_ticket = line
                .split(',')
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>();
            let is_valid = nums_in_ticket
                .iter()
                .all(|n| ranges.iter().flatten().any(|r| r.contains(n)));
            if is_valid { Some(nums_in_ticket) } else { None }
        })
        .collect::<Vec<Vec<_>>>();

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..tickets[0].len() {
        let column_values: Vec<_> = tickets.iter().map(|v| v[i]).collect();

        for (rule_index, rule_ranges) in ranges.iter().enumerate() {
            let all_match = column_values
                .iter()
                .all(|v| rule_ranges.iter().any(|r| r.contains(v)));

            if all_match {
                map.entry(i).or_insert_with(Vec::new).push(rule_index);
            }
        }
    }

    let mut sorted_fields: Vec<_> = map.into_iter().collect();
    sorted_fields.sort_by_key(|(_, v)| v.len());

    let mut result = Vec::new();
    while !sorted_fields.is_empty() {
        let first_value = sorted_fields[0].1[0];
        let first_len = sorted_fields[0].1.len();

        if first_len == 1 {
            result.push((sorted_fields[0].0, first_value));
            sorted_fields.remove(0);
            sorted_fields = sorted_fields
                .into_iter()
                .map(|(u, v)| {
                    let new_v: Vec<usize> = v.into_iter().filter(|&n| n != first_value).collect();
                    (u, new_v)
                })
                .collect();
        } else {
            println!("first ele with size: {}", first_len);
            break;
        }
    }
    result.sort_by_key(|&(_, range_index)| range_index);
    let mut sum = 1;
    for i in 0..=5 {
        sum *= my_ticket[result[i].0];
    }

    Ok(sum)
}
