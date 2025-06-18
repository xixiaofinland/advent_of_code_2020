use aoc_2020::day24b::*;

pub fn main() {
    match solve_day24b() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
