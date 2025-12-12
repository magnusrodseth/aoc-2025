/// Day 12: Christmas Tree Farm
///
/// Bin packing puzzle: determine if polyomino-like shapes can fit into a grid.
/// Shapes can be rotated and flipped. Count how many regions can fit all their presents.

use std::collections::HashSet;
use std::fs;

/// A shape is represented as a set of (row, col) offsets from an origin
type Shape = Vec<(i32, i32)>;

/// Parse a shape from its visual representation
fn parse_shape(shape_str: &str) -> Shape {
    let mut coords = Vec::new();
    for (row, line) in shape_str.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                coords.push((row as i32, col as i32));
            }
        }
    }
    // Normalize: shift so minimum row and col are 0
    if coords.is_empty() {
        return coords;
    }
    let min_row = coords.iter().map(|&(r, _)| r).min().unwrap();
    let min_col = coords.iter().map(|&(_, c)| c).min().unwrap();
    coords.iter().map(|&(r, c)| (r - min_row, c - min_col)).collect()
}

/// Generate all 8 orientations (4 rotations x 2 flips) of a shape
fn all_orientations(shape: &Shape) -> Vec<Shape> {
    let mut orientations = HashSet::new();
    let mut current = shape.clone();

    for _ in 0..4 {
        // Add current orientation
        let normalized = normalize(&current);
        orientations.insert(normalized.clone());

        // Add flipped version
        let flipped: Shape = current.iter().map(|&(r, c)| (r, -c)).collect();
        let normalized_flip = normalize(&flipped);
        orientations.insert(normalized_flip);

        // Rotate 90 degrees clockwise: (r, c) -> (c, -r)
        current = current.iter().map(|&(r, c)| (c, -r)).collect();
    }

    orientations.into_iter().collect()
}

/// Normalize a shape so its minimum row and col are 0, and sort for consistent comparison
fn normalize(shape: &Shape) -> Shape {
    if shape.is_empty() {
        return shape.clone();
    }
    let min_row = shape.iter().map(|&(r, _)| r).min().unwrap();
    let min_col = shape.iter().map(|&(_, c)| c).min().unwrap();
    let mut normalized: Shape = shape.iter().map(|&(r, c)| (r - min_row, c - min_col)).collect();
    normalized.sort();
    normalized
}

