use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Peekable,
};

use crate::AoCResult;

pub fn solve_day18b() -> AoCResult<usize> {
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
            let mut iter = tokens.into_iter().peekable();
            Ok(parse_expr(&mut iter))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();

    Ok(sum)
}

// expr = term ('*' term)*
fn parse_expr<I>(tokens: &mut Peekable<I>) -> usize
where
    I: Iterator<Item = String>,
{
    let mut acc = parse_term(tokens);

    while let Some(op) = tokens.peek() {
        if op == "*" {
            tokens.next(); // consume '*'
            let rhs = parse_term(tokens);
            acc *= rhs;
        } else {
            break;
        }
    }

    acc
}

// term = factor ('+' factor)*
fn parse_term<I>(tokens: &mut Peekable<I>) -> usize
where
    I: Iterator<Item = String>,
{
    let mut acc = parse_factor(tokens);

    while let Some(op) = tokens.peek() {
        if op == "+" {
            tokens.next(); // consume '+'
            let rhs = parse_factor(tokens);
            acc += rhs;
        } else {
            break;
        }
    }

    acc
}

// factor = number | '(' expr ')'
fn parse_factor<I>(tokens: &mut Peekable<I>) -> usize
where
    I: Iterator<Item = String>,
{
    match tokens.next().as_deref() {
        Some("(") => {
            let value = parse_expr(tokens);
            assert_eq!(tokens.next().as_deref(), Some(")"));
            value
        }
        Some(num) if num.chars().all(char::is_numeric) => num.parse().unwrap(),
        other => panic!("Unexpected factor: {:?}", other),
    }
}
