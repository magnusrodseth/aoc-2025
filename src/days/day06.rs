/// Day 6: Trash Compactor
///
/// Parse a horizontal math worksheet where numbers are arranged vertically
/// in columns with operators at the bottom. Solve each problem and sum all answers.

use std::fs;

/// Parse the input into a list of problems
/// Each problem is a vector of numbers and an operator
#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<i64>,
    operator: Operator,
}

fn parse_input(input: &str) -> Vec<Problem> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    // The last line contains operators
    let operator_line = lines.last().unwrap();
    let number_lines = &lines[..lines.len() - 1];

    // Step 1: Find where each operator is located (these mark problem columns)
    let mut operator_positions = Vec::new();
    for (idx, ch) in operator_line.chars().enumerate() {
        if ch == '*' || ch == '+' {
            operator_positions.push((idx, ch));
        }
    }

    // Step 2: For each operator position, extract all numbers from that problem
    let mut problems = Vec::new();

    for (op_col_idx, op_char) in operator_positions {
        let operator = if op_char == '*' {
            Operator::Multiply
        } else {
            Operator::Add
        };

        // For this problem, we need to find the column range it spans
        // We'll look backwards and forwards from the operator to find the problem boundaries

        // Find the start of this problem (leftmost column with content)
        let mut start_col = op_col_idx;
        for col in (0..op_col_idx).rev() {
            let has_content = number_lines.iter().any(|line| {
                col < line.len() && !line.chars().nth(col).unwrap().is_whitespace()
            });
            if has_content {
                start_col = col;
            } else {
                break;
            }
        }

        // Find the end of this problem (rightmost column with content)
        let mut end_col = op_col_idx;
        let max_len = number_lines.iter().map(|l| l.len()).max().unwrap_or(0);
        for col in (op_col_idx + 1)..max_len {
            let has_content = number_lines.iter().any(|line| {
                col < line.len() && !line.chars().nth(col).unwrap().is_whitespace()
            });
            if has_content {
                end_col = col;
            } else {
                break;
            }
        }

        // Extract numbers from each row within this column range
        let mut numbers = Vec::new();
        for line in number_lines {
            if start_col < line.len() {
                let segment = &line[start_col..=end_col.min(line.len() - 1)];
                if let Ok(num) = segment.trim().parse::<i64>() {
                    numbers.push(num);
                }
            }
        }

        problems.push(Problem { numbers, operator });
    }

    problems
}

fn solve_problem(problem: &Problem) -> i64 {
    match problem.operator {
        Operator::Add => problem.numbers.iter().sum(),
        Operator::Multiply => problem.numbers.iter().product(),
    }
}

/// Part 1 solution
pub fn part1(input: &str) -> i64 {
    let problems = parse_input(input);
    problems.iter().map(|p| solve_problem(p)).sum()
}

fn parse_input_part2(input: &str) -> Vec<Problem> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    // The last line contains operators
    let operator_line = lines.last().unwrap();
    let number_lines = &lines[..lines.len() - 1];

    // Step 1: Find where each operator is located (these mark problem columns)
    let mut operator_positions = Vec::new();
    for (idx, ch) in operator_line.chars().enumerate() {
        if ch == '*' || ch == '+' {
            operator_positions.push((idx, ch));
        }
    }

    // Step 2: For each operator position, extract numbers reading column-by-column from right to left
    let mut problems = Vec::new();

    for (op_col_idx, op_char) in operator_positions {
        let operator = if op_char == '*' {
            Operator::Multiply
        } else {
            Operator::Add
        };

        // Find the start and end of this problem (column range)
        let mut start_col = op_col_idx;
        for col in (0..op_col_idx).rev() {
            let has_content = number_lines.iter().any(|line| {
                col < line.len() && !line.chars().nth(col).unwrap().is_whitespace()
            });
            if has_content {
                start_col = col;
            } else {
                break;
            }
        }

        let mut end_col = op_col_idx;
        let max_len = number_lines.iter().map(|l| l.len()).max().unwrap_or(0);
        for col in (op_col_idx + 1)..max_len {
            let has_content = number_lines.iter().any(|line| {
                col < line.len() && !line.chars().nth(col).unwrap().is_whitespace()
            });
            if has_content {
                end_col = col;
            } else {
                break;
            }
        }

        // Read numbers column-by-column from right to left
        let mut numbers = Vec::new();
        for col in (start_col..=end_col).rev() {
            // Read this column top to bottom to form a number
            let mut digits = String::new();
            for line in number_lines {
                if col < line.len() {
                    let ch = line.chars().nth(col).unwrap();
                    if ch.is_ascii_digit() {
                        digits.push(ch);
                    }
                }
            }
            if !digits.is_empty() {
                if let Ok(num) = digits.parse::<i64>() {
                    numbers.push(num);
                }
            }
        }

        problems.push(Problem { numbers, operator });
    }

    problems
}

