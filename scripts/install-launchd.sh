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
echo "Creating LaunchAgents directory if needed..."
mkdir -p "$PLIST_DIR"
if [ ! -d "$PLIST_DIR" ]; then
    echo "❌ Failed to create $PLIST_DIR"
    exit 1
fi
echo "✓ LaunchAgents directory ready"

# Create logs directory
mkdir -p "$PROJECT_ROOT/logs"

# Create state directory
mkdir -p "$PROJECT_ROOT/state"

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
        <integer>2</integer>
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
