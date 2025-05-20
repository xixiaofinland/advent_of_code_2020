use crate::AoCResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Schedule {
    num: usize,
    time_offset: usize,
}

fn mod_inverse(a: usize, m: usize) -> usize {
    let (mut t, mut new_t) = (0isize, 1isize);
    let (mut r, mut new_r) = (m as isize, a as isize);

    while new_r != 0 {
        let quotient = r / new_r;
        t = t - quotient * new_t;
        std::mem::swap(&mut t, &mut new_t);
        r = r - quotient * new_r;
        std::mem::swap(&mut r, &mut new_r);
    }

    if r > 1 {
        panic!("modular inverse does not exist");
    }

    if t < 0 {
        t += m as isize;
    }

    t as usize
}

fn chinese_remainder(schedules: &[Schedule]) -> usize {
    let prod: usize = schedules.iter().map(|s| s.num).product();
    let mut sum = 0usize;

    for s in schedules {
        let ni = s.num;
        let ai = (ni - (s.time_offset % ni)) % ni;
        let p = prod / ni;
        let inv = mod_inverse(p, ni);
        sum += ai * inv * p;
    }

    sum % prod
}

pub fn solve_day13b() -> AoCResult<usize> {
    let file = File::open("data/input_day13a.txt")?;
    let reader = BufReader::new(file);

    let mut line_iter = reader.lines().flatten();
    let _earliest: usize = line_iter.next().unwrap().parse().unwrap();
    let line2 = line_iter.next().unwrap();

    let schedules: Vec<Schedule> = line2
        .split(',')
        .enumerate()
        .filter_map(|(i, id)| {
            if id == "x" {
                None
            } else {
                Some(Schedule {
                    num: id.parse().unwrap(),
                    time_offset: i,
                })
            }
        })
        .collect();

    let result = chinese_remainder(&schedules);
    Ok(result)
}

