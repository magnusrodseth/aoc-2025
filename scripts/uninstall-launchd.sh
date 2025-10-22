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