/// Parse the full input into shapes and regions
fn parse_input(input: &str) -> (Vec<Vec<Shape>>, Vec<(usize, usize, Vec<usize>)>) {
    let mut shapes: Vec<Vec<Shape>> = Vec::new();
    let mut regions = Vec::new();
    let mut current_shape_lines = Vec::new();
    let mut in_shape = false;

    for line in input.lines() {
        // Check for shape header like "0:" or "5:"
        if line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
           && line.contains(':')
           && !line.contains('x') {
            // Finalize previous shape if any
            if in_shape && !current_shape_lines.is_empty() {
                let shape_str = current_shape_lines.join("\n");
                let base_shape = parse_shape(&shape_str);
                shapes.push(all_orientations(&base_shape));
                current_shape_lines.clear();
            }
            in_shape = true;
        } else if line.contains('x') && line.contains(':') {
            // This is a region line like "4x4: 0 0 0 0 2 0"
            // Finalize previous shape if any
            if in_shape && !current_shape_lines.is_empty() {
                let shape_str = current_shape_lines.join("\n");
                let base_shape = parse_shape(&shape_str);
                shapes.push(all_orientations(&base_shape));
                current_shape_lines.clear();
            }
            in_shape = false;

            // Parse region
            let line_parts: Vec<&str> = line.split(':').collect();
            let dims: Vec<&str> = line_parts[0].split('x').collect();
            let width: usize = dims[0].trim().parse().unwrap();
            let height: usize = dims[1].trim().parse().unwrap();
            let counts: Vec<usize> = line_parts[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            regions.push((width, height, counts));
        } else if in_shape && (line.contains('#') || line.contains('.')) {
            current_shape_lines.push(line);
        }
        // Skip empty lines and other content
    }

    // Handle last shape if any (shouldn't happen if regions follow shapes)
    if in_shape && !current_shape_lines.is_empty() {
        let shape_str = current_shape_lines.join("\n");
        let base_shape = parse_shape(&shape_str);
        shapes.push(all_orientations(&base_shape));
    }

    (shapes, regions)
}

/// Check if a shape can be placed at a given position on the grid
fn can_place(grid: &[Vec<bool>], shape: &Shape, start_row: i32, start_col: i32, width: usize, height: usize) -> bool {
    for &(dr, dc) in shape {
        let r = start_row + dr;
        let c = start_col + dc;
        if r < 0 || c < 0 || r >= height as i32 || c >= width as i32 {
            return false;
        }
        if grid[r as usize][c as usize] {
            return false;
        }
    }
    true
}

/// Place a shape on the grid
fn place_shape(grid: &mut [Vec<bool>], shape: &Shape, start_row: i32, start_col: i32) {
    for &(dr, dc) in shape {
        let r = (start_row + dr) as usize;
        let c = (start_col + dc) as usize;
        grid[r][c] = true;
    }
}

/// Remove a shape from the grid
fn remove_shape(grid: &mut [Vec<bool>], shape: &Shape, start_row: i32, start_col: i32) {
    for &(dr, dc) in shape {
        let r = (start_row + dr) as usize;
        let c = (start_col + dc) as usize;
        grid[r][c] = false;
    }
}

/// Build a list of all pieces we need to place
fn build_pieces(counts: &[usize]) -> Vec<usize> {
    let mut pieces = Vec::new();
    for (shape_idx, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            pieces.push(shape_idx);
        }
    }
    pieces
}

/// Find the first empty cell in the grid (for more efficient search)
fn find_first_empty(grid: &[Vec<bool>]) -> Option<(usize, usize)> {
    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if !cell {
                return Some((r, c));
            }
        }
    }
    None
}

/// Try to solve the packing problem using backtracking
fn solve(
    grid: &mut Vec<Vec<bool>>,
    shapes: &[Vec<Shape>],
    remaining: &mut Vec<usize>,
    width: usize,
    height: usize,
) -> bool {
    solve_inner(grid, shapes, remaining, width, height)
}

fn solve_inner(
    grid: &mut Vec<Vec<bool>>,
    shapes: &[Vec<Shape>],
    remaining: &mut Vec<usize>,
    width: usize,
    height: usize,
) -> bool {
    if remaining.is_empty() {
        return true;
    }

    // Find the first empty cell - we'll try to place shapes that cover it
    // or mark it as permanently empty and move on
    let first_empty = find_first_empty(grid);

    let (target_r, target_c) = match first_empty {
        Some(pos) => pos,
        None => {
            // No empty cells left but still have pieces - can't fit
            return remaining.is_empty();
        }
    };

    // Count remaining cells needed
    let cells_needed: usize = remaining.iter()
        .map(|&idx| shapes[idx][0].len())
        .sum();

    // Count empty cells remaining
    let empty_cells: usize = grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| !cell)
        .count();

    // If we need more cells than available, fail early
    if cells_needed > empty_cells {
        return false;
    }

    // Try each remaining piece type (deduplicate identical shapes)
    let mut tried_shapes = std::collections::HashSet::new();
    for piece_idx in 0..remaining.len() {
        let shape_idx = remaining[piece_idx];
        if !tried_shapes.insert(shape_idx) {
            continue;
        }

        // Try each orientation
        for orientation in &shapes[shape_idx] {
            // Try to place the shape so it covers the first empty cell
            for &(dr, dc) in orientation.iter() {
                let start_row = target_r as i32 - dr;
                let start_col = target_c as i32 - dc;

                if can_place(grid, orientation, start_row, start_col, width, height) {
                    place_shape(grid, orientation, start_row, start_col);
                    let removed = remaining.remove(piece_idx);

                    if solve_inner(grid, shapes, remaining, width, height) {
                        remaining.insert(piece_idx, removed);
                        remove_shape(grid, orientation, start_row, start_col);
                        return true;
                    }

                    remaining.insert(piece_idx, removed);
                    remove_shape(grid, orientation, start_row, start_col);
                }
            }
        }
    }

    // No piece could be placed to cover the first empty cell
    // Mark this cell as "permanently empty" by filling it, then continue
    grid[target_r][target_c] = true;
    let result = solve_inner(grid, shapes, remaining, width, height);
    grid[target_r][target_c] = false;

    result
}

