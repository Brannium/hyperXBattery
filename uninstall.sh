#!/bin/bash

set -e

TOOL_NAME="hyperXBattery"
INSTALL_DIR="/usr/local/bin"

echo "Uninstalling $TOOL_NAME..."

# Check if tool is installed
if [ ! -f "$INSTALL_DIR/$TOOL_NAME" ]; then
    echo "Warning: $TOOL_NAME not found in $INSTALL_DIR"
    echo "It may already be uninstalled."
    exit 0
fi

# Confirm before uninstalling
read -p "Are you sure you want to uninstall $TOOL_NAME? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Uninstall cancelled."
    exit 0
fi

# Remove binary
echo "Removing binary from $INSTALL_DIR..."
sudo rm -f "$INSTALL_DIR/$TOOL_NAME"

echo "$TOOL_NAME has been uninstalled successfully!"

