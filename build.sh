#!/bin/bash
set -e

# Determine Raspberry Pi model (zero or 3a)
PI_MODEL=${1:-"zero"}  # Default to zero if not specified

if [ "$PI_MODEL" == "zero" ]; then
    TARGET="arm-unknown-linux-gnueabihf"
elif [ "$PI_MODEL" == "3a" ]; then
    TARGET="armv7-unknown-linux-gnueabihf"
else
    echo "Unknown model: $PI_MODEL. Use 'zero' or '3a'"
    exit 1
fi

echo "Building for Raspberry Pi $PI_MODEL ($TARGET)..."

# Build all binaries
# cargo build --release --target $TARGET --bin epaper_ui
cargo build --release --target $TARGET --bin hello_world
cargo build --release --target $TARGET --bin weather

echo "Build complete. Binaries in target/$TARGET/release/"