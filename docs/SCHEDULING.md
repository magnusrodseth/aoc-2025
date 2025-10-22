# Automated Scheduling Setup for macOS

## Overview

This document describes how to set up automated execution of the Advent of Code solver on macOS, ensuring it runs reliably even when your MacBook is asleep or closed.

## Why launchd Instead of Cron

On macOS, `launchd` is the preferred scheduling system over cron for several reasons:

- **Wake from sleep**: launchd can wake the system to run scheduled tasks
- **Missed execution handling**: If system was asleep during scheduled time, launchd can run the task on next wake
- **Native macOS integration**: Better integrated with system power management
- **More reliable**: Apple's recommended approach for scheduled tasks

## Important Notes About Sleep and Scheduling

### Reality Check

**Standard limitation**: When a MacBook's lid is closed and it's in sleep mode, even launchd **cannot wake the system** to run tasks. This is a macOS security and power management feature.

### Solutions

You have three options:

#### Option 1: Keep System Awake During Advent of Code (Recommended)

During December 1-12, 2025, prevent sleep during puzzle unlock times:

```bash
# Install caffeinate wrapper or use built-in caffeinate
# Keep system awake from 11:55 PM to 12:30 AM EST (when puzzles unlock at midnight EST)

# Create a separate launchd job to keep system awake
# See: caffeinate-during-aoc.plist below
```

#### Option 2: Power Nap (Limited Support)

Enable Power Nap on supported Macs (only works on AC power):

```
System Preferences > Battery > Power Adapter > Enable Power Nap
```

**Limitation**: Power Nap only runs specific Apple tasks, not custom launchd jobs.

#### Option 3: Keep MacBook Open or Plugged In

The simplest solution:
- Keep lid open during scheduled times
- Or use in clamshell mode with external display
- Or keep plugged in with "Prevent computer from sleeping automatically when display is off" enabled

#### Option 4: Run on Wake (Our Implementation)

