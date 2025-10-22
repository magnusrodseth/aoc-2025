/// Advent of Code 2025 - Main Entry Point
///
/// This is a simple runner for executing individual day solutions.
/// In the automated workflow, this will be called by the orchestration scripts.

use aoc_2025::days;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Advent of Code 2025");
        println!("Usage: cargo run -- <day>");
        println!("Example: cargo run -- 1");
        println!("\nAvailable days:");
        println!("  1: Calorie Counting");
        return;
    }

    let day: u8 = args[1]
        .parse()
        .expect("Day must be a number between 1 and 12");

    match day {
        1 => days::day01::run(),
        _ => println!("Day {} not yet implemented", day),
    }
}
