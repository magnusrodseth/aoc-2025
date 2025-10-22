/**
 * Configuration for the AOC 2025 Agent
 */

import path from 'path';

export const AOC_YEAR = 2025;
export const AOC_START_DAY = 1;
export const AOC_END_DAY = 12; // AoC 2025 only runs for 12 days

// Project paths
export const PROJECT_ROOT = path.resolve(import.meta.dir, '../..');
export const PUZZLES_DIR = path.join(PROJECT_ROOT, 'puzzles');
export const STATE_DIR = path.join(PROJECT_ROOT, 'state');
export const LOGS_DIR = path.join(PROJECT_ROOT, 'logs');
export const SRC_DAYS_DIR = path.join(PROJECT_ROOT, 'src', 'days');

// Session configuration
export const AOC_SESSION_FILE = path.join(process.env.HOME || '', '.adventofcode.session');

// Retry and timing configuration
export const MAX_RETRIES = 3;
export const INITIAL_BACKOFF_MS = 1000;
export const MAX_SUBMISSION_ATTEMPTS = 5;

// Solution timeout (seconds)
export const SOLUTION_TIMEOUT = 15;

export interface DayState {
  day: number;
  year: number;
  status: 'pending' | 'in_progress' | 'completed' | 'failed';
  part1?: {
    status: 'pending' | 'in_progress' | 'completed' | 'failed';
    answer?: number | string;
    submitted_at?: string;
    attempts: number;
  };
  part2?: {
    status: 'pending' | 'in_progress' | 'completed' | 'failed';
    answer?: number | string;
    submitted_at?: string;
    attempts: number;
  };
  started_at?: string;
  completed_at?: string;
  error?: string;
}

export interface PuzzleData {
  day: number;
  year: number;
  title: string;
  part1: {
    description: string;
    examples: Array<{
      input: string;
      expected_output: string;
      explanation?: string;
    }>;
  };
  part2?: {
    description: string;
    examples: Array<{
      input: string;
      expected_output: string;
      explanation?: string;
    }>;
  };
  input_file_path: string;
  puzzle_file_path: string;
}

/**
 * Get the current day in EST (Advent of Code timezone)
 */
export function getCurrentDayEST(): number {
  const estTime = new Date().toLocaleString('en-US', { timeZone: 'America/New_York' });
  const date = new Date(estTime);
  return date.getDate();
}

/**
 * Get the current month in EST
 */
export function getCurrentMonthEST(): number {
  const estTime = new Date().toLocaleString('en-US', { timeZone: 'America/New_York' });
  const date = new Date(estTime);
  return date.getMonth() + 1; // JavaScript months are 0-indexed
}

/**
 * Get the current year in EST
 */
export function getCurrentYearEST(): number {
  const estTime = new Date().toLocaleString('en-US', { timeZone: 'America/New_York' });
  const date = new Date(estTime);
  return date.getFullYear();
}

/**
 * Check if we're in the AoC 2025 active period
 */
export function isAoC2025Active(): boolean {
  const year = getCurrentYearEST();
  const month = getCurrentMonthEST();
  const day = getCurrentDayEST();

  if (year !== AOC_YEAR) return false;
  if (month !== 12) return false;
  if (day < AOC_START_DAY || day > AOC_END_DAY) return false;

  return true;
}

/**
 * Load state for a specific day
 */
export async function loadDayState(day: number): Promise<DayState | null> {
  const stateFile = path.join(STATE_DIR, `day${day}.json`);
  try {
    const data = await Bun.file(stateFile).text();
    return JSON.parse(data);
  } catch {
    return null;
  }
}

/**
 * Save state for a specific day
 */
export async function saveDayState(day: number, state: DayState): Promise<void> {
  const stateFile = path.join(STATE_DIR, `day${day}.json`);
  await Bun.write(stateFile, JSON.stringify(state, null, 2));
}

/**
 * Check if session cookie is configured
 */
export async function checkSessionCookie(): Promise<boolean> {
  try {
    const file = Bun.file(AOC_SESSION_FILE);
    const exists = await file.exists();
    if (!exists) return false;

    const content = await file.text();
    return content.trim().length > 0;
  } catch {
    return false;
  }
}
