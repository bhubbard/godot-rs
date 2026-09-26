#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const isAppleSiliconMac = process.platform === 'darwin' && process.arch === 'arm64';
const bundledBin = path.join(__dirname, 'godot-bin');
let binaryPath = (isAppleSiliconMac && fs.existsSync(bundledBin)) ? bundledBin : null;

if (!binaryPath) {
  const localRelease = path.join(__dirname, '..', 'target', 'release', 'godot');
  const localDebug = path.join(__dirname, '..', 'target', 'debug', 'godot');
  if (fs.existsSync(localRelease)) {
    binaryPath = localRelease;
  } else if (fs.existsSync(localDebug)) {
    binaryPath = localDebug;
  } else {
    binaryPath = 'godot';
  }
}

const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit'
});

child.on('error', (err) => {
  if (err.code === 'ENOENT') {
    console.error('Error: godot native binary not found.');
    console.error('Please ensure godot is installed on your PATH or build locally with: cargo build --release');
  } else {
    console.error(err);
  }
  process.exit(1);
});

child.on('exit', (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
  } else {
    process.exit(code ?? 0);
  }
});
