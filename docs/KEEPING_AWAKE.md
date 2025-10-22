# Keeping Your Mac Awake for AoC 2025

## The Problem

macOS will put your Mac to sleep when the lid is closed or after a period of inactivity. When asleep:
- **Launchd agents won't run**
- **Scheduled tasks are skipped**
- **Your AoC solver won't execute at 12:02 AM**

This guide provides multiple solutions to ensure your Mac stays awake during the critical puzzle unlock window.

---

## Solution 1: Automated Caffeinate (Recommended) ⭐

This is the **best solution** for hands-off automation. It uses launchd to automatically keep your Mac awake just before puzzle unlock.

### How It Works

```
11:57 PM EST → Caffeinate starts (keeps system awake for 2 hours)
12:00 AM EST → Puzzle unlocks
12:02 AM EST → Solver runs
1:57 AM EST  → Caffeinate ends, system can sleep
```

**Why 2 hours?**
- Gives plenty of buffer for solver execution
- Handles multiple retry attempts if needed
- Ensures completion even for complex puzzles

### Installation

```bash
# Install the caffeinate launchd agent
./scripts/install-caffeinate.sh
```

**Output:**
```
Installing AOC 2025 Caffeinate launchd agent...
✓ LaunchAgents directory ready
Created plist file at: ~/Library/LaunchAgents/com.aoc2025.caffeinate.plist
✅ Caffeinate agent installed and loaded!

Schedule:
  - Runs daily at 11:57 PM EST (3 minutes before puzzle unlock)
  - Keeps system awake for 2 hours (until 1:57 AM)
  - Ensures solver can run at 12:02 AM with plenty of buffer for retries
```

### Verify Installation

```bash
# Check if agent is loaded
launchctl list | grep com.aoc2025.caffeinate

# Expected output:
-   0   com.aoc2025.caffeinate
```

### Test It

```bash
# Manually trigger (for testing)
launchctl start com.aoc2025.caffeinate

# Check logs
tail -f logs/caffeinate-stdout.log
```

**Example log output:**
```
[2025-12-01 23:57:00] ========================================
[2025-12-01 23:57:00] AOC 2025 Caffeinate - Starting
[2025-12-01 23:57:00] ========================================
[2025-12-01 23:57:00] ✅ Within AoC 2025 active period: Day 1 of 12
[2025-12-01 23:57:00] Keeping system awake for 2 hours...
[2025-12-01 23:57:00] Caffeinate started (PID: 12345)
[2025-12-01 23:57:00] System will stay awake until 2025-12-02 01:57:00
```

### What Caffeinate Does

The `-dims` flags prevent:
- **-d**: Display from sleeping
- **-i**: System from idle sleeping
- **-m**: Disk from idle sleeping
- **-s**: System from sleeping (when plugged in)

### Uninstall

```bash
./scripts/uninstall-caffeinate.sh
```

---

## Solution 2: System Preferences

### Option A: Prevent Sleep When Display Is Off

Good for: Keeping Mac running while lid is closed (requires AC power)

**Steps:**
1. Open **System Settings** (or System Preferences)
2. Go to **Battery** > **Power Adapter**
3. Enable: **"Prevent your Mac from automatically sleeping when the display is off"**

**Pros:**
- Simple, no scripts needed
- Works indefinitely

**Cons:**
- Only works when plugged into power
- Mac stays awake ALL the time (wastes energy)
- Need to manually disable after AoC ends

### Option B: Increase Sleep Timer

**Steps:**
1. Open **System Settings** > **Lock Screen**
2. Set **"Turn display off when inactive"** to: Never (or very long time)
3. Set **"Require password after screen saver begins"** as desired

**Cons:**
- Still may sleep eventually
- Not reliable for overnight scheduling

---

## Solution 3: Clamshell Mode with External Display

Good for: Desktop-style setup during AoC

**Requirements:**
- External monitor
- Keyboard and mouse
- Mac plugged into power

**Steps:**
1. Connect external display, keyboard, and mouse
2. Plug Mac into power
3. Close the lid
4. Mac will run in "clamshell mode" using external display

**Pros:**
- Very reliable
- Can work normally while solver runs
- Mac won't sleep

**Cons:**
- Requires external display
- Less portable

---

## Solution 4: Keep Lid Open

Good for: Simple, guaranteed solution

**Steps:**
1. Leave MacBook lid open overnight
2. Optionally turn brightness to minimum

**Pros:**
- 100% reliable
- No configuration needed

