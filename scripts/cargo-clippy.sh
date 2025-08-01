#!/bin/bash

# Get list of staged Rust files
STAGED_RS_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$')

if [ -z "$STAGED_RS_FILES" ]; then
    echo "No staged Rust files found."
    exit 0
fi

echo "Staged Rust files:"
echo "$STAGED_RS_FILES"

# Check if any staged files are in src/ (main library or binaries)
STAGED_SRC_FILES=$(echo "$STAGED_RS_FILES" | grep '^src/')

if [ -n "$STAGED_SRC_FILES" ]; then
    echo "Running clippy on staged files..."
    
    # Run clippy fix first (allow dirty/staged for pre-commit)
    cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged -- -D warnings
    
    # Add any auto-fixed files back to staging
    git add -u
    
    # Then run clippy check without --locked to use the fixed code
    cargo clippy --workspace --all-targets --all-features -- -D warnings
else
    echo "No staged files in src/ directory, skipping clippy."
fi