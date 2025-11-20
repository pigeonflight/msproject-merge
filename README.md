# MS Project Merger

Smart merge tool for Microsoft Project files with WBS-based conflict resolution.

[![Build Release](https://github.com/pigeonflight/msproject-merge/actions/workflows/build.yml/badge.svg)](https://github.com/pigeonflight/msproject-merge/actions/workflows/build.yml)
[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/pigeonflight/msproject-merge/releases)

## 📥 Download

**[Download Latest Release](https://github.com/pigeonflight/msproject-merge/releases/latest)**

- **macOS**: `MsProjectMerger-v1.0.0.dmg`
- **Windows**: `MsProjectMerger-v1.0.0.exe`

## ✨ Features

### Smart Merge
- **WBS-based matching**: Uses Work Breakdown Structure codes as the primary key
- **Automatic conflict resolution**: Updates existing tasks or appends new ones
- **Multi-file support**: Merge multiple overlay files into a single base project

### File Format Support
- **MSPDI (XML)**: Full import/export support for Microsoft Project XML format
- **Excel**: Import and export task data via `.xlsx` files
- **Dependency preservation**: Maintains task predecessors and link types

### Modern UI
- **3-Step Wizard**: Select → Review → Export
- **Task Editing**: Edit any task field directly in Step 2
- **Dark Theme**: Professional dark mode with modern aesthetics
- **Status Badges**: Color-coded task status indicators

## 🚀 Quick Start

1. **Download** the installer for your platform
2. **Launch** MS Project Merger
3. **Step 1**: Add your Base Project and Overlay files
4. **Step 2**: Review merged data, edit tasks as needed
5. **Step 3**: Export to MSPDI (XML) or Excel

## 📖 How It Works

### Merge Logic

The merger uses **WBS codes** to match tasks:

1. **Base Project**: The first file you load becomes the base
2. **Overlay Files**: Additional files are merged into the base
3. **Matching**: Tasks with the same WBS code are updated
4. **Appending**: Tasks with new WBS codes are added to the end

### What Gets Updated

When a WBS match is found, these fields are updated from the overlay:
- Start/Finish Dates
- Duration
- % Complete
- Resource Names
- Notes/Description

### Dependencies

Task dependencies (predecessors) are preserved during merge and export:
- Predecessor UID
- Link Type (Finish-to-Start, Start-to-Start, etc.)
- Link Lag

## 🛠️ Building from Source

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- For macOS builds: Xcode Command Line Tools
- For Windows builds (cross-compile): Docker

### Build Commands

```bash
# Clone the repository
git clone git@github.com:pigeonflight/msproject-merge.git
cd msproject-merge

# Build and run
cargo run

# Build release
cargo build --release

# Build macOS DMG
./build_mac.sh

# Build Windows EXE (requires Docker)
./build_win_docker.sh
```

See [BUILD.md](BUILD.md) for detailed build instructions.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🔗 Links

- [Latest Release](https://github.com/pigeonflight/msproject-merge/releases/latest)
- [Build Instructions](BUILD.md)
- [Release Documentation](.github/RELEASE.md)
- [Report Issues](https://github.com/pigeonflight/msproject-merge/issues)

## 💡 Use Cases

- **Project Updates**: Merge schedule updates from different teams
- **Baseline Comparison**: Compare current project state with baseline
- **Multi-source Integration**: Combine tasks from multiple project files
- **Data Consolidation**: Merge Excel exports back into MS Project format

---

**Made with ❤️ for project managers everywhere**
