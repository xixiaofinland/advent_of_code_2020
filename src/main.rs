use aoc_2020::day22::*;

pub fn main() {
    match solve_day22a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
