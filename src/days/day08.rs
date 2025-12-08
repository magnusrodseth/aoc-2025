/// Day 8: Playground
///
/// Connect junction boxes in 3D space by their closest pairs.
/// Track circuits using Union-Find data structure.

use std::fs;

#[derive(Debug, Clone, Copy)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn distance_squared(&self, other: &Point3D) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same set
        }

        // Union by size
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        true
    }

    fn get_component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = std::collections::HashMap::new();

        for i in 0..n {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }

        let mut result: Vec<usize> = sizes.values().copied().collect();
        result.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending
        result
    }
}

fn parse_input(input: &str) -> Vec<Point3D> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            Point3D {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect()
}

pub fn part1(input: &str) -> i64 {
    solve(input, 1000)
}

fn solve(input: &str, num_attempts: usize) -> i64 {
    let points = parse_input(input);
    let n = points.len();

    // Generate all pairs with distances
    let mut edges = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let dist = points[i].distance_squared(&points[j]);
            edges.push((dist, i, j));
        }
    }

    // Sort by distance
    edges.sort_unstable_by_key(|&(dist, _, _)| dist);

    // Try the closest num_attempts pairs (whether they connect or not)
    let mut uf = UnionFind::new(n);

    for (_, i, j) in edges.iter().take(num_attempts) {
        uf.union(*i, *j); // Try to connect, may or may not succeed
    }

    // Get component sizes and multiply the three largest
    let sizes = uf.get_component_sizes();
    sizes[0] as i64 * sizes[1] as i64 * sizes[2] as i64
}

pub fn part2(input: &str) -> i64 {
    let points = parse_input(input);
    let n = points.len();

    // Generate all pairs with distances
    let mut edges = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let dist = points[i].distance_squared(&points[j]);
            edges.push((dist, i, j));
        }
    }

    // Sort by distance
    edges.sort_unstable_by_key(|&(dist, _, _)| dist);

    // Connect pairs until all in one component
    let mut uf = UnionFind::new(n);
    let mut components_remaining = n;
    let mut last_connection = (0, 0);

    for (_, i, j) in edges {
        if uf.union(i, j) {
            components_remaining -= 1;
            last_connection = (i, j);
            if components_remaining == 1 {
                break;
            }
        }
    }

    // Return product of X coordinates of the last connected pair
    points[last_connection.0].x as i64 * points[last_connection.1].x as i64
}

pub fn run() {
    let input = fs::read_to_string("puzzles/day08/input.txt")
        .expect("Failed to read input file");

    println!("Day 8: Playground");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1_example() {
        // 20 junction boxes, 10 connections
        // Expected: 5 × 4 × 2 = 40
        let points = parse_input(EXAMPLE_INPUT);
        let n = points.len();

        let mut edges = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                let dist = points[i].distance_squared(&points[j]);
                edges.push((dist, i, j));
            }
        }
        edges.sort_unstable_by_key(|&(dist, _, _)| dist);

        let mut uf = UnionFind::new(n);
        let mut connections_made = 0;

        for (_, i, j) in edges {
            if uf.union(i, j) {
                connections_made += 1;
                if connections_made == 10 {
                    break;
                }
            }
        }

        let sizes = uf.get_component_sizes();
        eprintln!("Component sizes: {:?}", sizes);
        eprintln!("Top 3: {} × {} × {} = {}", sizes[0], sizes[1], sizes[2],
                  sizes[0] * sizes[1] * sizes[2]);

        let result = solve(EXAMPLE_INPUT, 10);
        assert_eq!(result, 40, "Example should produce 40");
    }

    #[test]
    fn test_parsing() {
        let points = parse_input(EXAMPLE_INPUT);
        assert_eq!(points.len(), 20, "Should parse 20 points");
        assert_eq!(points[0].x, 162);
        assert_eq!(points[0].y, 817);
        assert_eq!(points[0].z, 812);
    }

    #[test]
    fn test_distance() {
        let p1 = Point3D { x: 0, y: 0, z: 0 };
        let p2 = Point3D { x: 3, y: 4, z: 0 };
        assert_eq!(p1.distance_squared(&p2), 25); // 3² + 4² = 25
    }

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(5);

        // Initially, all separate
        assert_eq!(uf.get_component_sizes().len(), 5);

        // Union 0 and 1
        assert!(uf.union(0, 1));
        assert_eq!(uf.find(0), uf.find(1));

        // Union 2 and 3
        assert!(uf.union(2, 3));

        // Union 0 and 2 (connects the two components)
        assert!(uf.union(0, 2));
        assert_eq!(uf.find(0), uf.find(3));

        // Try to union already connected
        assert!(!uf.union(1, 3));

        let sizes = uf.get_component_sizes();
        assert_eq!(sizes.len(), 2); // Two components
        assert_eq!(sizes[0], 4); // Largest is 4
        assert_eq!(sizes[1], 1); // Other is 1
    }

    #[test]
    fn test_part2_example() {
        // The example says the last connection is between 216,146,977 and 117,168,530
        // Product of X coords: 216 * 117 = 25272
        let result = part2(EXAMPLE_INPUT);
        assert_eq!(result, 25272, "Example should produce 25272");
    }
}
