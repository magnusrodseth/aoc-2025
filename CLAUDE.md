# Advent of Code 2025 - Automated Agentic Workflow

## Project Overview

This repository represents an experimental approach to Advent of Code 2025, where the entire puzzle-solving workflow is fully automated using Claude Code with custom skills. Rather than manually solving each day's puzzle, this project uses an AI-driven system that can:

1. Fetch daily puzzles automatically
2. Parse puzzle descriptions and extract test cases
3. Implement solutions using Test-Driven Development (TDD)
4. Run tests iteratively until all pass
5. Submit answers automatically
6. Handle submission failures with intelligent retry logic and backoff

## Technology Stack

- **Language**: Rust (all puzzle solutions implemented in Rust)
- **CLI Tool**: `aoc-cli` - Command-line interface for programmatic interaction with adventofcode.com
- **Automation**: Claude Code with custom skills and slash commands
- **Testing**: Rust's built-in test framework

## Usage

Run the `/solve` slash command to trigger the automated solving workflow:

```bash
# Solve today's puzzle (or specify a day)
/solve [day]
```

This triggers the aoc-orchestrator skill which coordinates the entire workflow.

## Architecture

### Core Workflow

```
┌─────────────────────────────────────────────────────────────┐
│                    /solve command trigger                    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  1. Fetch Puzzle (aoc-cli download)                         │
│     - Download puzzle description (puzzle.md)               │
│     - Download puzzle input (input.txt)                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Parse & Extract                                          │
│     - Parse puzzle description from Markdown                │
│     - Extract example inputs and expected outputs           │
│     - Identify part 1 vs part 2 requirements                │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. Test-Driven Development Loop                            │
│     - Generate test cases from examples                     │
│     - Implement solution incrementally                      │
│     - Run: cargo test                                       │
│     - Iterate until all tests pass                          │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Generate Answer                                          │
│     - Run solution with real puzzle input                   │
│     - Extract answer for submission                         │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  5. Submit Answer (aoc-cli submit)                          │
│     - Submit part 1 answer                                  │
│     - Parse submission response                             │
└──────────────────────┬──────────────────────────────────────┘
                       │
              ┌────────┴────────┐
              │                 │
              ▼                 ▼
    ┌─────────────────┐  ┌──────────────────┐
    │  Correct        │  │  Incorrect       │
    │  Move to Part 2 │  │  Retry Logic     │
    └─────────────────┘  └────────┬─────────┘
                                  │
                                  ▼
                    ┌──────────────────────────────┐
                    │  6. Failure Analysis         │
                    │  - Tests pass but answer     │
                    │    submission fails          │
                    │  - Reason about edge cases   │
                    │  - Analyze input patterns    │
                    │  - Check boundary conditions │
                    └──────────┬───────────────────┘
                               │
                               ▼
                    ┌──────────────────────────────┐
                    │  7. Backoff & Retry          │
                    │  - Check wait time from API  │
                    │  - Wait for retry window     │
                    │  - Implement fixes           │
                    │  - Re-submit                 │
                    └──────────────────────────────┘
```

### Directory Structure

```
aoc-2025/
├── CLAUDE.md                    # This file
├── Cargo.toml                   # Rust project manifest
├── src/
│   ├── lib.rs                   # Library code and utilities
│   ├── main.rs                  # CLI entry point
│   └── days/
│       ├── mod.rs               # Days module
│       ├── day01.rs             # Day 1 solution
│       ├── day02.rs             # Day 2 solution
│       └── ...                  # More days
├── .claude/
│   ├── commands/
│   │   └── solve.md             # /solve slash command
│   └── skills/
│       ├── aoc-orchestrator/
│       │   └── SKILL.md         # Main orchestration skill
│       ├── puzzle-fetcher/
│       │   └── SKILL.md         # Fetch and parse puzzles
│       ├── tdd-solver/
│       │   └── SKILL.md         # TDD implementation skill
│       ├── submission-handler/
│       │   └── SKILL.md         # Handle submissions and retries
│       └── daily-reporter/
│           └── SKILL.md         # Generate daily reports
├── puzzles/                     # Downloaded puzzle descriptions
│   ├── day01/
│   │   ├── puzzle.md
│   │   └── input.txt
│   └── ...
└── .adventofcode.session        # Session cookie (gitignored)
```

## AOC-CLI Integration

This project relies on `aoc-cli` for programmatic interaction with adventofcode.com:

### Key Commands Used

```bash
# Download today's puzzle and input
aoc download --day <DAY> --year 2025 \
  --puzzle-file puzzles/day<DAY>/puzzle.md \
  --input-file puzzles/day<DAY>/input.txt

# Submit an answer
aoc submit <PART> <ANSWER> --day <DAY> --year 2025

# Check calendar progress
aoc calendar --year 2025
```

