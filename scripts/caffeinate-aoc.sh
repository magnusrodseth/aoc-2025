#!/bin/bash

# AOC 2025 - Caffeinate Script
# Keeps system awake during puzzle unlock window

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

LOG_DIR="$PROJECT_ROOT/logs"
mkdir -p "$LOG_DIR"

LOG_FILE="$LOG_DIR/caffeinate_$(date +"%Y%m%d_%H%M%S").log"

log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

log "========================================"
log "AOC 2025 Caffeinate - Starting"
log "========================================"

# Check if we're in December 2025
CURRENT_YEAR=$(date +"%Y")
CURRENT_MONTH=$(date +"%m")

if [ "$CURRENT_YEAR" != "2025" ] || [ "$CURRENT_MONTH" != "12" ]; then
    log "Not December 2025, skipping caffeinate"
    exit 0
fi

# Check if day is within AoC 2025 range
export TZ="America/New_York"
CURRENT_DAY=$(date +"%d" | sed 's/^0*//')

if [ "$CURRENT_DAY" -lt 1 ] || [ "$CURRENT_DAY" -gt 12 ]; then
    log "Day $CURRENT_DAY is outside AoC 2025 range (1-12), skipping"
    exit 0
fi

log "✅ Within AoC 2025 active period: Day $CURRENT_DAY of 12"
log "Keeping system awake for 2 hours..."
log "This allows puzzle unlock at 12:00 AM and solver execution at 12:02 AM"

# caffeinate flags:
# -d: Prevent display from sleeping
# -i: Prevent system from idle sleeping
# -m: Prevent disk from idle sleeping
# -s: Prevent system from sleeping (requires AC power)
# -t: Timeout in seconds (7200 = 2 hours)

caffeinate -dims -t 7200 &
CAFFEINATE_PID=$!

log "Caffeinate started (PID: $CAFFEINATE_PID)"
log "System will stay awake until $(date -v+2H +'%Y-%m-%d %H:%M:%S')"
log "This ensures the solver can run at 12:02 AM and provides buffer for retries"

# Wait for caffeinate to finish
wait $CAFFEINATE_PID

log "Caffeinate completed"
log "System can now sleep normally"
log "========================================"
