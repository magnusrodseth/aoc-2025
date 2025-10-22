#!/usr/bin/env bun

/**
 * Check if AoC session cookie and Anthropic API key are properly configured
 */

import { AOC_SESSION_FILE, checkSessionCookie } from './config';

async function main() {
  console.log('========================================');
  console.log('AOC 2025 Setup Validation');
  console.log('========================================');
  console.log('');

  let hasErrors = false;

  // Check Anthropic API Key
  console.log('1. Checking Anthropic API Key...');
  const apiKey = process.env.ANTHROPIC_API_KEY;
  if (apiKey) {
    console.log('   ✅ ANTHROPIC_API_KEY is set');
  } else {
    console.error('   ❌ ANTHROPIC_API_KEY is not set');
    console.error('');
    console.error('   To get your API key:');
    console.error('   1. Go to https://console.anthropic.com/');
    console.error('   2. Sign in and navigate to API Keys');
    console.error('   3. Create a new key or copy existing one');
    console.error('   4. Set it in your environment:');
    console.error('');
    console.error('      export ANTHROPIC_API_KEY="your-key-here"');
    console.error('');
    console.error('   Add to your ~/.bashrc or ~/.zshrc for persistence:');
    console.error(`      echo 'export ANTHROPIC_API_KEY="your-key-here"' >> ~/.bashrc`);
    console.error('');
    hasErrors = true;
  }

  console.log('');

  // Check AoC Session Cookie
  console.log('2. Checking AoC Session Cookie...');
  console.log(`   Session file: ${AOC_SESSION_FILE}`);
  const hasSession = await checkSessionCookie();

  if (hasSession) {
    console.log('   ✅ Session cookie is configured');
  } else {
    console.error('   ❌ Session cookie not found or empty');
    console.error('');
    console.error('   To get your session cookie:');
    console.error('   1. Go to https://adventofcode.com');
    console.error('   2. Log in with your account');
    console.error('   3. Open browser DevTools (F12)');
    console.error('   4. Go to Application/Storage → Cookies');
    console.error('   5. Find the "session" cookie value');
    console.error('   6. Save it to ~/.adventofcode.session');
    console.error('');
    console.error('   Example:');
    console.error(`      echo "your_session_cookie_here" > ${AOC_SESSION_FILE}`);
    console.error('');
    hasErrors = true;
  }

  console.log('');
  console.log('========================================');

  if (hasErrors) {
    console.error('❌ Setup incomplete - fix the errors above');
    console.log('');
    process.exit(1);
  } else {
    console.log('✅ All checks passed!');
    console.log('');
    console.log('You are ready to run the autonomous solver:');
    console.log('  bun run agent:run-day 1');
    console.log('');
    process.exit(0);
  }
}

if (import.meta.main) {
  main();
}
