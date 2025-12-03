/// Day 3: Lobby
///
/// Find the maximum joltage possible from each battery bank by turning on exactly two batteries.
/// The joltage is the number formed by the two selected digits.
/// Sum the maximum joltages from all banks.

use std::fs;

/// Parse input into lines representing battery banks
fn parse_input(input: &str) -> Vec<&str> {
    input.lines().filter(|line| !line.is_empty()).collect()
}

/// Find the maximum joltage for a single battery bank
/// by selecting exactly two digits to form the largest possible number
fn max_joltage_for_bank(bank: &str) -> i64 {
    let digits: Vec<char> = bank.chars().collect();
    let mut max = 0i64;

    // Try all pairs of positions
    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            // Form a two-digit number from positions i and j
            let first = digits[i].to_digit(10).unwrap() as i64;
            let second = digits[j].to_digit(10).unwrap() as i64;
            let value = first * 10 + second;
            max = max.max(value);
        }
    }

    max
}

/// Part 1 solution
pub fn part1(input: &str) -> i64 {
    let banks = parse_input(input);
    banks.iter().map(|bank| max_joltage_for_bank(bank)).sum()
}

/// Find the maximum joltage for a bank by selecting exactly n batteries
/// Strategy: Use a greedy approach to select the n largest digits while maintaining order
fn max_joltage_for_bank_n(bank: &str, n: usize) -> i64 {
    let digits: Vec<char> = bank.chars().collect();
    let len = digits.len();

    if n >= len {
        // If we need all or more digits than available, use all
        return bank.parse::<i64>().unwrap_or(0);
    }

    // Greedy approach: for each position in the result, pick the largest digit
    // that still leaves enough digits for the remaining positions
    let mut result = String::new();
    let mut start_idx = 0;

    for position in 0..n {
        let remaining_after_this = n - position - 1;
        let search_end = len - remaining_after_this;

        // Find the largest digit in the valid range
        let mut max_digit = '0';
        let mut max_idx = start_idx;

        for i in start_idx..search_end {
            if digits[i] > max_digit {
                max_digit = digits[i];
                max_idx = i;
            }
        }

        result.push(max_digit);
        start_idx = max_idx + 1;
    }

    result.parse::<i64>().unwrap_or(0)
}

/// Part 2 solution
pub fn part2(input: &str) -> i64 {
    let banks = parse_input(input);
    banks.iter().map(|bank| max_joltage_for_bank_n(bank, 12)).sum()
}

/// Entry point for running this day
pub fn run() {
    let input = fs::read_to_string("puzzles/day03/input.txt")
        .expect("Failed to read input file");

    println!("Day 3: Lobby");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE_INPUT);
        assert_eq!(result, 357, "Example should return 357 (98+89+78+92)");
    }

    #[test]
    fn test_max_joltage_bank1() {
        assert_eq!(max_joltage_for_bank("987654321111111"), 98);
    }

    #[test]
    fn test_max_joltage_bank2() {
        assert_eq!(max_joltage_for_bank("811111111111119"), 89);
    }

    #[test]
    fn test_max_joltage_bank3() {
        assert_eq!(max_joltage_for_bank("234234234234278"), 78);
    }

    #[test]
    fn test_max_joltage_bank4() {
        assert_eq!(max_joltage_for_bank("818181911112111"), 92);
    }

    #[test]
    fn test_simple_bank() {
        assert_eq!(max_joltage_for_bank("12345"), 45);
    }

    #[test]
    fn test_two_digit_bank() {
        assert_eq!(max_joltage_for_bank("24"), 24);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE_INPUT);
        assert_eq!(result, 3121910778619, "Part 2 example should return 3121910778619");
    }

    #[test]
    fn test_max_joltage_12_bank1() {
        assert_eq!(max_joltage_for_bank_n("987654321111111", 12), 987654321111);
    }

    #[test]
    fn test_max_joltage_12_bank2() {
        assert_eq!(max_joltage_for_bank_n("811111111111119", 12), 811111111119);
    }

    #[test]
    fn test_max_joltage_12_bank3() {
        assert_eq!(max_joltage_for_bank_n("234234234234278", 12), 434234234278);
    }

    #[test]
    fn test_max_joltage_12_bank4() {
        assert_eq!(max_joltage_for_bank_n("818181911112111", 12), 888911112111);
    }
}
