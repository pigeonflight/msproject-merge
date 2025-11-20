#!/bin/bash
# Simple script to test Windows build on macOS using Wine

set -e

echo "Testing Windows build with Wine..."

# Check if Wine is installed
if ! command -v wine &> /dev/null; then
    echo "Wine is not installed. Install it with:"
    echo "  brew install --cask wine-stable"
    echo ""
    echo "Note: Wine requires Rosetta 2 on Apple Silicon Macs:"
    echo "  softwareupdate --install-rosetta --agree-to-license"
    exit 1
fi

# Check if Windows build exists
EXE_PATH="target/x86_64-pc-windows-gnu/release/MsProjectMerge-v1.0.0.exe"
if [ ! -f "$EXE_PATH" ]; then
    echo "Windows build not found. Build it first with:"
    echo "  ./build_win_docker.sh"
    exit 1
fi

echo "Launching Windows build..."
wine "$EXE_PATH"
