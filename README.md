# Advent of Code 2025 - Automated Agentic Solver

An experimental approach to Advent of Code using fully automated AI agents that solve puzzles using Test-Driven Development (TDD), with intelligent retry logic and minimal human intervention.

## Overview

This project automates the entire Advent of Code workflow:

1. **Fetch** daily puzzles automatically using `aoc-cli`
2. **Parse** puzzle descriptions to extract examples and requirements
3. **Solve** using strict TDD methodology with iterative testing
4. **Submit** answers with intelligent retry and failure analysis
5. **Handle** edge cases when tests pass but submissions fail

All orchestrated by Claude AI agents with specialized skills for each phase.

## Quick Start

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install aoc-cli
cargo install aoc-cli

# Set up your AoC session cookie
echo "your_session_cookie_here" > ~/.adventofcode.session
```

### Run the Demo

```bash
# Build the project
cargo build

# Run tests to see TDD in action
cargo test

# Run Day 1 solution
cargo run -- 1
```

Expected output:
```
Day 1: Calorie Counting
Part 1: 24000
Part 2: 45000
```

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
│   └── skills/
│       ├── aoc-orchestrator/      # Main workflow coordinator
│       ├── puzzle-fetcher/        # Download & parse puzzles
│       ├── tdd-solver/            # Implement solutions with TDD
│       └── submission-handler/    # Submit & handle retries
│
├── scripts/
│   ├── install-launchd.sh         # Install automated scheduler
│   ├── uninstall-launchd.sh       # Remove scheduler
│   └── run-solver.sh              # Main solver runner
│
├── docs/
│   └── SCHEDULING.md              # macOS scheduling documentation
│
└── puzzles/
    └── day01/
        ├── puzzle.md              # Puzzle description
        ├── input.txt              # Real input
        └── parsed.json            # Parsed examples (future)
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
- ✅ Example-based tests from puzzle description
- ✅ Edge case tests (empty input, single items, etc.)
- ✅ Helper function unit tests
- ✅ Comprehensive test coverage

### Intelligent Submission Handling

When tests pass but submission fails:

1. Analyze difference between example and real input
2. Identify common edge cases (overflow, off-by-one, parsing issues)
3. Generate new test cases
4. Re-implement with fixes
5. Retry with exponential backoff

### Automated Scheduling

Run automatically during Advent of Code 2025 (December 1-12):

```bash
# Install launchd agent (macOS)
./scripts/install-launchd.sh

# Runs daily at 12:02 AM EST (after puzzle unlock)
```

See [docs/SCHEDULING.md](docs/SCHEDULING.md) for complete setup instructions.

## Claude Agent Skills

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

Each skill is documented in `.claude/skills/*/SKILL.md` with detailed instructions.

## Documentation

- **[CLAUDE.md](CLAUDE.md)** - Complete project documentation
- **[docs/SCHEDULING.md](docs/SCHEDULING.md)** - Automated scheduling setup
- **[.claude/skills/](. claude/skills/)** - Agent skill specifications

## AoC 2025 Schedule

Advent of Code 2025 features **12 days of puzzles** (December 1-12).

Puzzles unlock at:
- **12:00 AM EST** (UTC-5)
- Automated solver runs at **12:02 AM EST**

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

- ✅ Fully automated puzzle solving
- ✅ Zero manual coding required
- ✅ 100% test coverage from examples
- ✅ Intelligent failure recovery
- ✅ Performance: Solutions < 15 seconds

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

## Future Enhancements

- [ ] Multi-language support (Python, JavaScript)
- [ ] Parallel testing of multiple approaches
- [ ] Performance benchmarking
- [ ] Automatic code refactoring
- [ ] Solution quality analysis
- [ ] Private leaderboard integration

## Contributing

This is an experimental project exploring the limits of agentic coding. Feel free to:

- Suggest improvements to skills
- Share edge cases that broke automation
- Propose new workflow optimizations

## License

MIT License - See LICENSE file for details

## Acknowledgments

- **Eric Wastl** for creating Advent of Code
- **aoc-cli** maintainers for the excellent CLI tool
- **Anthropic** for Claude and Agent Skills

## Getting Help

Issues with:
- **AoC puzzles**: See [adventofcode.com](https://adventofcode.com)
- **aoc-cli**: See [aoc-cli docs](https://github.com/scarvalhojr/aoc-cli)
- **This project**: Open an issue or check [CLAUDE.md](CLAUDE.md)

---

**Ready to experiment with automated coding?**

```bash
# Start here
cargo test          # See TDD in action
cargo run -- 1      # Run the demo
./scripts/install-launchd.sh  # Automate it
```

Happy coding! 🎄⭐
