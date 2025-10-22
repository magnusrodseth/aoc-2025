#!/bin/bash

# AOC 2025 - Main solver runner
# This script is called by launchd

set -e

# Determine script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

# Setup logging
LOG_DIR="$PROJECT_ROOT/logs"
mkdir -p "$LOG_DIR"

TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="$LOG_DIR/solver_$TIMESTAMP.log"

# Logging function
log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

log "========================================"
log "AOC 2025 Solver - Starting"
log "========================================"

# ============================================
# DATE VALIDATION - ADVENT OF CODE 2025
# ============================================
# AoC 2025 runs from December 1-12, 2025
# Puzzles unlock at 12:00 AM EST daily
# This script should ONLY run during this period
# ============================================

# Check if we're in 2025
CURRENT_YEAR=$(date +"%Y")
if [ "$CURRENT_YEAR" != "2025" ]; then
    log "Current year is $CURRENT_YEAR, not 2025. Skipping execution."
    log "AoC 2025 only runs in December 2025."
    exit 0
fi

# Check if we're in December
CURRENT_MONTH=$(date +"%m")
if [ "$CURRENT_MONTH" != "12" ]; then
    log "Current month is $CURRENT_MONTH, not December. Skipping execution."
    log "AoC 2025 only runs in December 2025."
    exit 0
fi

# Determine current day in EST timezone (AoC runs on EST)
export TZ="America/New_York"
CURRENT_DAY=$(date +"%d" | sed 's/^0*//')  # Remove leading zero

log "Date validation passed: December $CURRENT_DAY, 2025 (EST)"

# Check if day is within AoC 2025 range (December 1-12)
# AoC 2025 has only 12 days of puzzles, not the full 25
if [ "$CURRENT_DAY" -lt 1 ]; then
    log "Day $CURRENT_DAY is before AoC 2025 start date (December 1). Skipping."
    exit 0
fi

if [ "$CURRENT_DAY" -gt 12 ]; then
    log "Day $CURRENT_DAY is after AoC 2025 end date (December 12). Skipping."
    log "AoC 2025 only runs for 12 days (Dec 1-12), not the full 25 days."
    exit 0
fi

log "✅ Within AoC 2025 active period: Day $CURRENT_DAY of 12"

# Check if already solved
STATE_FILE="$PROJECT_ROOT/state/day${CURRENT_DAY}.json"
if [ -f "$STATE_FILE" ]; then
    # Check if jq is available
    if command -v jq &> /dev/null; then
        STATUS=$(jq -r '.status' "$STATE_FILE" 2>/dev/null || echo "unknown")
        if [ "$STATUS" = "completed" ]; then
            log "Day $CURRENT_DAY already completed, skipping"
            exit 0
        fi
    else
        log "Warning: jq not installed, cannot check state file"
    fi
fi

log "Running solver for Day $CURRENT_DAY..."

# Ensure cargo is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# For now, just run the simple day runner
# TODO: Replace with actual orchestrator binary when implemented
log "Running: cargo run -- $CURRENT_DAY"

if cargo run -- "$CURRENT_DAY" >> "$LOG_FILE" 2>&1; then
    log "✅ Solver completed successfully for Day $CURRENT_DAY"
else
    log "❌ Solver failed for Day $CURRENT_DAY"
    exit 1
fi

log "========================================"
log "AOC 2025 Solver - Finished"
log "========================================"
