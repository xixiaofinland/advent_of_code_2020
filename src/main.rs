use aoc_2020::day22b::*;

pub fn main() {
    match solve_day22b() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
