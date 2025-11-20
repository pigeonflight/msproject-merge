# Build Instructions

## Prerequisites
- Rust (latest stable)
- Docker (for Windows build)

## macOS Build
To build the macOS application and create a `.dmg` installer:

```bash
chmod +x build_mac.sh
./build_mac.sh
```

The output `.dmg` will be located in `target/release/MsProjectMerger.dmg`.

## Windows Build (via Docker)
To build the Windows `.exe` using Docker (cross-compilation):

```bash
chmod +x build_win_docker.sh
./build_win_docker.sh
```

The output `.exe` will be located in `target/x86_64-pc-windows-gnu/release/msproject-merge.exe`.
