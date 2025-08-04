use aoc_2020::days::*;

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
