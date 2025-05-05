use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::AoCResult;

pub fn solve_day4a() -> AoCResult<usize> {
    let reader = BufReader::new(File::open("data/input_day4a.txt")?);
    let mut passports = Vec::new();
    let mut current_passport = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            if !current_passport.is_empty() {
                passports.push(current_passport);
                current_passport = String::new();
            }
        } else {
            let data_to_push = if current_passport.is_empty() {
                &line
            } else {
                &format!(" {}", line)
            };

            current_passport.push_str(data_to_push);
        }
    }
    if !current_passport.is_empty() {
        passports.push(current_passport);
    }

    let passport_maps: Vec<HashMap<&str, &str>> = passports
        .iter()
        .map(|p| {
            p.split_whitespace()
                .filter_map(|item| item.split_once(':'))
                .collect()
        })
        .collect();

    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let count = passport_maps
        .iter()
        .filter(|p| required.iter().all(|key| p.contains_key(key)))
        .count();

    Ok(count)
}

pub fn solve_day4b() -> AoCResult<usize> {
    let reader = BufReader::new(File::open("data/input_day4a.txt")?);
    let mut passports = Vec::new();
    let mut current_passport = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            if !current_passport.is_empty() {
                passports.push(current_passport);
                current_passport = String::new();
            }
        } else {
            let data_to_push = if current_passport.is_empty() {
                &line
            } else {
                &format!(" {}", line)
            };

            current_passport.push_str(data_to_push);
        }
    }
    if !current_passport.is_empty() {
        passports.push(current_passport);
    }

    let passport_maps: Vec<HashMap<&str, &str>> = passports
        .iter()
        .map(|p| {
            p.split_whitespace()
                .filter_map(|item| item.split_once(':'))
                .collect()
        })
        .collect();

    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let count = passport_maps
        .iter()
        .filter(|&p| required.iter().all(|key| p.contains_key(key)))
        .filter(|&p| is_valid_year(p.get("byr").unwrap(), 1920, 2002))
        .filter(|&p| is_valid_year(p.get("iyr").unwrap(), 2010, 2020))
        .filter(|&p| is_valid_year(p.get("eyr").unwrap(), 2020, 2030))
        .filter(|&p| is_valid_height(p.get("hgt").unwrap()))
        .filter(|&p| is_valid_hcl(p.get("hcl").unwrap()))
        .filter(|&p| {
            let data = p.get("ecl").unwrap();
            matches!(*data, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
        })
        .filter(|&p| {
            let data = p.get("pid").unwrap();
            data.len() == 9 && data.chars().all(|d| d.is_ascii_digit())
        })
        .count();

    Ok(count)
}

fn is_valid_year(s: &str, min: usize, max: usize) -> bool {
    s.len() == 4
        && s.chars().all(|c| c.is_ascii_digit())
        && matches!(s.parse::<usize>(), Ok(n) if n>=min && n <=max)
}

fn is_valid_height(s: &str) -> bool {
    if let Some(num) = s.strip_suffix("cm") {
        matches!(num.parse::<usize>(), Ok(n) if (150..=193).contains(&n))
    } else if let Some(num) = s.strip_suffix("in") {
        matches!(num.parse::<usize>(), Ok(n) if (59..=76).contains(&n))
    } else {
        false
    }
}

fn is_valid_hcl(s: &str) -> bool {
    s.starts_with('#')
        && s.len() == 7
        && s[1..]
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}
