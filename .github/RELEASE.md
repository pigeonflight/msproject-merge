# GitHub Actions Automated Builds

This repository uses GitHub Actions to automatically build release artifacts for macOS and Windows.

## How it works

The workflow (`.github/workflows/build.yml`) triggers on:
- **Git tags** starting with `v` (e.g., `v1.0.0`, `v1.1.0`)
- **Manual dispatch** from the Actions tab

## Creating a Release

### Option 1: Using Git Tags (Recommended)

1. **Bump the version** in `Cargo.toml`:
   ```toml
   version = "1.0.1"
   ```

2. **Commit and tag**:
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 1.0.1"
   git tag v1.0.1
   git push origin main
   git push origin v1.0.1
   ```

3. **Automatic build**: GitHub Actions will:
   - Build macOS `.dmg`
   - Build Windows `.exe`
   - Create a GitHub Release with both files attached

### Option 2: Manual Trigger

1. Go to the **Actions** tab in your GitHub repository
2. Select **Build Release** workflow
3. Click **Run workflow**
4. Artifacts will be available for download (but won't create a release)

## Build Artifacts

After a successful build, you'll find:
- **macOS**: `MsProjectMerger-v{version}.dmg`
- **Windows**: `MsProjectMerger-v{version}.exe`

## Local Builds

You can still build locally using:
```bash
# macOS
./build_mac.sh

# Windows (requires Docker)
./build_win_docker.sh
```

## Caching

The workflow uses GitHub Actions cache to speed up builds by caching:
- Cargo registry
- Cargo index
- Build artifacts

This significantly reduces build times for subsequent runs.
