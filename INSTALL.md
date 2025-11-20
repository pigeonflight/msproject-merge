# Installation Instructions

## macOS Installation

### Method 1: Right-Click to Open (Recommended)

1. Download `MsProjectMerge-v1.0.0.dmg`
2. **Right-click** (or Control+click) on the DMG file
3. Select **"Open"** from the context menu
4. Click **"Open"** in the security dialog
5. Drag the app to your Applications folder
6. When you first launch the app:
   - **Right-click** on the app icon
   - Select **"Open"**
   - Click **"Open"** to confirm

### Method 2: System Settings Override

If you get a "cannot be opened" error:

1. Go to **System Settings** → **Privacy & Security**
2. Scroll down to the **Security** section
3. You'll see a message about the blocked app
4. Click **"Open Anyway"**
5. Confirm by clicking **"Open"**

### Method 3: Remove Quarantine Attribute (Advanced)

Open Terminal and run:
```bash
xattr -d com.apple.quarantine ~/Downloads/MsProjectMerge-v1.0.0.dmg
```

Then mount the DMG normally.

## Windows Installation

1. Download `MsProjectMerge-v1.0.0.exe`
2. Double-click to run
3. If Windows Defender SmartScreen appears:
   - Click **"More info"**
   - Click **"Run anyway"**

## Why These Steps Are Needed

The application is not code-signed with an Apple Developer certificate or Windows certificate. This is normal for open-source software. The steps above tell your operating system that you trust this application.

## Building from Source (Alternative)

If you prefer to build from source instead:

```bash
git clone https://github.com/pigeonflight/msproject-merge.git
cd msproject-merge
cargo build --release
cargo run --release
```

This way, you're building the app yourself and macOS/Windows won't block it.

## Future: Code Signing

To remove these security warnings in future releases, the app would need to be:
- **macOS**: Signed with an Apple Developer certificate ($99/year)
- **Windows**: Signed with a code signing certificate (~$200-400/year)

For now, the workarounds above are the standard approach for open-source applications.
