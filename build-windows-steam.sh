#!/bin/bash
set -euo pipefail

if ! command -v zip &> /dev/null; then
    echo "ERROR: 'zip' is not installed. Install it with:"
    echo "  sudo apt-get install -y zip"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/tower-defense"
BUILD_OUTPUT_DIR="$PROJECT_DIR/target/namui/x86_64-pc-windows-msvc"
ZIP_NAME="tower-defense-windows-steam.zip"
ZIP_PATH="$SCRIPT_DIR/$ZIP_NAME"

echo "=== Building tower-defense for x86_64-pc-windows-msvc ==="
namui build x86_64-pc-windows-msvc --manifest-path "$PROJECT_DIR/Cargo.toml" --release

echo "=== Build complete ==="

if [ ! -d "$BUILD_OUTPUT_DIR" ]; then
    echo "ERROR: Build output directory not found: $BUILD_OUTPUT_DIR"
    exit 1
fi

echo "=== Creating zip: $ZIP_NAME ==="
rm -f "$ZIP_PATH"
cd "$BUILD_OUTPUT_DIR"
zip -r "$ZIP_PATH" .

WINDOWS_DESKTOP="/mnt/c/Users/namse/Desktop"
if [ -d "$WINDOWS_DESKTOP" ]; then
    cp "$ZIP_PATH" "$WINDOWS_DESKTOP/$ZIP_NAME"
    echo "=== Copied to Windows Desktop ==="
fi

echo "=== Done ==="
echo "Output: $ZIP_PATH"
echo "Size: $(du -h "$ZIP_PATH" | cut -f1)"