Configure launchd to run when system wakes, if it missed the scheduled time (this is what we'll implement).

## Launchd Configuration

### Main AOC Solver Launch Agent

Create file: `~/Library/LaunchAgents/com.aoc2025.solver.plist`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <!-- Job label (unique identifier) -->
    <key>Label</key>
    <string>com.aoc2025.solver</string>

    <!-- Program to run -->
    <key>ProgramArguments</key>
    <array>
        <string>/Users/magnusrodseth/dev/personal/aoc-2025/scripts/run-solver.sh</string>
    </array>

    <!-- Run at specific time: 12:02 AM EST daily -->
    <!-- EST is UTC-5, so 12:02 AM EST = 5:02 AM UTC -->
    <key>StartCalendarInterval</key>
    <dict>
        <key>Minute</key>
        <integer>2</integer>
        <key>Hour</key>
        <integer>5</integer>
        <!-- 5:02 AM UTC = 12:02 AM EST -->
    </dict>

    <!-- Run on load (for testing) -->
    <key>RunAtLoad</key>
    <false/>

    <!-- Standard output log -->
    <key>StandardOutPath</key>
    <string>/Users/magnusrodseth/dev/personal/aoc-2025/logs/launchd-stdout.log</string>

    <!-- Standard error log -->
    <key>StandardErrorPath</key>
    <string>/Users/magnusrodseth/dev/personal/aoc-2025/logs/launchd-stderr.log</string>

    <!-- Working directory -->
    <key>WorkingDirectory</key>
    <string>/Users/magnusrodseth/dev/personal/aoc-2025</string>

    <!-- Environment variables -->
    <key>EnvironmentVariables</key>
    <dict>
        <key>PATH</key>
        <string>/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:/Users/magnusrodseth/.cargo/bin</string>
    </dict>

    <!-- Keep alive - run if crashed -->
    <key>KeepAlive</key>
    <false/>

    <!-- Only run during December -->
    <!-- Handled by the script itself -->
</dict>
</plist>
```

### Optional: Caffeinate During AoC Times

Prevent sleep during puzzle unlock window.

Create file: `~/Library/LaunchAgents/com.aoc2025.caffeinate.plist`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.aoc2025.caffeinate</string>

    <key>ProgramArguments</key>
    <array>
        <string>/usr/bin/caffeinate</string>
        <string>-dims</string>  <!-- prevent display sleep, idle sleep, and disk sleep -->
        <string>-t</string>
        <string>3600</string>   <!-- 1 hour -->
    </array>

    <!-- Start at 11:57 PM EST (4:57 AM UTC) -->
    <key>StartCalendarInterval</key>
    <dict>
        <key>Minute</key>
        <integer>57</integer>
        <key>Hour</key>
        <integer>4</integer>
    </dict>

    <key>RunAtLoad</key>
    <false/>
</dict>
</plist>
```

## Installation Scripts

### Install Launchd Agent

Create: `scripts/install-launchd.sh`

```bash
#!/bin/bash

# AOC 2025 - Install launchd agent

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
PLIST_DIR="$HOME/Library/LaunchAgents"
PLIST_NAME="com.aoc2025.solver.plist"
PLIST_PATH="$PLIST_DIR/$PLIST_NAME"

echo "Installing AOC 2025 Solver launchd agent..."

# Create LaunchAgents directory if it doesn't exist
mkdir -p "$PLIST_DIR"

# Create logs directory
mkdir -p "$PROJECT_ROOT/logs"

# Generate plist file with correct paths
cat > "$PLIST_PATH" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.aoc2025.solver</string>

    <key>ProgramArguments</key>
    <array>
        <string>$PROJECT_ROOT/scripts/run-solver.sh</string>
    </array>

    <key>StartCalendarInterval</key>
    <dict>
        <key>Minute</key>
        <integer>5</integer>
        <key>Hour</key>
        <integer>5</integer>
    </dict>

    <key>RunAtLoad</key>
    <false/>

    <key>StandardOutPath</key>
    <string>$PROJECT_ROOT/logs/launchd-stdout.log</string>

    <key>StandardErrorPath</key>
    <string>$PROJECT_ROOT/logs/launchd-stderr.log</string>

    <key>WorkingDirectory</key>
    <string>$PROJECT_ROOT</string>

    <key>EnvironmentVariables</key>
    <dict>
        <key>PATH</key>
        <string>/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:$HOME/.cargo/bin</string>
    </dict>

    <key>KeepAlive</key>
    <false/>
</dict>
</plist>
EOF

echo "Created plist file at: $PLIST_PATH"

# Load the agent
launchctl unload "$PLIST_PATH" 2>/dev/null || true
launchctl load "$PLIST_PATH"

echo "✅ Launchd agent installed and loaded!"
echo ""
echo "Status:"
launchctl list | grep com.aoc2025 || echo "Agent not found in list"

echo ""
echo "Logs will be written to:"
echo "  - $PROJECT_ROOT/logs/launchd-stdout.log"
echo "  - $PROJECT_ROOT/logs/launchd-stderr.log"
echo ""
echo "To manually trigger:"
echo "  launchctl start com.aoc2025.solver"
echo ""
echo "To uninstall:"
echo "  ./scripts/uninstall-launchd.sh"
```

### Uninstall Launchd Agent

Create: `scripts/uninstall-launchd.sh`

```bash
#!/bin/bash

# AOC 2025 - Uninstall launchd agent

PLIST_DIR="$HOME/Library/LaunchAgents"
PLIST_NAME="com.aoc2025.solver.plist"
PLIST_PATH="$PLIST_DIR/$PLIST_NAME"

echo "Uninstalling AOC 2025 Solver launchd agent..."

if [ -f "$PLIST_PATH" ]; then
    launchctl unload "$PLIST_PATH" 2>/dev/null || true
    rm "$PLIST_PATH"
    echo "✅ Launchd agent uninstalled!"
else
    echo "❌ Plist file not found at: $PLIST_PATH"
    exit 1
fi
```

### Main Solver Runner Script

Create: `scripts/run-solver.sh`

```bash
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

# Check if we're in December
CURRENT_MONTH=$(date +"%m")
CURRENT_YEAR=$(date +"%Y")

if [ "$CURRENT_YEAR" != "2025" ] || [ "$CURRENT_MONTH" != "12" ]; then
    log "Not December 2025, skipping execution"
    exit 0
fi

# Determine current day (in EST timezone)
# Convert current time to EST
export TZ="America/New_York"
CURRENT_DAY=$(date +"%d" | sed 's/^0*//')  # Remove leading zero

log "Current day (EST): $CURRENT_DAY"

# Only run for days 1-12 (AoC 2025 is 12 days)
if [ "$CURRENT_DAY" -lt 1 ] || [ "$CURRENT_DAY" -gt 12 ]; then
    log "Day $CURRENT_DAY is outside AoC 2025 range (1-12), skipping"
    exit 0
fi

# Check if already solved
STATE_FILE="$PROJECT_ROOT/state/day${CURRENT_DAY}.json"
if [ -f "$STATE_FILE" ]; then
    STATUS=$(jq -r '.status' "$STATE_FILE" 2>/dev/null || echo "unknown")
    if [ "$STATUS" = "completed" ]; then
        log "Day $CURRENT_DAY already completed, skipping"
        exit 0
    fi
fi

log "Running solver for Day $CURRENT_DAY..."

# Ensure cargo is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Run the orchestrator
# TODO: Replace with actual orchestrator binary when implemented
log "Running: cargo run --bin aoc-orchestrator -- --day $CURRENT_DAY"

if cargo run --bin aoc-orchestrator -- --day "$CURRENT_DAY" >> "$LOG_FILE" 2>&1; then
    log "✅ Solver completed successfully for Day $CURRENT_DAY"
else
    log "❌ Solver failed for Day $CURRENT_DAY"
    exit 1
fi

log "========================================"
log "AOC 2025 Solver - Finished"
log "========================================"
```

## Setup Instructions

### 1. Install Dependencies

```bash
# Install aoc-cli (if not already installed)
cargo install aoc-cli

# Ensure session cookie is configured
echo "your_session_cookie_here" > ~/.adventofcode.session

# OR set environment variable
export ADVENT_OF_CODE_SESSION="your_session_cookie_here"
```

### 2. Make Scripts Executable

```bash
cd ~/dev/personal/aoc-2025
chmod +x scripts/*.sh
```

### 3. Install Launchd Agent

```bash
./scripts/install-launchd.sh
```

### 4. Test the Setup

```bash
# Manually trigger the job
launchctl start com.aoc2025.solver

# Check logs
tail -f logs/launchd-stdout.log
tail -f logs/launchd-stderr.log
```

### 5. Verify Scheduling

```bash
# Check if agent is loaded
launchctl list | grep com.aoc2025

# View agent status
launchctl print gui/$(id -u)/com.aoc2025.solver
```

## Handling System Sleep

### Recommended Workflow During AoC

For December 1-12, 2025:

#### Option A: Keep System Awake During Unlock Times

```bash
# Run this before bed (or via another launchd job)
caffeinate -dims -t 3600 &

# This keeps system awake for 1 hour
# Schedule it to run at 11:55 PM EST (4:55 AM UTC)
```

#### Option B: Run on Wake

The launchd agent will automatically run if you wake the system after the scheduled time and it hasn't run yet that day.

#### Option C: Manual Trigger

If you miss the automated run:

```bash
# Check what day it is
date

# Manually run for specific day
./scripts/run-solver.sh
```

## Troubleshooting

### Agent Not Running

```bash
# Check if loaded
launchctl list | grep com.aoc2025

# If not listed, reload
launchctl load ~/Library/LaunchAgents/com.aoc2025.solver.plist

# Check for errors
cat ~/Library/LaunchAgents/com.aoc2025.solver.plist
```

### Logs Not Appearing

```bash
# Ensure log directory exists
mkdir -p logs

# Check permissions
ls -la logs/

# Manually run script to test
./scripts/run-solver.sh
```

### Wrong Timezone

```bash
# Verify EST time
export TZ="America/New_York"
date

# Update plist if needed to match your timezone offset
```

### Script Not Executing

```bash
# Check if script is executable
ls -la scripts/run-solver.sh

# Make executable
chmod +x scripts/run-solver.sh

# Test directly
./scripts/run-solver.sh
```

## Monitoring

### Check Recent Executions

```bash
# View latest log
ls -lt logs/*.log | head -1

# Tail the latest log
tail -f $(ls -t logs/solver_*.log | head -1)
```

### Monitor Launchd Status

```bash
# View agent details
launchctl print gui/$(id -u)/com.aoc2025.solver

# Check recent runs
log show --predicate 'subsystem == "com.apple.launchd"' --last 1h | grep aoc2025
```

## Uninstallation

```bash
# Run uninstall script
./scripts/uninstall-launchd.sh

# Verify removal
launchctl list | grep com.aoc2025
# Should return nothing
```

## Alternative: Using Cron (Not Recommended)

If you prefer cron despite limitations:

```bash
# Edit crontab
crontab -e

# Add entry (runs at 12:02 AM EST daily)
# NOTE: EST is typically 5 hours behind UTC
2 5 * 12 * cd /Users/magnusrodseth/dev/personal/aoc-2025 && ./scripts/run-solver.sh

# Cron limitations:
# - Won't wake system from sleep
# - Less reliable on modern macOS
# - No built-in logging
```

## Summary

- **launchd** is the recommended scheduling method on macOS
- System **will not wake from sleep** for custom tasks
- Use **caffeinate** or keep system awake during puzzle unlock times
- Agent will run on wake if it missed scheduled time
- Comprehensive logging for debugging
- Easy install/uninstall scripts provided

## December 2025 Schedule

Puzzles unlock at **12:00 AM EST** (5:00 AM UTC) on:
- December 1-12, 2025

Launchd configured to run at **12:02 AM EST** (5:02 AM UTC) to allow puzzle unlock.
