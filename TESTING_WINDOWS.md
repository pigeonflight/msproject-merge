# Testing Windows Builds

Since Wine has deprecation issues on macOS, here are better alternatives for testing Windows builds:

## Option 1: Download from GitHub Release (Easiest)

Your GitHub Actions automatically builds the Windows .exe. Just download it from:
https://github.com/pigeonflight/msproject-merge/releases/latest

Then test it on a real Windows machine or VM.

## Option 2: Use GitHub Codespaces (Free)

1. Go to your repository on GitHub
2. Click "Code" → "Codespaces" → "Create codespace"
3. In the codespace terminal:
   ```bash
   cargo build --release --target x86_64-pc-windows-gnu
   ```

## Option 3: QEMU Windows VM (Full Windows Environment)

See [QEMU.md](QEMU.md) for detailed instructions on setting up a Windows 11 VM.

## Option 4: Wine (Not Recommended - Deprecated)

Wine is being deprecated on macOS and has compatibility issues. Skip this option.

## Recommendation

For quick testing: **Use Option 1** (download from GitHub Release)

For development testing: **Use Option 3** (QEMU VM) - one-time setup, then you have a full Windows environment