**Cons:**
- Less secure (display visible)
- Screen stays on

---

## Solution 5: Manual Caffeinate

Good for: Testing or one-off nights

### Quick Command

```bash
# Keep awake for 2 hours (just run before bed)
caffeinate -dims -t 7200
```

### During AoC Window

```bash
# Run at 11:55 PM, keeps awake until 12:30 AM
caffeinate -dims -t 2100
```

**Flags:**
- `-d` = prevent display sleep
- `-i` = prevent idle sleep
- `-m` = prevent disk sleep
- `-s` = prevent system sleep (requires AC power)
- `-t 2100` = timeout after 2100 seconds (35 minutes)

**To stop manually:**
```bash
# Press Ctrl+C to stop caffeinate
```

---

## Solution 6: Third-Party Apps

### Amphetamine (Free, App Store)

1. Install [Amphetamine](https://apps.apple.com/us/app/amphetamine/id937984704) from App Store
2. Create a schedule:
   - Time: 11:55 PM - 12:30 AM
   - Days: December 1-12
   - Keep awake: Yes

**Pros:**
- GUI interface
- Reliable scheduling
- Can set specific dates

**Cons:**
- Requires separate app
- Not scriptable

---

## Recommended Setup for AoC 2025

### Best: Automated Caffeinate + Power

1. **Install caffeinate agent:**
   ```bash
   ./scripts/install-caffeinate.sh
   ```

2. **Keep Mac plugged in** during December 1-12

3. **Optionally**: Close lid with external display (clamshell mode)

### Alternative: Simple and Reliable

1. **Keep MacBook lid open** during puzzle unlock times
2. **Turn brightness to minimum**
3. **Plug into power**

---

## Monitoring

### Check if Caffeinate is Running

```bash
# See if caffeinate process is active
ps aux | grep caffeinate

# Example output:
magnusrodseth  12345  0.0  0.0  caffeinate -dims -t 1800
```

### View Logs

```bash
# Caffeinate logs
tail -f logs/caffeinate-stdout.log

# Solver logs
tail -f logs/launchd-stdout.log

# Latest execution
ls -lt logs/solver_*.log | head -1
```

---

## Troubleshooting

### Agent Installed but Mac Still Sleeps

**Check:**
1. Is Mac plugged into power? (Required for `-s` flag)
2. Are battery settings overriding caffeinate?
3. Is the agent actually running at the scheduled time?

**Debug:**
```bash
# Check agent status
launchctl list | grep caffeinate

# Check logs
cat logs/caffeinate-stdout.log

# Test manually
launchctl start com.aoc2025.caffeinate
```

### Caffeinate Not Preventing Sleep

**Solutions:**
1. Ensure Mac is plugged into AC power
2. Check battery settings aren't forcing sleep
3. Use clamshell mode as backup

### Want to Disable During Non-AoC Days

Don't worry! The caffeinate script has built-in date validation:
- Only runs during December 1-12, 2025
- Automatically skips all other days
- No need to disable/enable manually

---

## Power Consumption

### Caffeinate Power Usage

Running caffeinate for 2 hours per night:
- **Reasonable impact** on battery/power
- Only 2 hours/day × 12 days = 24 hours total
- Much better than keeping awake 24/7
- Ensures reliable completion of all retries

### Reduce Power While Awake

```bash
# Lower screen brightness to minimum
# Close unnecessary apps
# caffeinate will keep system alive but idle
```

---

## Security Considerations

### If Leaving Lid Open

1. Set screen saver with password requirement
2. Lock screen before bed: `Cmd + Ctrl + Q`
3. Or just let it auto-lock after a few minutes

### With Caffeinate Agent

- Screen can still lock while system stays awake
- Configure lock screen timeout in System Settings

---

## Summary: Quick Decision Guide

**Best for most users:**
- ✅ Install automated caffeinate agent
- ✅ Keep Mac plugged in at night

**If caffeinate doesn't work for you:**
- ✅ Keep lid open, brightness low
- ✅ Or use clamshell mode with external display

**For maximum reliability:**
- ✅ Automated caffeinate + lid open + plugged in

---

## Installation Commands

```bash
# Install everything
./scripts/install-launchd.sh      # Main solver
./scripts/install-caffeinate.sh   # Keep awake

# Verify
launchctl list | grep com.aoc2025

# Uninstall if needed
./scripts/uninstall-caffeinate.sh
./scripts/uninstall-launchd.sh
```

---

**You're all set! Your Mac will stay awake for AoC 2025!** 💤➡️☕️
