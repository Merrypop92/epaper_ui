#!/bin/bash
set -e

# Configuration
CI_MODE=${1:-"false"}  # Run in CI mode if specified

# Get the previous commit hash
PREV_COMMIT=$(cat .last_deployed_commit 2>/dev/null || echo "none")
CURRENT_COMMIT=$(git rev-parse HEAD)

# Check if we need to build
if [ "$CI_MODE" == "true" ] || [ "$PREV_COMMIT" != "$CURRENT_COMMIT" ]; then
    echo "Changes detected, building and deploying..."
    
    # Run the build
    ./deploy.sh
    
    # Save the current commit hash
    echo $CURRENT_COMMIT > .last_deployed_commit
    
    echo "CI/CD completed successfully!"
else
    echo "No changes detected, skipping build."
fi