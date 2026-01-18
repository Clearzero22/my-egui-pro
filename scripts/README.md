# Release Scripts

This directory contains scripts for building release packages of the Hacker News Reader.

## Scripts

### `release.sh` - Linux / macOS
Build release binary and create tarball archive.

**Usage:**
```bash
# Build with version from Cargo.toml
./release.sh

# Build with custom version
./release.sh 1.0.0
```

**Output:**
- `release/my_egui_pro-{version}-{platform}/` - Release directory with binary
- `release/my_egui_pro-{version}-{platform}.tar.gz` - Compressed archive
- `release/my_egui_pro-{version}-{platform}.sha256` - SHA256 checksum

### `release.bat` - Windows
Build release binary and create ZIP archive.

**Usage:**
```cmd
REM Build with version from Cargo.toml
release.bat

REM Build with custom version
release.bat 1.0.0
```

**Output:**
- `release\my_egui_pro-{version}-windows-amd64\` - Release directory with binary
- `release\my_egui_pro-{version}-windows-amd64.zip` - Compressed archive

## Platform Detection

The scripts automatically detect the platform and create appropriately named releases:

| OS | Platform Name | Binary Name |
|----|---------------|-------------|
| Linux (x86_64) | `linux-amd64` | `my_egui_pro` |
| Linux (ARM64) | `linux-arm64` | `my_egui_pro` |
| macOS (Intel) | `macos-amd64` | `my_egui_pro` |
| macOS (Apple Silicon) | `macos-arm64` | `my_egui_pro` |
| Windows | `windows-amd64` | `my_egui_pro.exe` |

## Installation

Users can install by:

**Linux/macOS:**
```bash
tar -xzf my_egui_pro-0.1.0-linux-amd64.tar.gz
cd my_egui_pro-0.1.0-linux-amd64
./my_egui_pro
```

**Windows:**
```powershell
Expand-Archive -Path my_egui_pro-0.1.0-windows-amd64.zip
cd my_egui_pro-0.1.0-windows-amd64
.\my_egui_pro.exe
```

## Building on Different Platforms

To build for multiple platforms, you have several options:

### Option 1: Build on each platform natively
Run `./release.sh` on Linux/macOS or `release.bat` on Windows.

### Option 2: Use GitHub Actions
The `.github/workflows/build.yml` workflow automatically builds for all platforms when you push a tag.

### Option 3: Cross-compile (advanced)
Use `cross` or setup toolchains for target platforms.

## Dependencies

The scripts require:
- `cargo` and `rustc` (Rust toolchain)
- `tar` and `gzip` (Linux/macOS) or `powershell` (Windows) - usually preinstalled

No additional dependencies needed for building!
