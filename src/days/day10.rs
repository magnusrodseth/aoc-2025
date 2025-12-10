/// Day 10: Factory
///
/// This is a "lights out" puzzle where we need to configure indicator lights
/// by pressing buttons that toggle specific lights. This is equivalent to
/// solving a system of linear equations over GF(2) (binary field).
///
/// For each machine:
/// - Start with all lights off (0)
/// - Need to reach target pattern shown in brackets
/// - Each button toggles specific lights
/// - Find minimum number of button presses
///
/// Algorithm: Gaussian elimination over GF(2)

use std::fs;

#[derive(Debug, Clone)]
struct Machine {
    target: Vec<bool>,  // Target state for each light
    buttons: Vec<Vec<usize>>,  // Which lights each button toggles
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_machine(line))
        .collect()
}

fn parse_machine(line: &str) -> Machine {
    // Parse format: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

    // Extract target pattern from [...]
    let target_start = line.find('[').expect("No opening bracket") + 1;
    let target_end = line.find(']').expect("No closing bracket");
    let target_str = &line[target_start..target_end];
    let target: Vec<bool> = target_str.chars().map(|c| c == '#').collect();

    // Extract buttons from (...)
    let mut buttons = Vec::new();
    let mut i = target_end + 1;
    while let Some(start) = line[i..].find('(') {
        let start = i + start;
        let end = line[start..].find(')').expect("No closing paren") + start;
        let button_str = &line[start + 1..end];

        let indices: Vec<usize> = button_str
            .split(',')
            .map(|s| s.trim().parse().expect("Invalid button index"))
            .collect();

        buttons.push(indices);
        i = end + 1;
    }

    Machine { target, buttons }
}

fn parse_joltage_requirements(line: &str) -> Vec<i64> {
    // Extract joltage from {...}
    let start = line.find('{').expect("No opening brace") + 1;
    let end = line.find('}').expect("No closing brace");
    let joltage_str = &line[start..end];

    joltage_str
        .split(',')
        .map(|s| s.trim().parse().expect("Invalid joltage value"))
        .collect()
}

fn solve_machine(machine: &Machine) -> usize {
    let n_lights = machine.target.len();
    let n_buttons = machine.buttons.len();

    // Try all possible combinations of button presses (brute force for small n_buttons)
    // This is feasible since AoC puzzles typically have reasonable button counts
    let mut min_presses = usize::MAX;

    // Try all 2^n_buttons combinations
    for mask in 0_u32..(1 << n_buttons) {
        let mut state = vec![false; n_lights];

        // Apply each button that's set in the mask
        for button_idx in 0..n_buttons {
            if mask & (1 << button_idx) != 0 {
                // Press this button
                for &light_idx in &machine.buttons[button_idx] {
                    state[light_idx] = !state[light_idx];
                }
            }
        }

        // Check if we reached the target
        if state == machine.target {
            let presses = mask.count_ones() as usize;
            min_presses = min_presses.min(presses);
        }
    }

    min_presses
}


pub fn part1(input: &str) -> usize {
    let machines = parse_input(input);
    machines.iter().map(|m| solve_machine(m)).sum()
}

fn solve_machine_joltage(buttons: &[Vec<usize>], joltage_targets: &[i64]) -> i64 {
    let n_counters = joltage_targets.len();
    let n_buttons = buttons.len();

    // Build coefficient matrix: matrix[counter][button] = 1 if button affects counter
    let mut coeff: Vec<Vec<i64>> = vec![vec![0; n_buttons]; n_counters];
    for (button_idx, button) in buttons.iter().enumerate() {
        for &counter_idx in button {
            if counter_idx < n_counters {
                coeff[counter_idx][button_idx] = 1;
            }
        }
    }

    // Use simplex-like approach: find a feasible solution, then optimize
    // Since this is integer programming with equality constraints, we can use
    // Gaussian elimination to find the solution space, then search for minimum

    solve_min_sum_ilp(&coeff, joltage_targets)
}

