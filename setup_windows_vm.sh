#!/bin/bash
# Quick setup script for Windows 11 VM with QEMU

set -e

VM_DIR="vm"
DISK_IMAGE="$VM_DIR/windows11.qcow2"
DISK_SIZE="50G"

echo "Setting up Windows 11 VM with QEMU..."

# Create VM directory
mkdir -p "$VM_DIR"

# Check if disk image already exists
if [ -f "$DISK_IMAGE" ]; then
    echo "✓ Virtual disk already exists: $DISK_IMAGE"
else
    echo "Creating virtual disk ($DISK_SIZE)..."
    qemu-img create -f qcow2 "$DISK_IMAGE" "$DISK_SIZE"
    echo "✓ Virtual disk created: $DISK_IMAGE"
fi

# Check for Windows ISO
ISO_PATH=""
if [ -f "$HOME/Downloads/Win11_English_x64.iso" ]; then
    ISO_PATH="$HOME/Downloads/Win11_English_x64.iso"
elif [ -f "$HOME/Downloads/Win11_23H2_English_x64.iso" ]; then
    ISO_PATH="$HOME/Downloads/Win11_23H2_English_x64.iso"
elif [ -f "$HOME/Downloads/Win11_25H2_English_x64.iso" ]; then
    ISO_PATH="$HOME/Downloads/Win11_25H2_English_x64.iso"
fi

if [ -z "$ISO_PATH" ]; then
    echo ""
    echo "⚠️  Windows 11 ISO not found in ~/Downloads/"
    echo ""
    echo "Please download Windows 11 ISO from:"
    echo "https://www.microsoft.com/software-download/windows11"
    echo ""
    echo "Save it to ~/Downloads/ and run this script again."
    exit 1
fi

echo "✓ Found Windows ISO: $ISO_PATH"
echo ""
echo "Starting Windows installation..."
echo "This will open a QEMU window. Follow the Windows setup wizard."
echo ""
echo "Press Enter to continue..."
read

# Try to use HVF acceleration, fall back to TCG if not available
ACCEL_FLAG=""
if qemu-system-x86_64 -accel help 2>&1 | grep -q hvf; then
    echo "Using HVF acceleration (fast)"
    ACCEL_FLAG="-accel hvf"
else
    echo "Using TCG emulation (slower, but works)"
    ACCEL_FLAG="-accel tcg"
fi

# Start installation
qemu-system-x86_64 \
  -m 4G \
  -smp 2 \
  -cpu qemu64 \
  $ACCEL_FLAG \
  -drive file="$DISK_IMAGE",if=virtio \
  -cdrom "$ISO_PATH" \
  -boot d \
  -device virtio-net,netdev=net0 \
  -netdev user,id=net0 \
  -display cocoa

echo ""
echo "Installation complete!"
echo "To run the VM again, use: ./run_windows_vm.sh"
