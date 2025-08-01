#!/bin/bash

# Try to execute into existing container
if docker exec -it kuavo_humanoid_rust_sdk zsh 2>/dev/null; then
    echo "Connected to existing container"
else
    echo "Container not running, starting with docker compose..."
    docker compose up -d
    # Wait a moment for container to start
    sleep 2
    # Connect to the newly started container
    docker exec -it kuavo_humanoid_rust_sdk zsh
fi