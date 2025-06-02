use aoc_2020::day19a::*;

pub fn main() {
    match solve_day19a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
