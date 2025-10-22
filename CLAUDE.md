# Advent of Code 2025 - Automated Agentic Workflow

## Project Overview

This repository represents an experimental approach to Advent of Code 2025, where the entire puzzle-solving workflow is fully automated using AI agents. Rather than manually solving each day's puzzle, this project builds a sophisticated automated system that can:

1. Fetch daily puzzles automatically
2. Parse puzzle descriptions and extract test cases
3. Implement solutions using Test-Driven Development (TDD)
4. Run tests iteratively until all pass
5. Submit answers automatically
6. Handle submission failures with intelligent retry logic and backoff

## Technology Stack

- **Language**: Rust (all puzzle solutions implemented in Rust)
- **CLI Tool**: `aoc-cli` - Command-line interface for programmatic interaction with adventofcode.com
- **Automation**: Claude AI agents with custom skills
- **Testing**: Rust's built-in test framework
- **Scheduling**: Local cron job for daily execution

## Architecture

### Core Workflow

```
┌─────────────────────────────────────────────────────────────┐
│                      Daily Cron Trigger                      │
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
    │  ✅ Correct     │  │  ❌ Incorrect    │
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
│   ├── main.rs                  # Main automation orchestrator
│   ├── days/
│   │   ├── mod.rs               # Days module
│   │   ├── day01.rs             # Day 1 solution
│   │   ├── day02.rs             # Day 2 solution
│   │   └── ...                  # More days
│   ├── parser/
│   │   ├── mod.rs               # Puzzle parser
│   │   └── example_extractor.rs # Extract test cases from examples
│   ├── runner/
│   │   ├── mod.rs               # Test runner and executor
│   │   └── submission.rs        # Submission handler with retry logic
│   └── utils/
│       ├── mod.rs               # Utility functions
│       └── input.rs             # Input parsing helpers
├── .claude/
│   └── skills/
│       ├── aoc-orchestrator/
│       │   └── SKILL.md         # Main orchestration skill
│       ├── puzzle-fetcher/
│       │   └── SKILL.md         # Fetch and parse puzzles
│       ├── tdd-solver/
│       │   └── SKILL.md         # TDD implementation skill
│       └── submission-handler/
│           └── SKILL.md         # Handle submissions and retries
├── puzzles/                     # Downloaded puzzle descriptions
│   ├── day01/
│   │   ├── puzzle.md
│   │   └── input.txt
│   └── ...
└── .adventofcode.session        # Session cookie (gitignored)
```

## AOC-CLI Integration

This project heavily relies on `aoc-cli` for programmatic interaction with adventofcode.com:

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

# View private leaderboard (optional)
aoc private-leaderboard <ID>
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

## Claude Skills Architecture

The automation is orchestrated through specialized Claude skills:

### 1. **aoc-orchestrator**
Main coordinator that manages the entire daily workflow from fetch to submission.

### 2. **puzzle-fetcher**
Downloads puzzles using aoc-cli and parses Markdown to extract examples and requirements.

### 3. **tdd-solver**
Implements solutions using TDD methodology, writing tests first and iterating until all pass.

### 4. **submission-handler**
Manages answer submission, parses responses, and handles retry logic with intelligent backoff.

## Automation Deployment

### Local Cron Setup

```bash
# Edit crontab
crontab -e

# Run daily at 12:01 AM EST (5:01 AM UTC) during December
1 5 1-25 12 * cd /path/to/aoc-2025 && /usr/local/bin/aoc-orchestrator-run
```

### Manual Execution

```bash
# Run orchestrator for specific day
cargo run -- --day <DAY>

# Run with debug output
cargo run -- --day <DAY> --debug

# Dry run (don't submit)
cargo run -- --day <DAY> --dry-run
```

## Goals & Success Metrics

- ✅ Fully automated puzzle solving without manual intervention
- ✅ Zero manual coding - agent writes all solutions
- ✅ Test coverage: 100% of examples from puzzle descriptions
- ✅ First submission success rate: Track and improve over 25 days
- ✅ Retry success rate: Measure effectiveness of failure analysis
- ✅ Average time to solve: From puzzle unlock to correct submission

## Learning Outcomes

This experiment explores:
- Limits of agentic coding capabilities
- Effectiveness of TDD in automated contexts
- AI reasoning about edge cases and failures
- Reliability of autonomous development workflows
- Integration of AI with external CLIs and APIs

## Safety & Rate Limiting

- Respects AoC's request to not overload servers
- Implements proper backoff on submission failures
- Rate limits: Maximum 1 submission per minute
- Logs all interactions for debugging
- Graceful degradation if automation fails

## Future Enhancements

- [ ] Multi-language support (Python, JavaScript, etc.)
- [ ] Parallel testing of multiple solution approaches
- [ ] Performance optimization tracking
- [ ] Automatic code cleanup and refactoring
- [ ] Integration with private leaderboards
- [ ] Daily solution summary reports
- [ ] Historical analysis of solving patterns
