use aoc_2020::day23::*;

pub fn main() {
    match solve_day23a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
