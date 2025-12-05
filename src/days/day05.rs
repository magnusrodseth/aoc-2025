/// Day 5: Cafeteria
///
/// Determine which available ingredient IDs are fresh by checking if they
/// fall within any of the fresh ingredient ID ranges.

use std::fs;

/// Parse a range line like "3-5" into (start, end)
fn parse_range(line: &str) -> (i64, i64) {
    let parts: Vec<&str> = line.trim().split('-').collect();
    let start: i64 = parts[0].parse().unwrap();
    let end: i64 = parts[1].parse().unwrap();
    (start, end)
}

/// Parse the input into (ranges, ingredient_ids)
fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<(i64, i64)> = parts[0]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_range(line))
        .collect();

    let ingredient_ids: Vec<i64> = parts[1]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse().unwrap())
        .collect();

    (ranges, ingredient_ids)
}

/// Check if an ingredient ID is fresh (falls within any range)
fn is_fresh(id: i64, ranges: &[(i64, i64)]) -> bool {
    ranges.iter().any(|(start, end)| id >= *start && id <= *end)
}

/// Part 1: Count how many available ingredient IDs are fresh
pub fn part1(input: &str) -> i64 {
    let (ranges, ingredient_ids) = parse_input(input);
    ingredient_ids.iter().filter(|&id| is_fresh(*id, &ranges)).count() as i64
}

/// Merge overlapping ranges and return total count of unique IDs
fn merge_ranges(ranges: &[(i64, i64)]) -> i64 {
    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start
    let mut sorted: Vec<(i64, i64)> = ranges.to_vec();
    sorted.sort_by_key(|r| r.0);

    // Merge overlapping ranges
    let mut merged: Vec<(i64, i64)> = Vec::new();
    let mut current = sorted[0];

    for &(start, end) in &sorted[1..] {
        if start <= current.1 + 1 {
            // Overlapping or adjacent, extend current range
            current.1 = current.1.max(end);
        } else {
            // No overlap, save current and start new
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    // Count total IDs in merged ranges
    merged.iter().map(|(start, end)| end - start + 1).sum()
}

/// Part 2: Count total unique fresh ingredient IDs from all ranges
pub fn part2(input: &str) -> i64 {
    let (ranges, _) = parse_input(input);
    merge_ranges(&ranges)
}

/// Entry point for running Day 5 solutions
pub fn run() {
    let input = fs::read_to_string("puzzles/day05/input.txt")
        .expect("Failed to read input file");

    println!("Day 5: Cafeteria");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("3-5"), (3, 5));
        assert_eq!(parse_range("10-14"), (10, 14));
        assert_eq!(parse_range("16-20"), (16, 20));
    }

    #[test]
    fn test_parse_input() {
        let (ranges, ids) = parse_input(EXAMPLE_INPUT);
        assert_eq!(ranges.len(), 4);
        assert_eq!(ranges[0], (3, 5));
        assert_eq!(ranges[1], (10, 14));
        assert_eq!(ranges[2], (16, 20));
        assert_eq!(ranges[3], (12, 18));
        assert_eq!(ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_is_fresh() {
        let ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];

        // ID 1 is spoiled (not in any range)
        assert!(!is_fresh(1, &ranges));

        // ID 5 is fresh (in range 3-5)
        assert!(is_fresh(5, &ranges));

        // ID 8 is spoiled
        assert!(!is_fresh(8, &ranges));

        // ID 11 is fresh (in range 10-14)
        assert!(is_fresh(11, &ranges));

        // ID 17 is fresh (in range 16-20 and 12-18)
        assert!(is_fresh(17, &ranges));

        // ID 32 is spoiled
        assert!(!is_fresh(32, &ranges));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_merge_ranges() {
        // Ranges: 3-5, 10-14, 16-20, 12-18
        // After merge: 3-5, 10-20
        // Count: 3 + 11 = 14
        let ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        assert_eq!(merge_ranges(&ranges), 14);
    }

    #[test]
    fn test_merge_adjacent_ranges() {
        // Ranges: 1-3, 4-6 -> merged: 1-6 = 6 IDs
        let ranges = vec![(1, 3), (4, 6)];
        assert_eq!(merge_ranges(&ranges), 6);
    }

    #[test]
    fn test_merge_single_range() {
        let ranges = vec![(5, 10)];
        assert_eq!(merge_ranges(&ranges), 6);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 14);
    }
}