/// Check if a region can fit all the required pieces
fn can_fit(shapes: &[Vec<Shape>], width: usize, height: usize, counts: &[usize]) -> bool {
    // Quick check: total cells
    let total_shape_cells: usize = counts.iter().enumerate()
        .map(|(idx, &count)| {
            if count == 0 || idx >= shapes.len() || shapes[idx].is_empty() {
                0
            } else {
                shapes[idx][0].len() * count
            }
        })
        .sum();

    if total_shape_cells > width * height {
        return false;
    }

    let mut grid = vec![vec![false; width]; height];
    let mut remaining = build_pieces(counts);

    // Sort pieces by size (largest first) for better pruning
    remaining.sort_by(|&a, &b| {
        let size_a = if shapes[a].is_empty() { 0 } else { shapes[a][0].len() };
        let size_b = if shapes[b].is_empty() { 0 } else { shapes[b][0].len() };
        size_b.cmp(&size_a)
    });

    solve(&mut grid, shapes, &mut remaining, width, height)
}

pub fn part1(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);

    regions.iter()
        .filter(|(width, height, counts)| can_fit(&shapes, *width, *height, counts))
        .count()
}

pub fn part2(input: &str) -> usize {
    // Day 12 Part 2 uses the same answer as Part 1 (final puzzle of AoC 2025)
    part1(input)
}

