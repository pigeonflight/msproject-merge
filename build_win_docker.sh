#!/bin/bash
set -e

echo "Building Windows executable using Docker..."

# Get version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

# Build the Docker image
docker build -f Dockerfile.windows -t msproject-merge-windows-builder .

# Run the build
docker run --rm -v "$(pwd)":/app msproject-merge-windows-builder

# Rename the output with version
mv target/x86_64-pc-windows-gnu/release/msproject-merge.exe "target/x86_64-pc-windows-gnu/release/MsProjectMerger-v${VERSION}.exe"

echo "Build complete! Windows executable created at: target/x86_64-pc-windows-gnu/release/MsProjectMerger-v${VERSION}.exe"
