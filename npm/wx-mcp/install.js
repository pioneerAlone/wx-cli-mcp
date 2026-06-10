#!/usr/bin/env node
'use strict';

const fs = require('fs');

const PLATFORM_PACKAGES = {
  'darwin-arm64': '@bakewell/wx-mcp-darwin-arm64',
  'darwin-x64':   '@bakewell/wx-mcp-darwin-x64',
  'linux-x64':    '@bakewell/wx-mcp-linux-x64',
  'linux-arm64':  '@bakewell/wx-mcp-linux-arm64',
  'win32-x64':    '@bakewell/wx-mcp-win32-x64',
};

const platformKey = `${process.platform}-${process.arch}`;
const pkg = PLATFORM_PACKAGES[platformKey];

if (!pkg) {
  console.log(`wx-mcp: no binary for ${platformKey}, skipping`);
  process.exit(0);
}

const ext = process.platform === 'win32' ? '.exe' : '';

try {
  const binaryPath = require.resolve(`${pkg}/bin/wx-mcp${ext}`);
  if (process.platform !== 'win32') {
    fs.chmodSync(binaryPath, 0o755);
  }
} catch {
  console.log(`wx-mcp: platform package ${pkg} not installed`);
}
