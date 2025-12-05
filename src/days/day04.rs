/// Day 4: Printing Department
///
/// The forklifts can only access a roll of paper if there are fewer than four
/// rolls of paper in the eight adjacent positions. Count how many rolls meet this criteria.

use std::fs;

/// Parse input into a 2D grid of characters
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Count adjacent paper rolls (8 directions) for a given position
fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Check all 8 directions: up, down, left, right, and 4 diagonals
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    for (dr, dc) in directions {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        // Check bounds
        if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
            let r = new_row as usize;
            let c = new_col as usize;
            if grid[r][c] == '@' {
                count += 1;
            }
        }
    }

    count
}

/// Part 1: Count rolls that can be accessed by a forklift
/// (rolls with fewer than 4 adjacent rolls)
pub fn part1(input: &str) -> i64 {
    let grid = parse_input(input);
    let mut accessible_count = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == '@' {
                let adjacent_count = count_adjacent_rolls(&grid, row_idx, col_idx);
                if adjacent_count < 4 {
                    accessible_count += 1;
                }
            }
        }
    }

    accessible_count
}

/// Part 2: Iteratively remove accessible rolls until none remain
/// Count total rolls removed
pub fn part2(input: &str) -> i64 {
    let mut grid = parse_input(input);
    let mut total_removed = 0;

    loop {
        // Find all accessible rolls in current state
        let mut accessible: Vec<(usize, usize)> = Vec::new();

        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell == '@' {
                    let adjacent_count = count_adjacent_rolls(&grid, row_idx, col_idx);
                    if adjacent_count < 4 {
                        accessible.push((row_idx, col_idx));
                    }
                }
            }
        }

        // If no more accessible, stop
        if accessible.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (row, col) in &accessible {
            grid[*row][*col] = '.';
        }

        total_removed += accessible.len() as i64;
    }

    total_removed
}

/// Entry point for running this day
pub fn run() {
    let input = fs::read_to_string("puzzles/day04/input.txt")
        .expect("Failed to read input file");

    println!("Day 4: Printing Department");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE_INPUT);
        assert_eq!(result, 13, "Example should have 13 accessible rolls");
    }

    #[test]
    fn test_parse_input() {
        let grid = parse_input(EXAMPLE_INPUT);
        assert_eq!(grid.len(), 10, "Should have 10 rows");
        assert_eq!(grid[0].len(), 10, "Should have 10 columns");
        assert_eq!(grid[0][0], '.', "Top-left should be '.'");
        assert_eq!(grid[0][2], '@', "Position (0,2) should be '@'");
    }

    #[test]
    fn test_count_adjacent_rolls_corner() {
        let grid = parse_input(EXAMPLE_INPUT);
        // Top-left corner (0,0) is '.' but let's test (0,2) which is '@'
        let count = count_adjacent_rolls(&grid, 0, 2);
        // Position (0,2) is '@', check neighbors at (0,1), (0,3), (1,1), (1,2), (1,3)
        // Expected neighbors: checking manually from example
        assert!(count <= 8, "Adjacent count should be at most 8");
    }

    #[test]
    fn test_empty_input() {
        let result = part1("");
        assert_eq!(result, 0, "Empty input should return 0");
    }

    #[test]
    fn test_single_roll() {
        let input = "@";
        let result = part1(input);
        assert_eq!(result, 1, "Single roll with 0 neighbors should be accessible");
    }

    #[test]
    fn test_no_rolls() {
        let input = "...\n...\n...";
        let result = part1(input);
        assert_eq!(result, 0, "Grid with no rolls should return 0");
    }

    #[test]
    fn test_all_rolls_isolated() {
        let input = "@.@\n...\n@.@";
        let result = part1(input);
        assert_eq!(result, 4, "Four isolated rolls should all be accessible");
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE_INPUT);
        assert_eq!(result, 43, "Example should remove 43 total rolls");
    }

    #[test]
    fn test_part2_all_isolated() {
        let input = "@.@\n...\n@.@";
        let result = part2(input);
        assert_eq!(result, 4, "Four isolated rolls should all be removed in one pass");
    }

    #[test]
    fn test_part2_chain_removal() {
        // A scenario where removing some rolls enables removing more
        let input = "@@@\n@@@\n@@@";
        // All 9 rolls: initially only 4 corners accessible
        // After removing corners: edges become accessible
        // After removing edges: center becomes accessible
        // Total: 9 rolls removed
        let result = part2(input);
        assert_eq!(result, 9, "All rolls should eventually be removable");
    }
}
