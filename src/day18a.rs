use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::AoCResult;

pub fn solve_day18a() -> AoCResult<usize> {
    let file = File::open("data/input_day18a.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"\d+|[()+*]")?;

    let sum = reader
        .lines()
        .map(|line| -> AoCResult<_> {
            let line = line?;
            let tokens: Vec<String> = re
                .find_iter(&line)
                .map(|m| m.as_str().to_string())
                .collect();
            Ok(parse_expr(&mut tokens.into_iter()))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();

    Ok(sum)
}

fn parse_expr<I>(tokens: &mut I) -> usize
where
    I: Iterator<Item = String>,
{
    let mut acc = parse_operand(tokens);

    while let Some(op) = tokens.next() {
        if op == ")" {
            break;
        }

        let rhs = parse_operand(tokens);

        acc = match op.as_str() {
            "+" => acc + rhs,
            "*" => acc * rhs,
            _ => panic!("Unknown operator: {}", op),
        };
    }

    acc
}

fn parse_operand<I>(tokens: &mut I) -> usize
where
    I: Iterator<Item = String>,
{
    match tokens.next().as_deref() {
        Some("(") => parse_expr(tokens),
        Some(num) if num.chars().all(char::is_numeric) => num.parse().unwrap(),
        Some(")") => panic!("Unexpected `)` when expecting operand"),
        other => panic!("Unexpected operand: {:?}", other),
    }
}

fn split_token(element: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut remaining = element;

    while let Some(rest) = remaining.strip_prefix('(') {
        tokens.push("(");
        remaining = rest;
    }

    let mut suffixes = Vec::new();
    while let Some(rest) = remaining.strip_suffix(')') {
        suffixes.push(")");
        remaining = rest;
    }

    if !remaining.is_empty() {
        tokens.push(remaining);
    }

    // Add any trailing ')' we found (in order)
    tokens.extend(suffixes.into_iter().rev());

    tokens
}

