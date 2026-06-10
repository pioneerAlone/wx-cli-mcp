#!/usr/bin/env node
'use strict';

const { execFileSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const PLATFORM_PACKAGES = {
  'darwin-arm64': '@bakewell/wx-mcp-darwin-arm64',
  'darwin-x64':   '@bakewell/wx-mcp-darwin-x64',
  'linux-x64':    '@bakewell/wx-mcp-linux-x64',
  'linux-arm64':  '@bakewell/wx-mcp-linux-arm64',
  'win32-x64':    '@bakewell/wx-mcp-win32-x64',
};

const platformKey = `${process.platform}-${process.arch}`;
const ext = process.platform === 'win32' ? '.exe' : '';

function getBinaryPath() {
  if (process.env.WX_MCP_BINARY) {
    return process.env.WX_MCP_BINARY;
  }

  const pkg = PLATFORM_PACKAGES[platformKey];
  if (!pkg) {
    console.error(`wx-mcp: unsupported platform ${platformKey}`);
    process.exit(1);
  }

  try {
    return require.resolve(`${pkg}/bin/wx-mcp${ext}`);
  } catch {
    const modPath = path.join(
      path.dirname(require.resolve(`${pkg}/package.json`)),
      `bin/wx-mcp${ext}`
    );
    if (fs.existsSync(modPath)) return modPath;
  }

  console.error(`wx-mcp: binary not found for ${platformKey}`);
  console.error('Try: npm install -g @bakewell/wx-mcp');
  process.exit(1);
}

try {
  execFileSync(getBinaryPath(), process.argv.slice(2), {
    stdio: 'inherit',
    env: { ...process.env },
  });
} catch (e) {
  if (e && e.status != null) process.exit(e.status);
  throw e;
}