pub fn run() {
    let input = fs::read_to_string("puzzles/day12/input.txt")
        .expect("Failed to read input file");

    println!("Day 12: Christmas Tree Farm");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn test_parse_shape() {
        let shape_str = "###\n#..\n###";
        let shape = parse_shape(shape_str);
        assert_eq!(shape.len(), 7);
        assert!(shape.contains(&(0, 0)));
        assert!(shape.contains(&(0, 1)));
        assert!(shape.contains(&(0, 2)));
        assert!(shape.contains(&(1, 0)));
        assert!(shape.contains(&(2, 0)));
        assert!(shape.contains(&(2, 1)));
        assert!(shape.contains(&(2, 2)));
    }

    #[test]
    fn test_all_orientations() {
        let shape = vec![(0, 0), (0, 1), (1, 0)]; // L-shape
        let orientations = all_orientations(&shape);
        // L-shape should have 4 distinct orientations
        assert!(orientations.len() >= 4);
    }

    #[test]
    fn test_parse_input() {
        let (shapes, regions) = parse_input(EXAMPLE_INPUT);
        assert_eq!(shapes.len(), 6);
        assert_eq!(regions.len(), 3);
        assert_eq!(regions[0], (4, 4, vec![0, 0, 0, 0, 2, 0]));
        assert_eq!(regions[1], (12, 5, vec![1, 0, 1, 0, 2, 2]));
        assert_eq!(regions[2], (12, 5, vec![1, 0, 1, 0, 3, 2]));
    }

    #[test]
    fn test_manual_solution() {
        // Manually verify that shape 4 can fit in the expected positions
        // Solution from puzzle:
        // AAA.
        // ABAB
        // ABAB
        // .BBB

        let (shapes, _) = parse_input(EXAMPLE_INPUT);

        // Shape A should have orientation 2: [(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 2)]
        // This matches A's cells when placed at (0,0)
        let shape_a_expected: Shape = vec![(0,0),(0,1),(0,2),(1,0),(1,2),(2,0),(2,2)];

        // Shape B relative coords (from placing at 1,1):
        // B cells: (1,1),(1,3),(2,1),(2,3),(3,1),(3,2),(3,3)
        // Normalized: (0,0),(0,2),(1,0),(1,2),(2,0),(2,1),(2,2)
        let shape_b_expected: Shape = vec![(0,0),(0,2),(1,0),(1,2),(2,0),(2,1),(2,2)];

        let mut found_a = false;
        let mut found_b = false;
        for orient in shapes[4].iter() {
            let mut sorted = orient.clone();
            sorted.sort();

            let mut a_sorted = shape_a_expected.clone();
            a_sorted.sort();
            if sorted == a_sorted {
                found_a = true;
            }

            let mut b_sorted = shape_b_expected.clone();
            b_sorted.sort();
            if sorted == b_sorted {
                found_b = true;
            }
        }
        assert!(found_a, "Should find orientation matching A's shape");
        assert!(found_b, "Should find orientation matching B's shape");

        // Now manually test if the shapes can be placed
        let mut grid = vec![vec![false; 4]; 4];

        // Find A's orientation and place it at (0,0)
        let orient_a = shapes[4].iter().find(|o| {
            let mut sorted = (*o).clone();
            sorted.sort();
            let mut expected = shape_a_expected.clone();
            expected.sort();
            sorted == expected
        }).unwrap();

        assert!(can_place(&grid, orient_a, 0, 0, 4, 4), "A should be placeable at (0,0)");
        place_shape(&mut grid, orient_a, 0, 0);

        println!("After placing A:");
        for row in &grid {
            println!("{:?}", row);
        }

        // Find B's orientation and place it at (1,1)
        let orient_b = shapes[4].iter().find(|o| {
            let mut sorted = (*o).clone();
            sorted.sort();
            let mut expected = shape_b_expected.clone();
            expected.sort();
            sorted == expected
        }).unwrap();

        println!("Trying to place B at (1,1) with orientation {:?}", orient_b);
        assert!(can_place(&grid, orient_b, 1, 1, 4, 4), "B should be placeable at (1,1)");
    }

    #[test]
    fn test_example_region1() {
        // 4x4 with 2 copies of shape 4
        let (shapes, _) = parse_input(EXAMPLE_INPUT);

        // Debug: print shape 4
        println!("Shape 4 has {} orientations", shapes[4].len());
        for (i, orient) in shapes[4].iter().enumerate() {
            println!("Orientation {}: {:?}", i, orient);
        }

        // Shape 4 should have 7 cells (###, #.., ###)
        let shape4_cells = shapes[4][0].len();
        println!("Shape 4 has {} cells", shape4_cells);

        // Total cells needed = 2 * 7 = 14, available = 4 * 4 = 16
        println!("Total cells needed: {}, available: {}", shape4_cells * 2, 4 * 4);

        let can = can_fit(&shapes, 4, 4, &[0, 0, 0, 0, 2, 0]);
        assert!(can, "Region 1 should be able to fit the pieces");
    }

    #[test]
    fn test_example_region2() {
        // 12x5 with shapes 0, 2, 4x2, 5x2
        let (shapes, _) = parse_input(EXAMPLE_INPUT);
        let can = can_fit(&shapes, 12, 5, &[1, 0, 1, 0, 2, 2]);
        assert!(can, "Region 2 should be able to fit the pieces");
    }

    #[test]
    fn test_example_region3() {
        // 12x5 with shapes 0, 2, 4x3, 5x2 - should NOT fit
        let (shapes, _) = parse_input(EXAMPLE_INPUT);
        let can = can_fit(&shapes, 12, 5, &[1, 0, 1, 0, 3, 2]);
        assert!(!can, "Region 3 should NOT be able to fit the pieces");
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }
}
