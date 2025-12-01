# Advent of Code 2025 - Autonomous Agentic Solver

A fully autonomous AI-powered system that solves Advent of Code 2025 puzzles using **Claude Code** with custom skills.

## Overview

This project uses **Claude Code with custom skills and slash commands** to create an autonomous workflow that:

1. **Fetches** puzzles from adventofcode.com using `aoc-cli`
2. **Parses** examples and extracts test cases
3. **Writes Rust solutions** using Test-Driven Development
4. **Runs tests** iteratively until passing
5. **Submits answers** with intelligent retry logic
6. **Handles failures** by analyzing edge cases and re-implementing

## Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install aoc-cli
cargo install aoc-cli

# Setup AoC session cookie
echo "your_session_cookie_here" > ~/.adventofcode.session
```

### Run the Solver

Use the `/solve` slash command in Claude Code:

```bash
# Solve today's puzzle (automatic day detection)
/solve

# Solve a specific day
/solve 1
```

This triggers the aoc-orchestrator skill which coordinates the entire workflow.

## Project Structure

```
aoc-2025/
├── CLAUDE.md                      # Comprehensive project documentation
├── README.md                      # This file
├── Cargo.toml                     # Rust project manifest
│
├── src/
│   ├── main.rs                    # Entry point
│   ├── lib.rs                     # Shared utilities
│   └── days/
│       ├── mod.rs                 # Days module
│       └── day01.rs               # Example day with TDD
│
├── .claude/
│   ├── commands/
│   │   └── solve.md               # /solve slash command
│   └── skills/
│       ├── aoc-orchestrator/      # Main workflow coordinator
│       ├── puzzle-fetcher/        # Download & parse puzzles
│       ├── tdd-solver/            # Implement solutions with TDD
│       ├── submission-handler/    # Submit & handle retries
│       └── daily-reporter/        # Generate status reports
│
└── puzzles/
    └── day01/
        ├── puzzle.md              # Puzzle description
        └── input.txt              # Real input
```

## Features

### Test-Driven Development

Every solution follows strict TDD:

```rust
#[test]
fn test_part1_example() {
    let input = "example from puzzle";
    assert_eq!(solve_part1(input), expected_output);
}
```

See `src/days/day01.rs` for a complete example with:

- Example-based tests from puzzle description
- Edge case tests (empty input, single items, etc.)
- Helper function unit tests

### Intelligent Submission Handling

When tests pass but submission fails:

1. Analyze difference between example and real input
2. Identify common edge cases (overflow, off-by-one, parsing issues)
3. Generate new test cases
4. Re-implement with fixes
5. Retry with exponential backoff

## Claude Code Skills

This project uses specialized Claude skills for each workflow phase:

### 1. AOC Orchestrator

- Coordinates entire workflow
- Manages state between runs
- Handles error recovery

### 2. Puzzle Fetcher

- Downloads puzzles via `aoc-cli`
- Parses Markdown to extract examples
- Structures data for solver

### 3. TDD Solver

- Generates test cases from examples
- Implements solutions incrementally
- Iterates until all tests pass

### 4. Submission Handler

- Submits answers
- Parses responses
- Implements retry logic with backoff

### 5. Daily Reporter

- Generates status reports
- Documents challenges and solutions
- Tracks success metrics

Each skill is documented in `.claude/skills/*/SKILL.md` with detailed instructions.

## Documentation

- **[CLAUDE.md](CLAUDE.md)** - Complete project documentation
- **[.claude/skills/](.claude/skills/)** - Agent skill specifications

## AoC 2025 Schedule

Advent of Code 2025 features puzzles December 1-25.

Puzzles unlock at:

- **12:00 AM EST** (UTC-5)

## Example: Day 1 Solution

The demo day showcases the complete TDD workflow:

```rust
// Parse input into structured data
fn parse_input(input: &str) -> Vec<Vec<i32>> { ... }

// Calculate totals for each group
fn calculate_totals(elf_inventories: &[Vec<i32>]) -> Vec<i32> { ... }

// Part 1: Find maximum
pub fn part1(input: &str) -> i32 { ... }

// Part 2: Sum top 3
pub fn part2(input: &str) -> i32 { ... }

// Comprehensive tests
#[cfg(test)]
mod tests {
    // Example tests from puzzle
    // Edge case tests
    // Unit tests for helpers
}
```

Run it:

```bash
cargo test days::day01  # Run all tests
cargo run -- 1          # Solve Day 1
```

## Goals & Success Metrics

- Fully automated puzzle solving
- Zero manual coding required
- 100% test coverage from examples
- Intelligent failure recovery
- Performance: Solutions < 15 seconds

Track metrics:

- First submission success rate
- Average attempts per part
- Time to correct answer
- Common failure patterns

## Safety & Rate Limiting

- Respects AoC's rate limits (1 submission/minute)
- Maximum 5 retry attempts per part
- Comprehensive logging for debugging
- Graceful degradation on failures

## License

MIT License - See LICENSE file for details

## Acknowledgments

- **Eric Wastl** for creating Advent of Code
- **aoc-cli** maintainers for the excellent CLI tool
- **Anthropic** for Claude Code

## Getting Help

Issues with:

- **AoC puzzles**: See [adventofcode.com](https://adventofcode.com)
- **aoc-cli**: See [aoc-cli docs](https://github.com/scarvalhojr/aoc-cli)
- **This project**: Check [CLAUDE.md](CLAUDE.md)
