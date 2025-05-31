use regex::Regex;

use crate::AoCResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_day18a() -> AoCResult<usize> {
    let file = File::open("data/input_day18a_simple.txt")?;
    let reader = BufReader::new(file);

    let tokens: Vec<Vec<_>> = reader
        .lines()
        .map(|line| -> AoCResult<_> {
            line?
                .split_whitespace()
                .try_fold(Vec::new(), |mut acc, element| -> AoCResult<_> {
                    acc.extend(split_token_regex(element).into_iter().map(String::from));
                    Ok(acc)
                })
        })
        .collect::<Result<_, _>>()?;

    eprintln!("gopro[408]: day18a.rs:10: tokens={:#?}", tokens);
    Ok(0)
}

fn split_token_regex(element: &str) -> Vec<&str> {
    let re = Regex::new(r"\(|\)|\w+").unwrap(); // matches '(', ')' or any word/number
    re.find_iter(element).map(|m| m.as_str()).collect()
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
