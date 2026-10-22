#!/bin/bash

set -e  # Exit on any error

TOOL_NAME="hyperXBattery"
INSTALL_DIR="/usr/local/bin"

echo "Installing $TOOL_NAME..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Build the release binary
echo "Building $TOOL_NAME..."
cargo build --release

# Get the binary path
BINARY="target/release/$TOOL_NAME"

if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    exit 1
fi

# Install to system location (requires sudo)
echo "Installing to $INSTALL_DIR (requires sudo)..."
sudo cp "$BINARY" "$INSTALL_DIR/$TOOL_NAME"
sudo chmod 755 "$INSTALL_DIR/$TOOL_NAME"

echo "$TOOL_NAME installed successfully!"
echo "You can now run: $TOOL_NAME"
