#!/usr/bin/env bun

/**
 * AOC 2025 - Main Autonomous Agent Orchestrator
 *
 * This script uses the Claude Agent SDK to autonomously:
 * 1. Fetch daily puzzles
 * 2. Solve them using TDD
 * 3. Submit answers
 * 4. Handle retries and failures
 */

import { query } from '@anthropic-ai/claude-agent-sdk';
import { getAgents, orchestratorConfig } from './agents';
import {
  AOC_YEAR,
  PROJECT_ROOT,
  LOGS_DIR,
  STATE_DIR,
  getCurrentDayEST,
  getCurrentMonthEST,
  getCurrentYearEST,
  isAoC2025Active,
  loadDayState,
  saveDayState,
  checkSessionCookie,
  type DayState
} from './config';
import path from 'path';
import { mkdir } from 'fs/promises';

interface CLIArgs {
  day?: number;
  dryRun: boolean;
  debug: boolean;
  force: boolean;
}

/**
 * Parse command line arguments
 */
function parseArgs(): CLIArgs {
  const args = process.argv.slice(2);
  const result: CLIArgs = {
    dryRun: false,
    debug: false,
    force: false
  };

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (arg === '--day' && args[i + 1]) {
      result.day = parseInt(args[i + 1]);
      i++;
    } else if (arg === '--dry-run') {
      result.dryRun = true;
    } else if (arg === '--debug') {
      result.debug = true;
    } else if (arg === '--force') {
      result.force = true;
    } else if (!arg.startsWith('--')) {
      // First non-flag argument is the day
      result.day = parseInt(arg);
    }
  }

  return result;
}

/**
 * Setup logging for a specific day
 */
async function setupLogging(day: number): Promise<string> {
  await mkdir(LOGS_DIR, { recursive: true });
  const timestamp = new Date().toISOString().replace(/:/g, '-').split('.')[0];
  const logFile = path.join(LOGS_DIR, `day${day}_${timestamp}.log`);
  return logFile;
}

/**
 * Initialize state for a day
 */
async function initializeDayState(day: number): Promise<DayState> {
  await mkdir(STATE_DIR, { recursive: true });

  const existingState = await loadDayState(day);
  if (existingState) {
    return existingState;
  }

  const newState: DayState = {
    day,
    year: AOC_YEAR,
    status: 'pending',
    started_at: new Date().toISOString()
  };

  await saveDayState(day, newState);
  return newState;
}

/**
 * Validate environment and prerequisites
 */
function validateEnvironment(): { valid: boolean; errors: string[] } {
  const errors: string[] = [];

  // Check for Anthropic API key
  if (!process.env.ANTHROPIC_API_KEY) {
    errors.push('ANTHROPIC_API_KEY environment variable is not set');
    errors.push('Get your API key from: https://console.anthropic.com/');
    errors.push('Set it with: export ANTHROPIC_API_KEY="your-key-here"');
  }

  return {
    valid: errors.length === 0,
    errors
  };
}

/**
 * Main orchestrator function
 */
