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
    sorted_fields.sort_by_key(|&(_, ref v)| v.len());

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
