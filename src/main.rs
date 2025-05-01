use aoc_2020::day1::*;

pub fn main() {
    // match solve_day1a() {
    match solve_day1b() {
        Ok(Some(result)) => {
            println!("Result: {}", result);
        }
        Ok(None) => {
            eprintln!("No reuslt found!");
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
