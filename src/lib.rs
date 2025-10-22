/// Advent of Code 2025 Library
///
/// This library contains all the puzzle solutions and shared utilities
/// for the automated AoC workflow.

pub mod days;

/// Common utilities used across multiple days
pub mod utils {
    use std::fs;

    /// Read a file and return its contents as a String
    pub fn read_input(day: u8) -> String {
        let path = format!("puzzles/day{:02}/input.txt", day);
        fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
    }

    /// Read a file from any path
    pub fn read_file(path: &str) -> String {
        fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to read file: {}", path))
    }

    /// Parse a grid of characters from input
    pub fn parse_char_grid(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }

    /// Parse a list of integers from lines
    pub fn parse_int_lines(input: &str) -> Vec<i64> {
        input
            .lines()
            .filter_map(|line| line.trim().parse().ok())
            .collect()
    }

    /// Split input by blank lines
    pub fn split_by_blank_lines(input: &str) -> Vec<&str> {
        input.split("\n\n").collect()
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;

    #[test]
    fn test_parse_char_grid() {
        let input = "abc\ndef\nghi";
        let grid = parse_char_grid(input);

        assert_eq!(grid.len(), 3);
        assert_eq!(grid[0], vec!['a', 'b', 'c']);
        assert_eq!(grid[1], vec!['d', 'e', 'f']);
        assert_eq!(grid[2], vec!['g', 'h', 'i']);
    }

    #[test]
    fn test_parse_int_lines() {
        let input = "1\n2\n3\n4\n5";
        let numbers = parse_int_lines(input);

        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_split_by_blank_lines() {
        let input = "group1\nline2\n\ngroup2\nline2";
        let groups = split_by_blank_lines(input);

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0], "group1\nline2");
        assert_eq!(groups[1], "group2\nline2");
    }
}
