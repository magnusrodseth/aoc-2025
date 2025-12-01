---
allowed-tools: Skill, Bash(aoc download:*), Bash(aoc submit:*), Bash(cargo test:*), Bash(cargo run:*), Bash(aoc calendar:*), Read, Write, Edit, Glob, Grep, TodoWrite
argument-hint: [day]
description: Solve an Advent of Code puzzle autonomously using TDD
---

# Advent of Code Solver

## Context

- Today's date: !`date +%Y-%m-%d`
- Current December day: !`date +%d`
- Year: 2025

## Pre-flight Check: Is This Day Already Solved?

**IMPORTANT**: Before doing anything else, check if this puzzle has already been completed:

1. Run `aoc calendar --year 2025` to see your progress
2. Look at the output - completed days show decorations/artwork next to them, uncompleted days are blank
3. Additionally, check if a report already exists: `puzzles/day<DD>/report.md`
   - If the report exists and shows both parts completed → **EXIT EARLY** with message: "Day X is already complete! Nothing to do."
4. You can also try downloading the puzzle to see status in the response

If both parts are already solved, do NOT invoke the orchestrator skill. Simply report success and exit.

## Your Task

Solve the Advent of Code puzzle for **Day $ARGUMENTS** (or today's day if no argument provided).

**IMPORTANT**: Use the `aoc-orchestrator` skill to run the full automated workflow:

1. **Invoke the skill**: Use the Skill tool with `skill: "aoc-orchestrator"` to load the orchestration instructions
2. **Follow the workflow**: The skill will guide you through:
   - Fetching the puzzle using `aoc download`
   - Parsing examples from the puzzle description
   - Implementing solutions with TDD (write tests first!)
   - Running `cargo test` until all tests pass
   - Submitting answers with `aoc submit`
   - Handling failures with intelligent retry logic

## Key Commands

```bash
# Download puzzle
aoc download --day <DAY> --year 2025 --puzzle-file puzzles/day<DD>/puzzle.md --input-file puzzles/day<DD>/input.txt --overwrite

# Run tests
cargo test

# Run solution
cargo run -- <DAY>

# Submit answer
aoc submit <PART> <ANSWER> --day <DAY> --year 2025
```

## Success Criteria

- ⭐ Part 1 answer accepted
- ⭐ Part 2 answer accepted
- All tests passing
- Daily report generated in `puzzles/day<DD>/report.md`

## Begin!

Start by invoking the `aoc-orchestrator` skill, then follow its instructions to solve the puzzle autonomously.
