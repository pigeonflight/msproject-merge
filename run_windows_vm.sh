#!/bin/bash
# Run the Windows 11 VM

set -e

VM_DIR="vm"
DISK_IMAGE="$VM_DIR/windows11.qcow2"

if [ ! -f "$DISK_IMAGE" ]; then
    echo "Error: VM not set up yet. Run ./setup_windows_vm.sh first"
    exit 1
fi

echo "Starting Windows 11 VM..."
echo "To transfer files, use the shared folder method (see QEMU.md)"
echo ""

# Start VM with shared folder
qemu-system-x86_64 \
  -m 4G \
  -smp 2 \
  -cpu host \
  -accel hvf \
  -drive file="$DISK_IMAGE",if=virtio \
  -device virtio-net,netdev=net0 \
  -netdev user,id=net0,hostfwd=tcp::8000-:80,smb="$(pwd)/target/x86_64-pc-windows-gnu/release" \
  -display cocoa
