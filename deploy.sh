#!/bin/bash
set -e

# Configuration (customize these)
PI_IP=${1:-"raspberrypi.local"}  # Default Pi hostname/IP
PI_USER=${2:-"pi"}               # Default username
DEPLOY_DIR="/home/$PI_USER/epaper_ui"

# Target architecture (for Raspberry Pi Zero)
TARGET="arm-unknown-linux-gnueabihf"

# Generate a version based on date and git commit
VERSION="$(date +%Y%m%d)-$(git rev-parse --short HEAD 2>/dev/null || echo 'unknown')"
echo "Building version: $VERSION"

# Build first
echo "Building for Raspberry Pi ($TARGET)..."
sudo docker run --rm -v "$(pwd):/app" epaper-builder cargo build --release --target $TARGET

# Check if build succeeded
if [ ! -f "target/$TARGET/release/hello_world" ] || [ ! -f "target/$TARGET/release/weather" ]; then
    echo "Build failed: binaries not found"
    exit 1
fi

# Create directory on Pi if it doesn't exist
echo "Creating directory on Raspberry Pi..."
ssh $PI_USER@$PI_IP "mkdir -p $DEPLOY_DIR"

# Copy binaries
echo "Deploying to $PI_USER@$PI_IP:$DEPLOY_DIR..."
scp target/$TARGET/release/hello_world $PI_USER@$PI_IP:$DEPLOY_DIR/
scp target/$TARGET/release/weather $PI_USER@$PI_IP:$DEPLOY_DIR/

# Create a version file
echo $VERSION > version.txt
scp version.txt $PI_USER@$PI_IP:$DEPLOY_DIR/

echo "Deployment complete!"