#!/bin/bash
# Release Build Script for Hacker News Reader
# Usage: ./scripts/release.sh [version]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get version from argument or Cargo.toml
VERSION=${1:-$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)}
APP_NAME="my_egui_pro"
APP_DISPLAY_NAME="Hacker News Reader"

echo -e "${GREEN}=== Building ${APP_DISPLAY_NAME} v${VERSION} ===${NC}"

# Get target OS
OS=$(uname -s)
ARCH=$(uname -m)

# Determine binary name and extension
if [[ "$OS" == "Linux" ]]; then
    BINARY_NAME="${APP_NAME}"
    EXTENSION=""
    PLATFORM="linux"
    if [[ "$ARCH" == "aarch64" ]]; then
        PLATFORM="linux-arm64"
    else
        PLATFORM="linux-amd64"
    fi
elif [[ "$OS" == "Darwin" ]]; then
    BINARY_NAME="${APP_NAME}"
    EXTENSION=""
    PLATFORM="macos"
    if [[ "$ARCH" == "arm64" ]]; then
        PLATFORM="macos-arm64"
    else
        PLATFORM="macos-amd64"
    fi
else
    BINARY_NAME="${APP_NAME}.exe"
    EXTENSION=".exe"
    PLATFORM="windows"
fi

# Clean previous build
echo -e "${YELLOW}Cleaning previous build...${NC}"
cargo clean

# Build release
echo -e "${YELLOW}Building release binary...${NC}"
cargo build --release

# Create release directory
RELEASE_DIR="release/${APP_NAME}-${VERSION}-${PLATFORM}"
mkdir -p "$RELEASE_DIR"

# Copy binary
echo -e "${YELLOW}Copying binary to release directory...${NC}"
cp "target/release/${BINARY_NAME}" "${RELEASE_DIR}/${APP_NAME}${EXTENSION}"

# Make executable (Linux/macOS)
if [[ "$OS" != "Windows" ]]; then
    chmod +x "${RELEASE_DIR}/${APP_NAME}${EXTENSION}"
fi

# Create archive
echo -e "${YELLOW}Creating release archive...${NC}"
cd release
if [[ "$OS" == "Linux" ]]; then
    tar -czf "${APP_NAME}-${VERSION}-${PLATFORM}.tar.gz" "${APP_NAME}-${VERSION}-${PLATFORM}/"
    echo -e "${GREEN}Created: ${APP_NAME}-${VERSION}-${PLATFORM}.tar.gz${NC}"
elif [[ "$OS" == "Darwin" ]]; then
    tar -czf "${APP_NAME}-${VERSION}-${PLATFORM}.tar.gz" "${APP_NAME}-${VERSION}-${PLATFORM}/"
    echo -e "${GREEN}Created: ${APP_NAME}-${VERSION}-${PLATFORM}.tar.gz${NC}"
fi
cd ..

# Generate checksum
echo -e "${YELLOW}Generating checksums...${NC}"
if command -v sha256sum &> /dev/null; then
    cd release
    sha256sum "${APP_NAME}-${VERSION}-${PLATFORM}.tar.gz" > "${APP_NAME}-${VERSION}-${PLATFORM}.sha256"
    echo -e "${GREEN}Checksum: $(cat ${APP_NAME}-${VERSION}-${PLATFORM}.sha256)${NC}"
    cd ..
fi

# Create release notes
cat > "${RELEASE_DIR}/RELEASE_NOTES.md" << EOF
# ${APP_DISPLAY_NAME} v${VERSION} - ${PLATFORM}

## Installation

### Extract
\`\`\`bash
tar -xzf ${APP_NAME}-${VERSION}-${PLATFORM}.tar.gz
cd ${APP_NAME}-${VERSION}-${PLATFORM}
\`\`\`

### Run
\`\`\`bash
./${APP_NAME}${EXTENSION}
\`\`\`

## Features

- Hacker News reader with 6 categories (Top, New, Best, Ask, Show, Jobs)
- Clickable story titles to open articles
- Save favorite stories (SQLite storage)
- Gruvbox theme (Dark/Light) with toggle

## System Requirements

- ${PLATFORM^} system
- No additional dependencies required

## Data Location

- Config: \`~/.local/share/my_egui_pro/config.json\`
- Database: \`~/.local/share/my_egui_pro/favorites.db\`

## Changelog

- Initial release
EOF

# Summary
echo ""
echo -e "${GREEN}=== Build Complete ===${NC}"
echo -e "Platform: ${GREEN}${PLATFORM}${NC}"
echo -e "Version: ${GREEN}${VERSION}${NC}"
echo -e "Binary: ${GREEN}${RELEASE_DIR}/${APP_NAME}${EXTENSION}${NC}"
echo -e "Archive: ${GREEN}release/${APP_NAME}-${VERSION}-${PLATFORM}.tar.gz${NC}"
echo ""
echo -e "${YELLOW}To install:${NC}"
echo "  tar -xzf release/${APP_NAME}-${VERSION}-${PLATFORM}.tar.gz"
echo "  cd ${APP_NAME}-${VERSION}-${PLATFORM}"
echo "  ./${APP_NAME}${EXTENSION}"
