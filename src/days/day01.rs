/// Day 1: Secret Entrance
///
/// A safe dial goes from 0-99 in a circle. Starting at 50, follow rotation
/// instructions and count how many times the dial points at 0.

use std::fs;

/// Parse a single rotation instruction (e.g., "L68" or "R48")
/// Returns (direction, distance) where direction is -1 for L and 1 for R
fn parse_rotation(s: &str) -> (i64, i64) {
    let s = s.trim();
    if s.is_empty() {
        return (0, 0);
    }
    let direction = if s.starts_with('L') { -1 } else { 1 };
    let distance: i64 = s[1..].parse().unwrap_or(0);
    (direction, distance)
}

/// Apply a rotation to the current position
/// Returns the new position (0-99)
fn apply_rotation(position: i64, direction: i64, distance: i64) -> i64 {
    let new_pos = position + direction * distance;
    // Handle wrapping: mod 100, but handle negative numbers
    ((new_pos % 100) + 100) % 100
}

/// Count how many times the dial passes through 0 during a rotation
/// This includes every click that lands on 0 (during and at the end)
fn count_zeros_during_rotation(start: i64, direction: i64, distance: i64) -> i64 {
    if distance == 0 {
        return 0;
    }

    // For a rotation, we visit positions: start+dir, start+2*dir, ..., start+D*dir
    // We need to count how many of these (in raw form, before mod) are multiples of 100

    if direction == -1 {
        // Left rotation: range [start-distance, start-1]
        // Count multiples of 100 in this range
        let a = start - distance;
        let b = start - 1;
        // floor(b/100) - floor((a-1)/100) handles negative numbers correctly with integer division
        floor_div(b, 100) - floor_div(a - 1, 100)
    } else {
        // Right rotation: range [start+1, start+distance]
        let a = start + 1;
        let b = start + distance;
        floor_div(b, 100) - floor_div(a - 1, 100)
    }
}

/// Integer floor division that handles negative numbers correctly
fn floor_div(a: i64, b: i64) -> i64 {
    if a >= 0 {
        a / b
    } else {
        (a - b + 1) / b
    }
}

/// Part 1: Count how many times the dial points at 0 after any rotation
pub fn part1(input: &str) -> i64 {
    let mut position: i64 = 50; // Dial starts at 50
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (direction, distance) = parse_rotation(line);
        position = apply_rotation(position, direction, distance);
        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

/// Part 2: Count all times the dial passes through 0 during any rotation
pub fn part2(input: &str) -> i64 {
    let mut position: i64 = 50; // Dial starts at 50
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (direction, distance) = parse_rotation(line);
        zero_count += count_zeros_during_rotation(position, direction, distance);
        position = apply_rotation(position, direction, distance);
    }

    zero_count
}