/// Part 2 solution
pub fn part2(input: &str) -> i64 {
    let problems = parse_input_part2(input);
    problems.iter().map(|p| solve_problem(p)).sum()
}

/// Entry point for running this day
pub fn run() {
    let input = fs::read_to_string("puzzles/day06/input.txt")
        .expect("Failed to read input file");

    println!("Day 6: Trash Compactor");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE_INPUT);
        assert_eq!(
            result, 4277556,
            "Example should give grand total of 4277556"
        );
    }

    #[test]
    fn test_solve_problem_multiply() {
        let problem = Problem {
            numbers: vec![123, 45, 6],
            operator: Operator::Multiply,
        };
        assert_eq!(solve_problem(&problem), 33210, "123 * 45 * 6 = 33210");
    }

    #[test]
    fn test_solve_problem_add() {
        let problem = Problem {
            numbers: vec![328, 64, 98],
            operator: Operator::Add,
        };
        assert_eq!(solve_problem(&problem), 490, "328 + 64 + 98 = 490");
    }

    #[test]
    fn test_parse_input() {
        let problems = parse_input(EXAMPLE_INPUT);
        assert_eq!(problems.len(), 4, "Should have 4 problems");

        // First problem: 123 * 45 * 6
        assert_eq!(problems[0].numbers, vec![123, 45, 6]);
        assert_eq!(problems[0].operator, Operator::Multiply);

        // Second problem: 328 + 64 + 98
        assert_eq!(problems[1].numbers, vec![328, 64, 98]);
        assert_eq!(problems[1].operator, Operator::Add);

        // Third problem: 51 * 387 * 215
        assert_eq!(problems[2].numbers, vec![51, 387, 215]);
        assert_eq!(problems[2].operator, Operator::Multiply);

        // Fourth problem: 64 + 23 + 314
        assert_eq!(problems[3].numbers, vec![64, 23, 314]);
        assert_eq!(problems[3].operator, Operator::Add);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE_INPUT);
        assert_eq!(
            result, 3263827,
            "Part 2 example should give grand total of 3263827"
        );
    }

    #[test]
    fn test_parse_input_part2() {
        let problems = parse_input_part2(EXAMPLE_INPUT);
        assert_eq!(problems.len(), 4, "Should have 4 problems");

        // Reading right to left, column by column:
        // Rightmost problem (4th): 4 + 431 + 623 = 1058
        assert_eq!(problems[3].numbers, vec![4, 431, 623]);
        assert_eq!(problems[3].operator, Operator::Add);

        // Second from right (3rd): 175 * 581 * 32 = 3253600
        assert_eq!(problems[2].numbers, vec![175, 581, 32]);
        assert_eq!(problems[2].operator, Operator::Multiply);

        // Third from right (2nd): 8 + 248 + 369 = 625
        assert_eq!(problems[1].numbers, vec![8, 248, 369]);
        assert_eq!(problems[1].operator, Operator::Add);

        // Leftmost (1st): 356 * 24 * 1 = 8544
        assert_eq!(problems[0].numbers, vec![356, 24, 1]);
        assert_eq!(problems[0].operator, Operator::Multiply);
    }
}
