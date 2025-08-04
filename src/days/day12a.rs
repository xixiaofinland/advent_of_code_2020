use crate::AoCResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Action {
    N(i32),
    S(i32),
    W(i32),
    E(i32),
    F(i32),
    L(i32),
    R(i32),
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        let (action, val) = s.split_at(1);
        let num = val.parse().expect(&format!("Invalid number: {}", val));
        match action {
            "N" => Action::N(num),
            "S" => Action::S(num),
            "W" => Action::W(num),
            "E" => Action::E(num),
            "F" => Action::F(num),
            "L" => Action::L(num),
            "R" => Action::R(num),
            _ => panic!("Invalid action: {}", action),
        }
    }
}

#[derive(Debug)]
struct State {
    dir: i32, // 0=N, 1=E, 2=S, 3=W
    x: i32,
    y: i32,
}

impl State {
    fn apply(&mut self, action: Action) {
        match action {
            Action::N(n) => self.y -= n,
            Action::S(n) => self.y += n,
            Action::E(n) => self.x += n,
            Action::W(n) => self.x -= n,
            Action::L(n) => self.dir = (self.dir + 4 - n / 90) % 4,
            Action::R(n) => self.dir = (self.dir + n / 90) % 4,
            Action::F(n) => match self.dir {
                0 => self.y -= n,
                1 => self.x += n,
                2 => self.y += n,
                3 => self.x -= n,
                _ => unreachable!(),
            },
        }
    }
}

pub fn solve_day12a() -> AoCResult<usize> {
    let file = File::open("data/input_day12a.txt")?;
    let reader = BufReader::new(file);

    let actions = reader
        .lines()
        .map(|line| -> AoCResult<_> { Ok(Action::from(line?.as_str())) })
        .collect::<Result<Vec<_>, _>>()?;

    let mut state = State { dir: 1, x: 0, y: 0 };

    for action in actions {
        state.apply(action);
    }

    Ok((state.x.abs() + state.y.abs()).try_into().unwrap())
}