/// Entry point for running Day 1 solutions
pub fn run() {
    let input = fs::read_to_string("puzzles/day01/input.txt")
        .expect("Failed to read input file");

    println!("Day 1: Secret Entrance");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example input from the puzzle description
    const EXAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_parse_rotation_left() {
        assert_eq!(parse_rotation("L68"), (-1, 68));
        assert_eq!(parse_rotation("L5"), (-1, 5));
    }

    #[test]
    fn test_parse_rotation_right() {
        assert_eq!(parse_rotation("R48"), (1, 48));
        assert_eq!(parse_rotation("R14"), (1, 14));
    }

    #[test]
    fn test_apply_rotation_simple() {
        // From 50, L68 should go to 82 (50 - 68 = -18, wraps to 82)
        assert_eq!(apply_rotation(50, -1, 68), 82);
        // From 82, L30 should go to 52
        assert_eq!(apply_rotation(82, -1, 30), 52);
        // From 52, R48 should go to 0 (52 + 48 = 100, wraps to 0)
        assert_eq!(apply_rotation(52, 1, 48), 0);
    }

    #[test]
    fn test_apply_rotation_wrap_left() {
        // From 5, L10 should go to 95
        assert_eq!(apply_rotation(5, -1, 10), 95);
        // From 0, L1 should go to 99
        assert_eq!(apply_rotation(0, -1, 1), 99);
    }

    #[test]
    fn test_apply_rotation_wrap_right() {
        // From 99, R1 should go to 0
        assert_eq!(apply_rotation(99, 1, 1), 0);
        // From 95, R10 should go to 5
        assert_eq!(apply_rotation(95, 1, 10), 5);
    }

    #[test]
    fn test_part1_example() {
        // From puzzle: dial points at 0 three times at end of rotations
        let result = part1(EXAMPLE_INPUT);
        assert_eq!(result, 3, "Part 1: Should count 3 times at position 0");
    }

    #[test]
    fn test_part1_trace_example() {
        // Trace through the example step by step
        let mut position: i64 = 50;
        let rotations = [
            ("L68", 82),  // 50 - 68 = -18 -> 82
            ("L30", 52),  // 82 - 30 = 52
            ("R48", 0),   // 52 + 48 = 100 -> 0
            ("L5", 95),   // 0 - 5 = -5 -> 95
            ("R60", 55),  // 95 + 60 = 155 -> 55
            ("L55", 0),   // 55 - 55 = 0
            ("L1", 99),   // 0 - 1 = -1 -> 99
            ("L99", 0),   // 99 - 99 = 0
            ("R14", 14),  // 0 + 14 = 14
            ("L82", 32),  // 14 - 82 = -68 -> 32
        ];

        for (instruction, expected) in rotations {
            let (direction, distance) = parse_rotation(instruction);
            position = apply_rotation(position, direction, distance);
            assert_eq!(position, expected, "After {} should be at {}", instruction, expected);
        }
    }

    #[test]
    fn test_part1_single_zero() {
        // R50 from 50 should land on 0
        let input = "R50";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_part1_no_zeros() {
        // L1 from 50 should land on 49
        let input = "L1";
        assert_eq!(part1(input), 0);
    }

    // Part 2 tests

    #[test]
    fn test_count_zeros_left_passes_zero() {
        // From 50, L68 passes through 0 once (at position 0)
        assert_eq!(count_zeros_during_rotation(50, -1, 68), 1);
    }

    #[test]
    fn test_count_zeros_left_no_pass() {
        // From 82, L30 doesn't pass through 0
        assert_eq!(count_zeros_during_rotation(82, -1, 30), 0);
    }

    #[test]
    fn test_count_zeros_right_lands_on_zero() {
        // From 52, R48 lands on 0 (passes through 100, which is 0)
        assert_eq!(count_zeros_during_rotation(52, 1, 48), 1);
    }

    #[test]
    fn test_count_zeros_right_passes_zero() {
        // From 95, R60 passes through 0 once (at 100)
        assert_eq!(count_zeros_during_rotation(95, 1, 60), 1);
    }

    #[test]
    fn test_count_zeros_left_lands_on_zero() {
        // From 55, L55 lands on 0
        assert_eq!(count_zeros_during_rotation(55, -1, 55), 1);
    }

    #[test]
    fn test_count_zeros_left_from_zero() {
        // From 0, L1 goes to 99, doesn't pass 0 (starts at 0, first click is 99)
        assert_eq!(count_zeros_during_rotation(0, -1, 1), 0);
    }

    #[test]
    fn test_count_zeros_left_wraps_to_zero() {
        // From 99, L99 lands on 0
        assert_eq!(count_zeros_during_rotation(99, -1, 99), 1);
    }

    #[test]
    fn test_count_zeros_right_1000() {
        // From 50, R1000 should pass through 0 ten times
        // Positions: 51, 52, ..., 100 (1st zero), ..., 200 (2nd), ..., 1000 (10th), then ends at 50
        assert_eq!(count_zeros_during_rotation(50, 1, 1000), 10);
    }

    #[test]
    fn test_count_zeros_left_during_82() {
        // From 14, L82 passes through 0 once
        assert_eq!(count_zeros_during_rotation(14, -1, 82), 1);
    }

    #[test]
    fn test_part2_example() {
        // From puzzle: 3 at end of rotation + 3 during rotations = 6
        let result = part2(EXAMPLE_INPUT);
        assert_eq!(result, 6, "Part 2: Should count 6 total times at position 0");
    }

    #[test]
    fn test_part2_single_large_rotation() {
        // R1000 from 50 should pass 0 ten times
        let input = "R1000";
        assert_eq!(part2(input), 10);
    }

    #[test]
    fn test_floor_div() {
        assert_eq!(floor_div(10, 100), 0);
        assert_eq!(floor_div(100, 100), 1);
        assert_eq!(floor_div(150, 100), 1);
        assert_eq!(floor_div(-1, 100), -1);
        assert_eq!(floor_div(-100, 100), -1);
        assert_eq!(floor_div(-101, 100), -2);
    }
}
