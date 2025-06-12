use aoc_2020::day21::*;

pub fn main() {
    match solve_day21() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
