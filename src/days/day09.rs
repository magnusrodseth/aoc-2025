/// Day 9: Movie Theater
///
/// Find the largest rectangle that can be formed using two red tiles as opposite corners.
/// The area of a rectangle with corners at (x1,y1) and (x2,y2) is:
/// (|x2-x1| + 1) * (|y2-y1| + 1)

use std::fs;

/// Parse input into a list of (x, y) coordinates
fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                let x: i64 = parts[0].trim().parse().ok()?;
                let y: i64 = parts[1].trim().parse().ok()?;
                Some((x, y))
            } else {
                None
            }
        })
        .collect()
}

/// Calculate the area of a rectangle with two opposite corners
fn rectangle_area(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    let width = (p2.0 - p1.0).abs() + 1;
    let height = (p2.1 - p1.1).abs() + 1;
    width * height
}

/// Part 1: Find the largest rectangle area using any two red tiles as opposite corners
pub fn part1(input: &str) -> i64 {
    let tiles = parse_input(input);
    let n = tiles.len();

    if n < 2 {
        return 0;
    }

    let mut max_area = 0;

    // Check all pairs of tiles
    for i in 0..n {
        for j in (i + 1)..n {
            let area = rectangle_area(tiles[i], tiles[j]);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

/// Efficient polygon containment checker using coordinate compression
struct EfficientPolygon {
    /// Vertical edges: (x, y_min, y_max) sorted by x
    vertical_edges: Vec<(i64, i64, i64)>,
    /// All unique y coordinates (sorted)
    y_coords: Vec<i64>,
    /// Minimum and maximum y
    min_y: i64,
    max_y: i64,
}

impl EfficientPolygon {
    fn from_tiles(tiles: &[(i64, i64)]) -> Self {
        let mut vertical_edges = Vec::new();

        for i in 0..tiles.len() {
            let p1 = tiles[i];
            let p2 = tiles[(i + 1) % tiles.len()];

            if p1.0 == p2.0 {
                let y_min = p1.1.min(p2.1);
                let y_max = p1.1.max(p2.1);
                vertical_edges.push((p1.0, y_min, y_max));
            }
        }

        vertical_edges.sort();

        let mut y_coords: Vec<i64> = tiles.iter().map(|p| p.1).collect();
        y_coords.sort();
        y_coords.dedup();

        let min_y = *y_coords.first().unwrap();
        let max_y = *y_coords.last().unwrap();

        EfficientPolygon {
            vertical_edges,
            y_coords,
            min_y,
            max_y,
        }
    }

    /// Get the horizontal span(s) at a given y coordinate using ray casting
    fn get_span_at_y(&self, y: i64) -> Option<(i64, i64)> {
        if y < self.min_y || y > self.max_y {
            return None;
        }

        // Find all vertical edges that cross or touch this y level
        let mut crossings: Vec<i64> = self
            .vertical_edges
            .iter()
            .filter(|(_, y_min, y_max)| y >= *y_min && y <= *y_max)
            .map(|(x, _, _)| *x)
            .collect();

        crossings.sort();
        crossings.dedup();

        if crossings.len() >= 2 {
            // For a simple rectilinear polygon, the interior at y is between
            // the leftmost and rightmost crossings
            Some((*crossings.first().unwrap(), *crossings.last().unwrap()))
        } else {
            None
        }
    }

    /// Check if a rectangle is entirely within the polygon
    fn contains_rectangle(&self, p1: (i64, i64), p2: (i64, i64)) -> bool {
        let min_x = p1.0.min(p2.0);
        let max_x = p1.0.max(p2.0);
        let min_y = p1.1.min(p2.1);
        let max_y = p1.1.max(p2.1);

        // Check corners and boundaries by sampling key y-values
        // For a rectilinear polygon, we only need to check at y-coordinates
        // where the polygon boundary changes (i.e., at tile y-coordinates)

        // Collect all relevant y-coordinates: the rectangle's y-range intersected with tile y-coords
        let relevant_ys: Vec<i64> = self
            .y_coords
            .iter()
            .filter(|&&y| y >= min_y && y <= max_y)
            .copied()
            .collect();

        // Also include the rectangle's min_y and max_y if not already present
        let mut check_ys: Vec<i64> = relevant_ys;
        if !check_ys.contains(&min_y) {
            check_ys.push(min_y);
        }
        if !check_ys.contains(&max_y) {
            check_ys.push(max_y);
        }
        check_ys.sort();
        check_ys.dedup();

        // Check each y level
        for y in check_ys {
            if let Some((span_min, span_max)) = self.get_span_at_y(y) {
                if min_x < span_min || max_x > span_max {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

/// Part 2: Find the largest rectangle using red corners, but only including red/green tiles
pub fn part2(input: &str) -> i64 {
    let tiles = parse_input(input);
    let n = tiles.len();

    if n < 2 {
        return 0;
    }

    // Use efficient polygon for large inputs
    let polygon = EfficientPolygon::from_tiles(&tiles);

    let mut max_area = 0;

    // Check all pairs of red tiles as corners
    for i in 0..n {
        for j in (i + 1)..n {
            if polygon.contains_rectangle(tiles[i], tiles[j]) {
                let area = rectangle_area(tiles[i], tiles[j]);
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    max_area
}

/// Entry point for running Day 9 solutions
pub fn run() {
    let input = fs::read_to_string("puzzles/day09/input.txt")
        .expect("Failed to read input file");

    println!("Day 9: Movie Theater");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // Example input from the puzzle description
    const EXAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    // Helper functions for tests using flood-fill approach
    fn build_colored_region(tiles: &[(i64, i64)]) -> HashSet<(i64, i64)> {
        let mut colored: HashSet<(i64, i64)> = HashSet::new();

        if tiles.is_empty() {
            return colored;
        }

        // Add all boundary tiles
        for i in 0..tiles.len() {
            let p1 = tiles[i];
            let p2 = tiles[(i + 1) % tiles.len()];
            add_line_to_set(&mut colored, p1, p2);
        }

        fill_interior(&mut colored, tiles)
    }

    fn add_line_to_set(set: &mut HashSet<(i64, i64)>, p1: (i64, i64), p2: (i64, i64)) {
        let (x1, y1) = p1;
        let (x2, y2) = p2;

        if x1 == x2 {
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            for y in min_y..=max_y {
                set.insert((x1, y));
            }
        } else if y1 == y2 {
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            for x in min_x..=max_x {
                set.insert((x, y1));
            }
        }
    }

    fn fill_interior(
        boundary: &mut HashSet<(i64, i64)>,
        tiles: &[(i64, i64)],
    ) -> HashSet<(i64, i64)> {
        let min_x = tiles.iter().map(|p| p.0).min().unwrap() - 1;
        let max_x = tiles.iter().map(|p| p.0).max().unwrap() + 1;
        let min_y = tiles.iter().map(|p| p.1).min().unwrap() - 1;
        let max_y = tiles.iter().map(|p| p.1).max().unwrap() + 1;

        let mut exterior: HashSet<(i64, i64)> = HashSet::new();
        let mut stack = vec![(min_x, min_y)];

        while let Some((x, y)) = stack.pop() {
            if x < min_x || x > max_x || y < min_y || y > max_y {
                continue;
            }
            if boundary.contains(&(x, y)) || exterior.contains(&(x, y)) {
                continue;
            }
            exterior.insert((x, y));
            stack.push((x + 1, y));
            stack.push((x - 1, y));
            stack.push((x, y + 1));
            stack.push((x, y - 1));
        }

        let mut colored = boundary.clone();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if !exterior.contains(&(x, y)) {
                    colored.insert((x, y));
                }
            }
        }
        colored
    }

    fn rectangle_in_region(p1: (i64, i64), p2: (i64, i64), colored: &HashSet<(i64, i64)>) -> bool {
        let min_x = p1.0.min(p2.0);
        let max_x = p1.0.max(p2.0);
        let min_y = p1.1.min(p2.1);
        let max_y = p1.1.max(p2.1);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if !colored.contains(&(x, y)) {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_parse_input() {
        let tiles = parse_input(EXAMPLE_INPUT);
        assert_eq!(tiles.len(), 8);
        assert_eq!(tiles[0], (7, 1));
        assert_eq!(tiles[1], (11, 1));
        assert_eq!(tiles[7], (7, 3));
    }

    #[test]
    fn test_rectangle_area_example1() {
        // Rectangle between (2,5) and (9,7) should have area 24
        let area = rectangle_area((2, 5), (9, 7));
        assert_eq!(area, 24);
    }

    #[test]
    fn test_rectangle_area_example2() {
        // Rectangle between (7,1) and (11,7) should have area 35
        let area = rectangle_area((7, 1), (11, 7));
        assert_eq!(area, 35);
    }

    #[test]
    fn test_rectangle_area_example3() {
        // Rectangle between (7,3) and (2,3) should have area 6
        // This is a thin horizontal rectangle: width=6, height=1
        let area = rectangle_area((7, 3), (2, 3));
        assert_eq!(area, 6);
    }

    #[test]
    fn test_rectangle_area_example4() {
        // Rectangle between (2,5) and (11,1) should have area 50
        let area = rectangle_area((2, 5), (11, 1));
        assert_eq!(area, 50);
    }

    #[test]
    fn test_part1_example() {
        // From puzzle: largest rectangle has area 50
        let result = part1(EXAMPLE_INPUT);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part1_two_tiles() {
        let input = "0,0\n5,5";
        // Rectangle from (0,0) to (5,5): width=6, height=6, area=36
        assert_eq!(part1(input), 36);
    }

    #[test]
    fn test_part1_same_row() {
        let input = "0,0\n10,0";
        // Rectangle from (0,0) to (10,0): width=11, height=1, area=11
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part1_same_column() {
        let input = "0,0\n0,10";
        // Rectangle from (0,0) to (0,10): width=1, height=11, area=11
        assert_eq!(part1(input), 11);
    }

    // Part 2 tests

    #[test]
    fn test_build_colored_region() {
        let tiles = parse_input(EXAMPLE_INPUT);
        let colored = build_colored_region(&tiles);

        // Red tiles should be in the region
        assert!(colored.contains(&(7, 1)));
        assert!(colored.contains(&(11, 1)));
        assert!(colored.contains(&(2, 3)));

        // Green tiles (connecting lines) should be in the region
        assert!(colored.contains(&(8, 1))); // Between (7,1) and (11,1)
        assert!(colored.contains(&(9, 1)));
        assert!(colored.contains(&(10, 1)));

        // Interior tiles should be in the region
        assert!(colored.contains(&(8, 2))); // Inside the polygon
        assert!(colored.contains(&(5, 4))); // Inside the polygon

        // Exterior tiles should NOT be in the region
        assert!(!colored.contains(&(1, 1))); // Outside
        assert!(!colored.contains(&(12, 5))); // Outside
    }

    #[test]
    fn test_part2_example() {
        // From puzzle: largest valid rectangle has area 24
        let result = part2(EXAMPLE_INPUT);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_part2_rectangle_15() {
        // Rectangle between (7,3) and (11,1) should have area 15
        // and should be valid (all tiles are red/green)
        let tiles = parse_input(EXAMPLE_INPUT);
        let colored = build_colored_region(&tiles);

        assert!(rectangle_in_region((7, 3), (11, 1), &colored));
        assert_eq!(rectangle_area((7, 3), (11, 1)), 15);
    }

    #[test]
    fn test_part2_rectangle_3() {
        // Rectangle between (9,7) and (9,5) should have area 3
        let tiles = parse_input(EXAMPLE_INPUT);
        let colored = build_colored_region(&tiles);

        assert!(rectangle_in_region((9, 7), (9, 5), &colored));
        assert_eq!(rectangle_area((9, 7), (9, 5)), 3);
    }
}
