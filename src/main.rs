use aoc_2020::day1::solve_day1a;

pub fn main() {
    match solve_day1a() {
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