/// Solve using Gaussian elimination to reduce the system, then enumerate over free variables
fn solve_min_sum_ilp(coeff: &[Vec<i64>], targets: &[i64]) -> i64 {
    let n_counters = coeff.len();
    let n_buttons = if n_counters > 0 { coeff[0].len() } else { return 0; };

    // Convert to rational arithmetic for exact computation
    // Create augmented matrix [A | b]
    type Rat = (i64, i64); // (numerator, denominator)

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a.abs() } else { gcd(b, a % b) }
    }

    fn rat_reduce(r: Rat) -> Rat {
        if r.0 == 0 { return (0, 1); }
        let g = gcd(r.0, r.1);
        let (n, d) = (r.0 / g, r.1 / g);
        if d < 0 { (-n, -d) } else { (n, d) }
    }

    fn rat_sub(a: Rat, b: Rat) -> Rat {
        rat_reduce((a.0 * b.1 - b.0 * a.1, a.1 * b.1))
    }

    fn rat_mul(a: Rat, b: Rat) -> Rat {
        rat_reduce((a.0 * b.0, a.1 * b.1))
    }

    fn rat_div(a: Rat, b: Rat) -> Rat {
        rat_mul(a, (b.1, b.0))
    }

    // Build augmented matrix
    let mut aug: Vec<Vec<Rat>> = vec![vec![(0, 1); n_buttons + 1]; n_counters];
    for i in 0..n_counters {
        for j in 0..n_buttons {
            aug[i][j] = (coeff[i][j], 1);
        }
        aug[i][n_buttons] = (targets[i], 1);
    }

    // Gaussian elimination with partial pivoting
    let mut pivot_cols = Vec::new();
    let mut row = 0;

    for col in 0..n_buttons {
        if row >= n_counters { break; }

        // Find pivot
        let mut pivot_row = None;
        for r in row..n_counters {
            if aug[r][col].0 != 0 {
                pivot_row = Some(r);
                break;
            }
        }

        let pivot_row = match pivot_row {
            Some(r) => r,
            None => continue, // No pivot in this column
        };

        // Swap rows
        aug.swap(row, pivot_row);
        pivot_cols.push(col);

        // Scale pivot row
        let pivot = aug[row][col];
        for j in col..=n_buttons {
            aug[row][j] = rat_div(aug[row][j], pivot);
        }

        // Eliminate
        for r in 0..n_counters {
            if r != row && aug[r][col].0 != 0 {
                let factor = aug[r][col];
                for j in col..=n_buttons {
                    aug[r][j] = rat_sub(aug[r][j], rat_mul(factor, aug[row][j]));
                }
            }
        }

        row += 1;
    }

    // Now we have row echelon form
    // pivot_cols contains the basic variables
    // Free variables are all other columns

    let free_cols: Vec<usize> = (0..n_buttons).filter(|c| !pivot_cols.contains(c)).collect();
    let n_free = free_cols.len();

    // For each assignment of free variables, compute basic variables
    // Basic variable i (in pivot_cols[i]) = aug[i][n_buttons] - sum(aug[i][free_j] * free_j)

    // If no free variables, we have a unique solution
    if n_free == 0 {
        let mut result = vec![0i64; n_buttons];
        for (i, &col) in pivot_cols.iter().enumerate() {
            let (n, d) = aug[i][n_buttons];
            if d != 1 || n < 0 { return 0; } // Non-integer or negative
            result[col] = n;
        }
        return result.iter().sum();
    }

    // With free variables, we need to search
    // Limit the search space by bounding free variables
    let max_target = *targets.iter().max().unwrap_or(&0);

    let mut best = i64::MAX;
    let mut free_values = vec![0i64; n_free];

    search_free_vars(&aug, &pivot_cols, &free_cols, n_buttons, targets,
                     &mut free_values, 0, max_target, &mut best);

    if best == i64::MAX { 0 } else { best }
}

