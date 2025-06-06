use aoc_2020::day20a::*;

pub fn main() {
    match solve_day20a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
