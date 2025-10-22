#!/bin/bash

# AOC 2025 - Install caffeinate launchd agent

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
PLIST_DIR="$HOME/Library/LaunchAgents"
PLIST_NAME="com.aoc2025.caffeinate.plist"
PLIST_PATH="$PLIST_DIR/$PLIST_NAME"

echo "Installing AOC 2025 Caffeinate launchd agent..."

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

# Generate plist file with correct paths
cat > "$PLIST_PATH" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.aoc2025.caffeinate</string>

    <key>ProgramArguments</key>
    <array>
        <string>$PROJECT_ROOT/scripts/caffeinate-aoc.sh</string>
    </array>

    <!-- Run at 11:57 PM EST (4:57 AM UTC) to keep system awake before puzzle unlock -->
    <key>StartCalendarInterval</key>
    <dict>
        <key>Minute</key>
        <integer>57</integer>
        <key>Hour</key>
        <integer>4</integer>
    </dict>

    <key>RunAtLoad</key>
    <false/>

    <key>StandardOutPath</key>
    <string>$PROJECT_ROOT/logs/caffeinate-stdout.log</string>

    <key>StandardErrorPath</key>
    <string>$PROJECT_ROOT/logs/caffeinate-stderr.log</string>

    <key>WorkingDirectory</key>
    <string>$PROJECT_ROOT</string>

    <key>EnvironmentVariables</key>
    <dict>
        <key>PATH</key>
        <string>/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin</string>
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

echo "✅ Caffeinate agent installed and loaded!"
echo ""
echo "Status:"
launchctl list | grep com.aoc2025.caffeinate || echo "Agent not found in list"

echo ""
echo "Schedule:"
echo "  - Runs daily at 11:57 PM EST (3 minutes before puzzle unlock)"
echo "  - Keeps system awake for 2 hours (until 1:57 AM)"
echo "  - Ensures solver can run at 12:02 AM with plenty of buffer for retries"
echo ""
echo "Logs will be written to:"
echo "  - $PROJECT_ROOT/logs/caffeinate-stdout.log"
echo "  - $PROJECT_ROOT/logs/caffeinate-stderr.log"
echo ""
echo "To manually trigger (for testing):"
echo "  launchctl start com.aoc2025.caffeinate"
echo ""
echo "To uninstall:"
echo "  ./scripts/uninstall-caffeinate.sh"
