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
struct WaypointState {
    x: i32, // "+" east, "-" west
    y: i32, // "+" north, "-" south
}

#[derive(Debug)]
struct ShipState {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct State {
    waypoint: WaypointState,
    ship: ShipState,
}

impl State {
    fn new() -> Self {
        Self {
            waypoint: WaypointState { x: 10, y: 1 },
            ship: ShipState { x: 0, y: 0 },
        }
    }

    fn apply(&mut self, action: Action) {
        match action {
            Action::N(n) => self.waypoint.y += n,
            Action::S(n) => self.waypoint.y -= n,

            Action::E(n) => self.waypoint.x += n,
            Action::W(n) => self.waypoint.x -= n,

            Action::R(n) => {
                let (x, y) = (self.waypoint.x, self.waypoint.y);
                match n % 360 {
                    90 => {
                        self.waypoint.x = y;
                        self.waypoint.y = -x;
                    }
                    180 => {
                        self.waypoint.x = -x;
                        self.waypoint.y = -y;
                    }
                    270 => {
                        self.waypoint.x = -y;
                        self.waypoint.y = x;
                    }
                    _ => panic!("Invalid R rotation: {}", n),
                }
            }
            Action::L(n) => {
                let (x, y) = (self.waypoint.x, self.waypoint.y);
                match n % 360 {
                    90 => {
                        self.waypoint.x = -y;
                        self.waypoint.y = x;
                    }
                    180 => {
                        self.waypoint.x = -x;
                        self.waypoint.y = -y;
                    }
                    270 => {
                        self.waypoint.x = y;
                        self.waypoint.y = -x;
                    }
                    _ => panic!("Invalid L rotation: {}", n),
                }
            }

            Action::F(n) => {
                self.ship.x += self.waypoint.x * n;
                self.ship.y += self.waypoint.y * n;
            }
        }
    }
}

pub fn solve_day12b() -> AoCResult<usize> {
    let file = File::open("data/input_day12a.txt")?;
    let reader = BufReader::new(file);

    let actions = reader
        .lines()
        .map(|line| -> AoCResult<_> { Ok(Action::from(line?.as_str())) })
        .collect::<Result<Vec<_>, _>>()?;

    let mut state = State::new();

    for action in actions {
        state.apply(action);
    }
    eprintln!("gopro[372]: day12b.rs:126: state={:#?}", state);

    Ok((state.ship.x.abs() + state.ship.y.abs())
        .try_into()
        .unwrap())
}
