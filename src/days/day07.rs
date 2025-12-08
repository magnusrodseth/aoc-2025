/// Day 7: Laboratories
///
/// Simulate tachyon beams in a manifold. Beams start at S, travel downward.
/// When a beam hits a splitter (^), it stops and two new beams emerge
/// from the left and right of the splitter.

use std::collections::HashSet;
use std::fs;

/// Parse the grid and find the start position
fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Find the starting position 'S'
    let mut start = (0, 0);
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'S' {
                start = (row, col);
            }
        }
    }

    (grid, start)
}

/// Simulate the tachyon beam and count splits
fn simulate_beam(grid: &[Vec<char>], start: (usize, usize)) -> usize {
    let rows = grid.len();
    let mut split_count = 0;

    // Active beams: positions where beams are currently moving down
    // We use a set to avoid counting the same beam position multiple times
    let mut active_beams: HashSet<(usize, usize)> = HashSet::new();

    // Start with beam at S position, moving downward
    active_beams.insert(start);

    // Process row by row, moving beams downward
    let mut current_row = start.0;

    while current_row < rows && !active_beams.is_empty() {
        // Move all beams down one row
        current_row += 1;
        if current_row >= rows {
            break;
        }

        // Check what each beam hits in the new row
        let mut next_beams: HashSet<(usize, usize)> = HashSet::new();

        for &(_, col) in &active_beams {
            if col < grid[current_row].len() {
                let ch = grid[current_row][col];
                if ch == '^' {
                    // Beam hits a splitter - it stops and creates two new beams
                    split_count += 1;

                    // New beam to the left (if in bounds)
                    if col > 0 {
                        next_beams.insert((current_row, col - 1));
                    }
                    // New beam to the right (if in bounds)
                    if col + 1 < grid[current_row].len() {
                        next_beams.insert((current_row, col + 1));
                    }
                } else {
                    // Beam continues downward
                    next_beams.insert((current_row, col));
                }
            }
        }

        active_beams = next_beams;
    }

    split_count
}

/// Part 1 solution: count total number of splits
pub fn part1(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    simulate_beam(&grid, start)
}

/// Part 2 solution: Count the number of distinct timelines
/// Each path through the manifold represents a timeline where the particle
/// takes different left/right choices at each splitter.
pub fn part2(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    count_timelines(&grid, start)
}

/// Count the number of distinct timelines (paths) through the manifold
/// Each timeline represents a unique sequence of left/right choices at splitters
fn count_timelines(grid: &[Vec<char>], start: (usize, usize)) -> usize {
    let rows = grid.len();

    // We track (row, col, timeline_count) for each active beam position
    // Multiple timelines can be at the same position, so we track counts
    use std::collections::HashMap;

    // Map from column position to number of timelines at that position
    // We use isize to handle negative columns (beams that exit left)
    let mut timeline_counts: HashMap<isize, usize> = HashMap::new();
    timeline_counts.insert(start.1 as isize, 1);

    let mut current_row = start.0;
    let mut exited_timelines: usize = 0;

    while current_row < rows && !timeline_counts.is_empty() {
        current_row += 1;
        if current_row >= rows {
            break;
        }

        let row_len = grid[current_row].len() as isize;
        let mut next_counts: HashMap<isize, usize> = HashMap::new();

        for (&col, &count) in &timeline_counts {
            // Check if beam is out of bounds
            if col < 0 || col >= row_len {
                // Beam has exited the manifold - these timelines are complete
                exited_timelines += count;
                continue;
            }

            let ch = grid[current_row][col as usize];
            if ch == '^' {
                // Each timeline splits into two: one goes left, one goes right
                *next_counts.entry(col - 1).or_insert(0) += count;
                *next_counts.entry(col + 1).or_insert(0) += count;
            } else {
                // Timelines continue downward
                *next_counts.entry(col).or_insert(0) += count;
            }
        }

        timeline_counts = next_counts;
    }

    // Total number of timelines is:
    // - Timelines that exited the sides during simulation
    // - Plus timelines that exited the bottom (still active at the end)
    exited_timelines + timeline_counts.values().sum::<usize>()
}

/// Entry point for running this day
pub fn run() {
    let input = fs::read_to_string("puzzles/day07/input.txt")
        .expect("Failed to read input file");

    println!("Day 7: Laboratories");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn test_parse_input() {
        let (grid, start) = parse_input(EXAMPLE_INPUT);
        assert_eq!(start, (0, 7), "S should be at row 0, col 7");
        assert_eq!(grid[0][7], 'S');
        assert_eq!(grid[2][7], '^');
    }

    #[test]
    fn test_simple_single_split() {
        let input = "S\n.\n^";
        assert_eq!(part1(input), 1, "Single splitter should cause 1 split");
    }

    #[test]
    fn test_no_splitters() {
        let input = "S\n.\n.";
        assert_eq!(part1(input), 0, "No splitters means no splits");
    }

    #[test]
    fn test_two_level_split() {
        // S at center, one splitter, then two more below
        let input = "..S..\n.....\n..^..\n.....\n.^.^.";
        assert_eq!(part1(input), 3, "Should have 3 splits: 1 at first level, 2 at second");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 40);
    }

    #[test]
    fn test_part2_simple() {
        // Single splitter: 2 timelines
        let input = "S\n.\n^";
        assert_eq!(part2(input), 2, "Single splitter creates 2 timelines");
    }

    #[test]
    fn test_part2_no_splitters() {
        let input = "S\n.\n.";
        assert_eq!(part2(input), 1, "No splitters means 1 timeline");
    }

    #[test]
    fn test_part2_two_sequential_splitters() {
        // Two sequential splitters (aligned): 2 * 2 = 4 timelines
        let input = ".S.\n...\n.^.\n...\n^.^";
        assert_eq!(part2(input), 4, "Two levels of splitting creates 4 timelines");
    }
}
