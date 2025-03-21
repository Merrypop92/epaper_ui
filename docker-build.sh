#!/bin/bash
set -e

# Build the Docker image if it doesn't exist
if ! docker image inspect epaper-builder &>/dev/null; then
    echo "Building Docker image for cross-compilation..."
    docker build -t epaper-builder -f Dockerfile.build .
fi

# Set Raspberry Pi model
PI_MODEL=${1:-"zero"}

if [ "$PI_MODEL" == "zero" ]; then
    TARGET="arm-unknown-linux-gnueabihf"
elif [ "$PI_MODEL" == "3a" ]; then
    TARGET="armv7-unknown-linux-gnueabihf"
else
    echo "Unknown model: $PI_MODEL. Use 'zero' or '3a'"
    exit 1
fi

echo "Building for Raspberry Pi $PI_MODEL ($TARGET)..."

# Run the build in Docker
docker run --rm -v "$(pwd):/app" epaper-builder \
    sh -c "cargo build --release --target $TARGET"

echo "Build completed. Binaries in target/$TARGET/release/"