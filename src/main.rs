use aoc_2020::day19b::*;

pub fn main() {
    match solve_day19b() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
