use aoc_2020::day1::*;
use aoc_2020::day2::*;
use aoc_2020::day3::solve_day3a;

pub fn main() {
    match solve_day3a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
    // match solve_day2b() {
    //     Ok(count) => {
    //         println!("Result: {}", count);
    //     }
    //     Err(e) => {
    //         eprintln!("Err: {}", e);
    //     }
    // }

    // match solve_day2a() {
    //     Ok(count) => {
    //         println!("Result: {}", count);
    //     }
    //     Err(e) => {
    //         eprintln!("Err: {}", e);
    //     }
    // }

    // match solve_day1b() {
    //     Ok(Some(result)) => {
    //         println!("Result: {}", result);
    //     }
    //     Ok(None) => {
    //         eprintln!("No reuslt found!");
    //     }
    //     Err(e) => {
    //         eprintln!("Err: {}", e);
    //     }
    // }

    // match solve_day1a() {
    //     Ok(Some(result)) => {
    //         println!("Result: {}", result);
    //     }
    //     Ok(None) => {
    //         eprintln!("No reuslt found!");
    //     }
    //     Err(e) => {
    //         eprintln!("Err: {}", e);
    //     }
}
