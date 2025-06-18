use aoc_2020::day24a::*;

pub fn main() {
    match solve_day24a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
