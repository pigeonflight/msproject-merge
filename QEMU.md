# Running Windows with QEMU

This guide helps you set up a Windows VM using QEMU to test the Windows build of MS Project Merge.

## Prerequisites

QEMU is already installed via Homebrew.

## Setup Windows VM

### 1. Download Windows 11 ISO

Download the Windows 11 ISO from Microsoft:
https://www.microsoft.com/software-download/windows11

Save it to a location outside the git repository (e.g., `~/Downloads/`).

### 2. Create a Virtual Disk

```bash
# Create a directory for VM files (gitignored)
mkdir -p vm

# Create a 50GB virtual disk
qemu-img create -f qcow2 vm/windows11.qcow2 50G
```

### 3. Install Windows

```bash
# Start the VM with the Windows ISO
qemu-system-x86_64 \
  -m 4G \
  -smp 2 \
  -cpu host \
  -accel hvf \
  -drive file=vm/windows11.qcow2,if=virtio \
  -cdrom ~/Downloads/Win11_English_x64.iso \
  -boot d \
  -device virtio-net,netdev=net0 \
  -netdev user,id=net0 \
  -display default
```

Follow the Windows installation wizard. This will take 15-30 minutes.

### 4. Run Windows VM (After Installation)

```bash
# Start the installed Windows VM
qemu-system-x86_64 \
  -m 4G \
  -smp 2 \
  -cpu host \
  -accel hvf \
  -drive file=vm/windows11.qcow2,if=virtio \
  -device virtio-net,netdev=net0 \
  -netdev user,id=net0,hostfwd=tcp::5555-:3389 \
  -display default
```

### 5. Transfer Files to VM

**Option A: Shared Folder (Recommended)**
```bash
# Start VM with shared folder
qemu-system-x86_64 \
  -m 4G \
  -smp 2 \
  -cpu host \
  -accel hvf \
  -drive file=vm/windows11.qcow2,if=virtio \
  -device virtio-net,netdev=net0 \
  -netdev user,id=net0,smb=$(pwd)/target/x86_64-pc-windows-gnu/release \
  -display default
```

In Windows, access the shared folder via: `\\10.0.2.4\qemu`

**Option B: HTTP Server**
```bash
# In a separate terminal, start a simple HTTP server
cd target/x86_64-pc-windows-gnu/release
python3 -m http.server 8000
```

In the Windows VM browser, go to: `http://10.0.2.2:8000/`

### 6. Test the Windows Build

1. Copy `MsProjectMerge-v1.0.0.exe` to the Windows VM
2. Right-click → "Run as administrator" (if needed)
3. Test the application

## Useful QEMU Commands

### Create Snapshot
```bash
qemu-img snapshot -c clean_install vm/windows11.qcow2
```

### List Snapshots
```bash
qemu-img snapshot -l vm/windows11.qcow2
```

### Restore Snapshot
```bash
qemu-img snapshot -a clean_install vm/windows11.qcow2
```

### Check Disk Usage
```bash
qemu-img info vm/windows11.qcow2
```

## Performance Tips

- **Use HVF acceleration**: `-accel hvf` (macOS Hypervisor Framework)
- **Allocate more RAM**: `-m 8G` (if you have enough RAM)
- **More CPU cores**: `-smp 4` (if you have a multi-core CPU)
- **Enable KVM** (on Linux): `-accel kvm`

## Troubleshooting

### VM is slow
- Increase RAM: `-m 8G`
- Increase CPU cores: `-smp 4`
- Make sure HVF is enabled: `-accel hvf`

### Can't access shared folder
- Use HTTP server method instead
- Or use SCP/SFTP if you enable SSH in Windows

### Display issues
- Try different display backends: `-display cocoa` or `-display sdl`

## Alternative: Use UTM (GUI for QEMU)

If you prefer a GUI, install UTM (free, open-source):
```bash
brew install --cask utm
```

UTM provides a user-friendly interface for QEMU on macOS.

## Notes

- All VM files are stored in the `vm/` directory (gitignored)
- The Windows ISO should be kept outside the repository
- QCOW2 format supports snapshots and compression
- The VM will use about 20-30GB of disk space after Windows installation
