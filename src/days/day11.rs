/// Day 11: Reactor
///
/// Find all paths from `you` to `out` in a directed graph.
/// Each line defines a device and its outputs.

use std::collections::HashMap;
use std::fs;

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let from = parts[0].trim().to_string();
        let outputs: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        graph.insert(from, outputs);
    }

    graph
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if current == target {
        return 1;
    }

    if let Some(&count) = memo.get(current) {
        return count;
    }

    let count = match graph.get(current) {
        Some(outputs) => {
            outputs.iter()
                .map(|next| count_paths(graph, next, target, memo))
                .sum()
        }
        None => 0, // Dead end
    };

    memo.insert(current.to_string(), count);
    count
}

pub fn part1(input: &str) -> u64 {
    let graph = parse_input(input);
    let mut memo = HashMap::new();
    count_paths(&graph, "you", "out", &mut memo)
}

fn count_paths_with_required(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    must_visit: &[&str],
    visited_required: u32,  // Bitmask of which required nodes we've visited
    memo: &mut HashMap<(String, u32), u64>,
) -> u64 {
    // Check if current node is one of the required ones
    let mut new_visited = visited_required;
    for (i, &node) in must_visit.iter().enumerate() {
        if current == node {
            new_visited |= 1 << i;
        }
    }

    if current == target {
        // Only count if we've visited all required nodes
        let all_visited = (1 << must_visit.len()) - 1;
        return if new_visited == all_visited { 1 } else { 0 };
    }

    let key = (current.to_string(), new_visited);
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    let count = match graph.get(current) {
        Some(outputs) => {
            outputs.iter()
                .map(|next| count_paths_with_required(graph, next, target, must_visit, new_visited, memo))
                .sum()
        }
        None => 0,
    };

    memo.insert(key, count);
    count
}

pub fn part2(input: &str) -> u64 {
    let graph = parse_input(input);
    let mut memo = HashMap::new();
    let must_visit = &["dac", "fft"];
    count_paths_with_required(&graph, "svr", "out", must_visit, 0, &mut memo)
}

pub fn run() {
    let input = fs::read_to_string("puzzles/day11/input.txt")
        .expect("Failed to read input file");

    println!("Day 11: Reactor");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE_INPUT);
        assert_eq!(result, 5, "Should find 5 paths from you to out");
    }

    #[test]
    fn test_parse_input() {
        let graph = parse_input(EXAMPLE_INPUT);
        assert_eq!(graph.get("you"), Some(&vec!["bbb".to_string(), "ccc".to_string()]));
        assert_eq!(graph.get("bbb"), Some(&vec!["ddd".to_string(), "eee".to_string()]));
    }

    #[test]
    fn test_count_simple_path() {
        // Simple chain: a -> b -> out
        let input = "a: b\nb: out\n";
        let graph = parse_input(input);
        let mut memo = HashMap::new();
        let count = count_paths(&graph, "a", "out", &mut memo);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_count_branching_paths() {
        // Branching: a -> b, c -> out
        let input = "a: b c\nb: out\nc: out\n";
        let graph = parse_input(input);
        let mut memo = HashMap::new();
        let count = count_paths(&graph, "a", "out", &mut memo);
        assert_eq!(count, 2);
    }

    const EXAMPLE_INPUT_PART2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE_INPUT_PART2);
        assert_eq!(result, 2, "Should find 2 paths from svr to out visiting both dac and fft");
    }
}
