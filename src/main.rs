use aoc_2020::day25a::*;

pub fn main() {
    match solve_day25a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