fn search_free_vars(
    aug: &[Vec<(i64, i64)>],
    pivot_cols: &[usize],
    free_cols: &[usize],
    n_buttons: usize,
    targets: &[i64],
    free_values: &mut [i64],
    idx: usize,
    max_val: i64,
    best: &mut i64
) {
    let n_free = free_cols.len();

    if idx == n_free {
        // Compute basic variables
        let mut result = vec![0i64; n_buttons];

        // Set free variables
        for (i, &col) in free_cols.iter().enumerate() {
            result[col] = free_values[i];
        }

        // Compute basic variables
        for (i, &col) in pivot_cols.iter().enumerate() {
            let (rhs_n, rhs_d) = aug[i][n_buttons];
            let mut val_n = rhs_n;
            let mut val_d = rhs_d;

            // Subtract contributions from free variables
            for (j, &free_col) in free_cols.iter().enumerate() {
                let (coef_n, coef_d) = aug[i][free_col];
                // val -= coef * free_values[j]
                // val_n/val_d -= (coef_n/coef_d) * free_values[j]
                let sub_n = coef_n * free_values[j];
                let sub_d = coef_d;
                // val_n/val_d - sub_n/sub_d = (val_n * sub_d - sub_n * val_d) / (val_d * sub_d)
                val_n = val_n * sub_d - sub_n * val_d;
                val_d = val_d * sub_d;
                // Reduce
                let g = gcd_helper(val_n, val_d);
                val_n /= g;
                val_d /= g;
                if val_d < 0 { val_n = -val_n; val_d = -val_d; }
            }

            // Check if integer
            if val_d != 1 && val_d != -1 {
                if val_n % val_d != 0 { return; }
                val_n /= val_d;
            } else if val_d == -1 {
                val_n = -val_n;
            }

            if val_n < 0 { return; } // Negative solution
            result[col] = val_n;
        }

        let sum: i64 = result.iter().sum();
        if sum < *best {
            *best = sum;
        }
        return;
    }

    // Determine max value for this free variable
    // Based on ensuring basic variables stay non-negative
    let max_for_this = max_val; // Use full target range

    for val in 0..=max_for_this {
        free_values[idx] = val;
        search_free_vars(aug, pivot_cols, free_cols, n_buttons, targets,
                        free_values, idx + 1, max_val, best);
    }
    free_values[idx] = 0;
}

fn gcd_helper(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd_helper(b, a % b) }
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let machine = parse_machine(line);
            let joltage = parse_joltage_requirements(line);
            solve_machine_joltage(&machine.buttons, &joltage)
        })
        .sum()
}

pub fn run() {
    let input = fs::read_to_string("puzzles/day10/input.txt")
        .expect("Failed to read input file");

    println!("Day 10: Factory");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE_INPUT);
        assert_eq!(result, 7, "Example should give 2+3+2=7");
    }

    #[test]
    fn test_parse_machine() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine(line);

        assert_eq!(machine.target, vec![false, true, true, false]);
        assert_eq!(machine.buttons.len(), 6);
        assert_eq!(machine.buttons[0], vec![3]);
        assert_eq!(machine.buttons[1], vec![1, 3]);
    }

    #[test]
    fn test_solve_first_machine() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine(line);
        let result = solve_machine(&machine);
        assert_eq!(result, 2, "First machine needs 2 button presses");
    }

    #[test]
    fn test_solve_second_machine() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = parse_machine(line);
        let result = solve_machine(&machine);
        assert_eq!(result, 3, "Second machine needs 3 button presses");
    }

    #[test]
    fn test_solve_third_machine() {
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = parse_machine(line);
        let result = solve_machine(&machine);
        assert_eq!(result, 2, "Third machine needs 2 button presses");
    }

    #[test]
    fn test_parse_joltage() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let joltage = parse_joltage_requirements(line);
        assert_eq!(joltage, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE_INPUT);
        assert_eq!(result, 33, "Example should give 10+12+11=33");
    }

    #[test]
    fn test_solve_first_machine_joltage() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine(line);
        let joltage = parse_joltage_requirements(line);
        let result = solve_machine_joltage(&machine.buttons, &joltage);
        assert_eq!(result, 10, "First machine joltage needs 10 button presses");
    }

    #[test]
    fn test_solve_second_machine_joltage() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = parse_machine(line);
        let joltage = parse_joltage_requirements(line);
        let result = solve_machine_joltage(&machine.buttons, &joltage);
        assert_eq!(result, 12, "Second machine joltage needs 12 button presses");
    }

    #[test]
    fn test_solve_third_machine_joltage() {
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = parse_machine(line);
        let joltage = parse_joltage_requirements(line);
        let result = solve_machine_joltage(&machine.buttons, &joltage);
        assert_eq!(result, 11, "Third machine joltage needs 11 button presses");
    }
}