async function main() {
  const args = parseArgs();

  console.log('========================================');
  console.log('AOC 2025 Autonomous Agent');
  console.log('========================================');
  console.log('');

  // Validate environment
  const envCheck = validateEnvironment();
  if (!envCheck.valid) {
    console.error('❌ Environment validation failed:\n');
    envCheck.errors.forEach(err => console.error(`  ${err}`));
    console.error('');
    process.exit(1);
  }

  // Determine which day to solve
  let day: number;
  if (args.day) {
    day = args.day;
    console.log(`Using specified day: ${day}`);

    // Warn if running outside AoC period for manual day
    if (!isAoC2025Active()) {
      const year = getCurrentYearEST();
      const month = getCurrentMonthEST();
      const currentDay = getCurrentDayEST();

      console.log('');
      console.log('⚠️  Note: Not in AoC 2025 active period');
      console.log(`   Current date (EST): ${year}-${month.toString().padStart(2, '0')}-${currentDay.toString().padStart(2, '0')}`);
      console.log(`   AoC 2025 runs: December 1-12, 2025`);
      console.log('   Continuing with manual day specification...');
      console.log('');
    }
  } else {
    // Auto-detect mode: strict date validation
    if (!isAoC2025Active()) {
      const year = getCurrentYearEST();
      const month = getCurrentMonthEST();
      const currentDay = getCurrentDayEST();

      console.log('Not in AoC 2025 active period');
      console.log(`Current date (EST): ${year}-${month.toString().padStart(2, '0')}-${currentDay.toString().padStart(2, '0')}`);
      console.log('AoC 2025 runs: December 1-12, 2025');
      console.log('');
      console.log('Use --day to solve a specific day manually');
      process.exit(0);
    }
    day = getCurrentDayEST();
    console.log(`Using current EST day: ${day}`);
  }

  // Validate day
  if (day < 1 || day > 25) {
    console.error(`Invalid day: ${day}. Must be between 1 and 25.`);
    process.exit(1);
  }

  // Check session cookie
  const hasSession = await checkSessionCookie();
  if (!hasSession) {
    console.error('Session cookie not found or empty.');
    console.error('Please add your session cookie to: ~/.adventofcode.session');
    console.error('Get it from: https://adventofcode.com (login and copy session cookie)');
    process.exit(1);
  }

  // Setup logging
  const logFile = await setupLogging(day);
  console.log(`Logging to: ${logFile}`);

  // Initialize state
  const state = await initializeDayState(day);

  // Check if already completed
  if (state.status === 'completed' && !args.force) {
    console.log(`Day ${day} already completed!`);
    console.log(`Part 1: ${state.part1?.answer} (${state.part1?.attempts} attempts)`);
    console.log(`Part 2: ${state.part2?.answer} (${state.part2?.attempts} attempts)`);
    console.log('Use --force to re-run');
    process.exit(0);
  }

  if (args.force) {
    console.log('Force mode: Re-running day...');
    state.status = 'in_progress';
    await saveDayState(day, state);
  }

  console.log('Starting autonomous solving workflow...');
  console.log('');

  // Build the prompt for the orchestrator
  const prompt = `You are the AOC 2025 autonomous orchestrator.

Your mission: Solve Advent of Code Day ${day} completely autonomously.

Current state:
${JSON.stringify(state, null, 2)}

Working directory: ${PROJECT_ROOT}
Dry run mode: ${args.dryRun}
Debug mode: ${args.debug}

Your workflow:
1. Use the Task tool to invoke the 'puzzle-fetcher' agent to download and parse the puzzle
2. Use the Task tool to invoke the 'tdd-solver' agent to implement Part 1 solution
3. Use the Task tool to invoke the 'submission-handler' agent to submit Part 1
4. If Part 1 succeeds, repeat steps 2-3 for Part 2
5. Update the state file at state/day${day}.json after each step
6. Log all progress to ${logFile}

${args.dryRun ? '⚠️  DRY RUN MODE: Do not actually submit answers, just log what would be submitted.' : ''}

Important:
- You have full autonomy to run commands, edit files, and write Rust code
- Follow TDD strictly when implementing solutions
- Handle failures intelligently with retry logic
- Update state file after each major step
- Be methodical and thorough

Begin!`;

  try {
    // Run the orchestrator with full autonomy
    const result = query({
      prompt,
      options: {
        cwd: PROJECT_ROOT,
        // Use Claude Code's system prompt + our orchestrator instructions
        systemPrompt: {
          type: 'preset',
          preset: 'claude_code',
          append: orchestratorConfig.systemPrompt
        },
        // Load project settings (includes CLAUDE.md)
        settingSources: ['project', 'local'],
        // Define the subagents
        agents: getAgents(),
        // Full autonomy mode
        permissionMode: 'bypassPermissions',
        // Tools available to orchestrator
        allowedTools: [
          'Task',
          'Bash',
          'Read',
          'Write',
          'Edit',
          'Glob',
          'Grep',
          'TodoWrite'
        ],
        model: 'claude-sonnet-4-5-20250929',
        // Enable debug output if requested
        includePartialMessages: args.debug,
        // Log stderr
        stderr: (data) => {
          if (args.debug) {
            console.error('[STDERR]', data);
          }
        }
      }
    });

    // Stream the results
    let finalResult = '';
    for await (const message of result) {
      if (message.type === 'assistant') {
        // Print assistant messages
        for (const block of message.message.content) {
          if (block.type === 'text') {
            console.log(block.text);
            finalResult += block.text + '\\n';
          }
        }
      } else if (message.type === 'result') {
        // Final result
        console.log('');
        console.log('========================================');
        console.log('Orchestration Complete');
        console.log('========================================');
        console.log(`Status: ${message.subtype}`);
        console.log(`Turns: ${message.num_turns}`);
        console.log(`Duration: ${(message.duration_ms / 1000).toFixed(2)}s`);
        console.log(`Cost: $${message.total_cost_usd.toFixed(4)}`);
        console.log('');

        if (message.subtype === 'success') {
          console.log('✅ Success!');
          console.log(message.result);

          // Update state to completed
          const finalState = await loadDayState(day);
          if (finalState) {
            finalState.status = 'completed';
            finalState.completed_at = new Date().toISOString();
            await saveDayState(day, finalState);
          }
        } else {
          console.error('❌ Failed');
          if (message.subtype === 'error_max_turns') {
            console.error('Exceeded maximum turns');
          }

          // Update state to failed
          const finalState = await loadDayState(day);
          if (finalState) {
            finalState.status = 'failed';
            finalState.error = message.subtype;
            await saveDayState(day, finalState);
          }
        }

        // Write final log
        await Bun.write(logFile, finalResult);
      }
    }

    console.log(`Full log written to: ${logFile}`);
  } catch (error) {
    console.error('Fatal error:', error);

    // Update state to failed
    const errorState = await loadDayState(day);
    if (errorState) {
      errorState.status = 'failed';
      errorState.error = error instanceof Error ? error.message : String(error);
      await saveDayState(day, errorState);
    }

    process.exit(1);
  }
}

// Run if this is the main module
if (import.meta.main) {
  main().catch((error) => {
    console.error('Unhandled error:', error);
    process.exit(1);
  });
}

export { main };
