#!/usr/bin/env bash
set -euo pipefail

# Build a universal macOS DMG that supports both Apple Silicon and Intel Macs.
# Run this from the repository root on macOS.

rustup target add aarch64-apple-darwin x86_64-apple-darwin
pnpm install
pnpm tauri build -- --target universal-apple-darwin --bundles dmg