### Session Authentication

The session cookie must be placed in `~/.adventofcode.session` or set via the `ADVENT_OF_CODE_SESSION` environment variable.

## Test-Driven Development Approach

Each day's solution follows a strict TDD methodology:

1. **Parse Examples**: Extract example inputs and outputs from puzzle description
2. **Write Tests First**: Create test cases before implementation
3. **Implement Incrementally**: Build solution step by step
4. **Red-Green-Refactor**:
   - Red: Write failing test
   - Green: Make it pass with minimal code
   - Refactor: Clean up implementation
5. **Run Full Suite**: Ensure all tests pass before submission

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = "example input from puzzle";
        assert_eq!(solve_part1(input), expected_output);
    }

    #[test]
    fn test_part1_example2() {
        // Additional examples if provided
    }

    #[test]
    fn test_part2_example1() {
        let input = "example input for part 2";
        assert_eq!(solve_part2(input), expected_output);
    }
}
```

## Retry Logic & Failure Handling

When tests pass but submission fails:

1. **Analyze Discrepancy**:

   - Compare test inputs vs real input
   - Look for edge cases not covered by examples
   - Check for input parsing issues
   - Verify integer overflow, off-by-one errors, etc.

2. **Extract Wait Time**:

   - Parse AoC response for retry timeout
   - Common messages: "Please wait X minutes before trying again"
   - Extract duration and schedule retry

3. **Implement Fix**:

   - Add new test cases for suspected edge cases
   - Fix implementation
   - Verify all tests still pass

4. **Exponential Backoff**:
   - Respect AoC's rate limiting
   - Wait for specified duration
   - Retry submission
   - Maximum retry attempts: 5 per part

## Claude Skills

The automation is orchestrated through specialized Claude Code skills:

### 1. **aoc-orchestrator**

Main coordinator that manages the entire daily workflow from fetch to submission.

### 2. **puzzle-fetcher**

Downloads puzzles using aoc-cli and parses Markdown to extract examples and requirements.

### 3. **tdd-solver**

Implements solutions using TDD methodology, writing tests first and iterating until all pass.

### 4. **submission-handler**

Manages answer submission, parses responses, and handles retry logic with intelligent backoff.

### 5. **daily-reporter**

Generates daily status reports documenting the solving process and results.

## Goals & Success Metrics

- Fully automated puzzle solving without manual intervention
- Zero manual coding - agent writes all solutions
- Test coverage: 100% of examples from puzzle descriptions
- First submission success rate: Track and improve over 25 days
- Retry success rate: Measure effectiveness of failure analysis

## Safety & Rate Limiting

- Respects AoC's request to not overload servers
- Implements proper backoff on submission failures
- Rate limits: Maximum 1 submission per minute
- Logs all interactions for debugging
- Graceful degradation if automation fails

## Lessons Learned

### Common Algorithm Patterns

| Pattern | When to Use | Example Days |
|---------|-------------|--------------|
| **Memoized DFS** | Graph traversal, path counting | Day 11 |
| **Backtracking** | Constraint satisfaction, bin packing | Day 12 |
| **Brute force** | Small input space (≤2^10 combinations) | Day 10 Part 1 |
| **Greedy** | Locally optimal = globally optimal | Day 3 Part 2 |
| **Union-Find** | Connected components, MST | Day 8 |
| **Coordinate compression** | Large sparse coordinate spaces | Day 9 |
| **Linear algebra** | Toggle systems, optimization | Day 10 |

### Common Pitfalls

1. **Integer overflow**: Start with `i64` or `u64` for large inputs. Day 3 Part 2 answer exceeded `i32::MAX`.

2. **Performance assumptions**: Real input is often 100-1000x larger than examples. Day 9's flood-fill worked on examples but timed out on real input (96,000 × 96,000 coordinate space).

3. **Semantic precision**: "Attempt the first N connections" ≠ "Make N successful connections" (Day 8). Read problem statements carefully.

4. **GF(2) vs Integer arithmetic**: Binary toggle problems (lights out) use XOR/GF(2). Additive problems need integer LP. They look similar but require completely different algorithms (Day 10).

5. **Part 2 key design**: When memoizing, Part 2 often requires extending the memo key (e.g., adding a bitmask for "must visit" constraints in Day 11).

### Success Metrics (Days 1-12)

- **First attempt success rate**: ~85% of submissions correct on first try
- **Days requiring multiple attempts**: Day 10 Part 2 (3 attempts - integer LP)
- **Days requiring optimization**: Day 9 (coordinate compression for performance)
- **Common retry causes**: Integer overflow, off-by-one, misread problem statement
