/// Day 1: Calorie Counting
///
/// This solution demonstrates the TDD approach used throughout the repository.
/// Tests are written first based on examples from the puzzle description,
/// then the implementation is built to pass those tests.

use std::fs;

/// Parse the input into groups of calorie counts per elf
///
/// Input format: Groups of numbers separated by blank lines
/// Each group represents one elf's inventory
fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .filter_map(|line| line.trim().parse::<i32>().ok())
                .collect()
        })
        .collect()
}

/// Calculate total calories for each elf
fn calculate_totals(elf_inventories: &[Vec<i32>]) -> Vec<i32> {
    elf_inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .collect()
}

/// Part 1: Find the maximum calories carried by a single elf
pub fn part1(input: &str) -> i32 {
    let inventories = parse_input(input);
    let totals = calculate_totals(&inventories);

    *totals.iter().max().unwrap_or(&0)
}

/// Part 2: Find the sum of top 3 elves' calories
pub fn part2(input: &str) -> i32 {
    let inventories = parse_input(input);
    let mut totals = calculate_totals(&inventories);

    // Sort in descending order and take top 3
    totals.sort_by(|a, b| b.cmp(a));
    totals.iter().take(3).sum()
}

/// Entry point for running Day 1 solutions
pub fn run() {
    let input = fs::read_to_string("puzzles/day01/input.txt")
        .expect("Failed to read input file");

    println!("Day 1: Calorie Counting");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example input from the puzzle description
    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_parse_input() {
        let result = parse_input(EXAMPLE_INPUT);

        assert_eq!(result.len(), 5, "Should have 5 elves");
        assert_eq!(result[0], vec![1000, 2000, 3000], "First elf inventory");
        assert_eq!(result[1], vec![4000], "Second elf inventory");
        assert_eq!(result[2], vec![5000, 6000], "Third elf inventory");
        assert_eq!(result[3], vec![7000, 8000, 9000], "Fourth elf inventory");
        assert_eq!(result[4], vec![10000], "Fifth elf inventory");
    }

    #[test]
    fn test_calculate_totals() {
        let inventories = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        let totals = calculate_totals(&inventories);

        assert_eq!(totals[0], 6000, "First elf total");
        assert_eq!(totals[1], 4000, "Second elf total");
        assert_eq!(totals[2], 11000, "Third elf total");
        assert_eq!(totals[3], 24000, "Fourth elf total");
        assert_eq!(totals[4], 10000, "Fifth elf total");
    }

    #[test]
    fn test_part1_example() {
        // From puzzle: "the elf carrying the most calories has 24000 calories"
        let result = part1(EXAMPLE_INPUT);
        assert_eq!(result, 24000, "Part 1: Maximum calories should be 24000");
    }

    #[test]
    fn test_part1_single_elf() {
        // Edge case: Only one elf
        let input = "1000\n2000";
        assert_eq!(part1(input), 3000);
    }

    #[test]
    fn test_part1_empty_input() {
        // Edge case: Empty input
        assert_eq!(part1(""), 0);
    }

    #[test]
    fn test_part2_example() {
        // From puzzle: "top three elves are carrying 24000, 11000, and 10000
        // calories respectively, for a total of 45000 calories"
        let result = part2(EXAMPLE_INPUT);
        assert_eq!(result, 45000, "Part 2: Top 3 total should be 45000");
    }

    #[test]
    fn test_part2_fewer_than_three_elves() {
        // Edge case: Only 2 elves
        let input = "1000\n2000\n\n3000";
        let result = part2(input);
        assert_eq!(result, 6000, "Should sum only available elves");
    }

    #[test]
    fn test_part2_exactly_three_elves() {
        // Edge case: Exactly 3 elves
        let input = "100\n\n200\n\n300";
        let result = part2(input);
        assert_eq!(result, 600, "Should sum all three elves");
    }

    #[test]
    fn test_part2_same_values() {
        // Edge case: Multiple elves with same calorie counts
        let input = "1000\n\n1000\n\n1000\n\n1000";
        let result = part2(input);
        assert_eq!(result, 3000, "Should handle duplicate values");
    }
}
