/// Day 2: Gift Shop
///
/// Find invalid product IDs in given ranges. An invalid ID is made only of
/// some sequence of digits repeated twice (e.g., 55, 6464, 123123).
/// No leading zeroes allowed.

use std::fs;

/// Check if a number is invalid (made of a pattern repeated exactly twice)
/// Examples: 11 (1 repeated), 6464 (64 repeated), 123123 (123 repeated)
/// Not invalid: 101 (has leading zero when split), 111 (odd length)
fn is_invalid_id(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Must have even length to be splittable into two equal parts
    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let first_half = &s[..half];
    let second_half = &s[half..];

    // Check if both halves are equal
    if first_half != second_half {
        return false;
    }

    // Check for leading zeros - first half shouldn't start with 0
    // (unless it's just "0", but then the number would be 00 which is 0, not a valid pattern)
    if first_half.starts_with('0') {
        return false;
    }

    true
}

/// Check if a number is invalid for Part 2 (made of a pattern repeated at least twice)
/// Examples: 11 (1x2), 111 (1x3), 6464 (64x2), 123123 (123x2), 123123123 (123x3)
fn is_invalid_id_v2(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Need at least 2 digits to have a repeated pattern
    if len < 2 {
        return false;
    }

    // Try each possible pattern length from 1 to len/2
    for pattern_len in 1..=len / 2 {
        // Length must be divisible by pattern length
        if len % pattern_len != 0 {
            continue;
        }

        let repetitions = len / pattern_len;
        if repetitions < 2 {
            continue;
        }

        let pattern = &s[..pattern_len];

        // Pattern cannot have leading zeros
        if pattern.starts_with('0') {
            continue;
        }

        // Check if the entire string is this pattern repeated
        let mut is_repeated = true;
        for i in 1..repetitions {
            let start = i * pattern_len;
            let end = start + pattern_len;
            if &s[start..end] != pattern {
                is_repeated = false;
                break;
            }
        }

        if is_repeated {
            return true;
        }
    }

    false
}

/// Parse the input and extract ranges
/// Input format: "11-22,95-115,998-1012,..."
fn parse_ranges(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .split(',')
        .filter_map(|range| {
            let parts: Vec<&str> = range.trim().split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<i64>().ok()?;
                let end = parts[1].parse::<i64>().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect()
}

/// Find all invalid IDs in the given ranges and sum them (Part 1: exactly twice)
fn sum_invalid_ids(ranges: &[(i64, i64)]) -> i64 {
    let mut sum = 0;
    for &(start, end) in ranges {
        for id in start..=end {
            if is_invalid_id(id) {
                sum += id;
            }
        }
    }
    sum
}

/// Find all invalid IDs in the given ranges and sum them (Part 2: at least twice)
fn sum_invalid_ids_v2(ranges: &[(i64, i64)]) -> i64 {
    let mut sum = 0;
    for &(start, end) in ranges {
        for id in start..=end {
            if is_invalid_id_v2(id) {
                sum += id;
            }
        }
    }
    sum
}

/// Part 1: Find and sum all invalid product IDs
pub fn part1(input: &str) -> i64 {
    let ranges = parse_ranges(input);
    sum_invalid_ids(&ranges)
}

/// Part 2: Find and sum all invalid product IDs (pattern repeated at least twice)
pub fn part2(input: &str) -> i64 {
    let ranges = parse_ranges(input);
    sum_invalid_ids_v2(&ranges)
}

/// Entry point for running Day 2 solutions
pub fn run() {
    let input = fs::read_to_string("puzzles/day02/input.txt")
        .expect("Failed to read input file");

    println!("Day 2: Gift Shop");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_is_invalid_id_basic() {
        assert!(is_invalid_id(11), "11 should be invalid (1 repeated)");
        assert!(is_invalid_id(22), "22 should be invalid (2 repeated)");
        assert!(is_invalid_id(99), "99 should be invalid (9 repeated)");
    }

    #[test]
    fn test_is_invalid_id_four_digits() {
        assert!(is_invalid_id(6464), "6464 should be invalid (64 repeated)");
        assert!(is_invalid_id(1010), "1010 should be invalid (10 repeated)");
    }

    #[test]
    fn test_is_invalid_id_six_digits() {
        assert!(is_invalid_id(123123), "123123 should be invalid (123 repeated)");
    }

    #[test]
    fn test_is_invalid_id_not_repeated() {
        assert!(!is_invalid_id(12), "12 should be valid (not repeated)");
        assert!(!is_invalid_id(101), "101 should be valid (odd length)");
        assert!(!is_invalid_id(1234), "1234 should be valid (not repeated)");
    }

    #[test]
    fn test_is_invalid_id_leading_zeros() {
        // 0101 would be invalid pattern, but it's represented as 101 (no leading zero)
        assert!(!is_invalid_id(101), "101 should be valid (01 has leading zero)");
    }

    #[test]
    fn test_parse_ranges() {
        let input = "11-22,95-115";
        let ranges = parse_ranges(input);
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0], (11, 22));
        assert_eq!(ranges[1], (95, 115));
    }

    #[test]
    fn test_range_11_22() {
        // Should find 11 and 22
        let ranges = vec![(11, 22)];
        let mut invalids = vec![];
        for &(start, end) in &ranges {
            for id in start..=end {
                if is_invalid_id(id) {
                    invalids.push(id);
                }
            }
        }
        assert_eq!(invalids, vec![11, 22], "Range 11-22 should have invalid IDs: 11, 22");
    }

    #[test]
    fn test_range_95_115() {
        // Should find 99
        let ranges = vec![(95, 115)];
        let mut invalids = vec![];
        for &(start, end) in &ranges {
            for id in start..=end {
                if is_invalid_id(id) {
                    invalids.push(id);
                }
            }
        }
        assert_eq!(invalids, vec![99], "Range 95-115 should have invalid ID: 99");
    }

    #[test]
    fn test_range_998_1012() {
        // Should find 1010
        let ranges = vec![(998, 1012)];
        let mut invalids = vec![];
        for &(start, end) in &ranges {
            for id in start..=end {
                if is_invalid_id(id) {
                    invalids.push(id);
                }
            }
        }
        assert_eq!(invalids, vec![1010], "Range 998-1012 should have invalid ID: 1010");
    }

    #[test]
    fn test_range_222220_222224() {
        // Should find 222222
        let ranges = vec![(222220, 222224)];
        let mut invalids = vec![];
        for &(start, end) in &ranges {
            for id in start..=end {
                if is_invalid_id(id) {
                    invalids.push(id);
                }
            }
        }
        assert_eq!(invalids, vec![222222], "Range 222220-222224 should have invalid ID: 222222");
    }

    #[test]
    fn test_range_446443_446449() {
        // Should find 446446
        let ranges = vec![(446443, 446449)];
        let mut invalids = vec![];
        for &(start, end) in &ranges {
            for id in start..=end {
                if is_invalid_id(id) {
                    invalids.push(id);
                }
            }
        }
        assert_eq!(invalids, vec![446446], "Range 446443-446449 should have invalid ID: 446446");
    }

    #[test]
    fn test_part1_example() {
        // Expected: 1227775554
        let result = part1(EXAMPLE_INPUT);
        assert_eq!(result, 1227775554, "Part 1 example should sum to 1227775554");
    }

    // Part 2 tests

    #[test]
    fn test_is_invalid_id_v2_basic() {
        // Same as Part 1: pattern repeated twice
        assert!(is_invalid_id_v2(11), "11 should be invalid (1x2)");
        assert!(is_invalid_id_v2(22), "22 should be invalid (2x2)");
        assert!(is_invalid_id_v2(99), "99 should be invalid (9x2)");
    }

    #[test]
    fn test_is_invalid_id_v2_three_times() {
        // Pattern repeated three times
        assert!(is_invalid_id_v2(111), "111 should be invalid (1x3)");
        assert!(is_invalid_id_v2(999), "999 should be invalid (9x3)");
        assert!(is_invalid_id_v2(123123123), "123123123 should be invalid (123x3)");
    }

    #[test]
    fn test_is_invalid_id_v2_five_times() {
        // Pattern repeated five times
        assert!(is_invalid_id_v2(1212121212), "1212121212 should be invalid (12x5)");
        assert!(is_invalid_id_v2(2121212121), "2121212121 should be invalid (21x5)");
    }

    #[test]
    fn test_is_invalid_id_v2_seven_times() {
        // Pattern repeated seven times
        assert!(is_invalid_id_v2(1111111), "1111111 should be invalid (1x7)");
    }

    #[test]
    fn test_is_invalid_id_v2_specific_examples() {
        // From puzzle Part 2 examples
        assert!(is_invalid_id_v2(565656), "565656 should be invalid (56x3)");
        assert!(is_invalid_id_v2(824824824), "824824824 should be invalid (824x3)");
    }

    #[test]
    fn test_is_invalid_id_v2_not_repeated() {
        assert!(!is_invalid_id_v2(12), "12 should be valid");
        assert!(!is_invalid_id_v2(101), "101 should be valid");
        assert!(!is_invalid_id_v2(1234), "1234 should be valid");
        assert!(!is_invalid_id_v2(112), "112 should be valid (not uniform repetition)");
    }

    #[test]
    fn test_range_95_115_v2() {
        // Part 2: Should find 99 and 111
        let mut invalids = vec![];
        for id in 95..=115 {
            if is_invalid_id_v2(id) {
                invalids.push(id);
            }
        }
        assert_eq!(invalids, vec![99, 111], "Range 95-115 should have invalid IDs: 99, 111");
    }

    #[test]
    fn test_range_998_1012_v2() {
        // Part 2: Should find 999 and 1010
        let mut invalids = vec![];
        for id in 998..=1012 {
            if is_invalid_id_v2(id) {
                invalids.push(id);
            }
        }
        assert_eq!(invalids, vec![999, 1010], "Range 998-1012 should have invalid IDs: 999, 1010");
    }

    #[test]
    fn test_part2_example() {
        // Expected: 4174379265
        let result = part2(EXAMPLE_INPUT);
        assert_eq!(result, 4174379265, "Part 2 example should sum to 4174379265");
    }
}
