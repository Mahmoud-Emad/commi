#!/bin/bash

# Release automation script for Commi
# Usage: ./scripts/release.sh <version>

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RESET='\033[0m'

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${RESET} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${RESET} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${RESET} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${RESET} $1"
}

# Check if version is provided
if [ $# -eq 0 ]; then
    print_error "Version number is required"
    echo "Usage: $0 <version>"
    echo "Example: $0 4.1.0"
    exit 1
fi

VERSION=$1

# Validate version format (semantic versioning)
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "Invalid version format. Use semantic versioning (e.g., 4.1.0)"
    exit 1
fi

print_status "Starting release process for version $VERSION"

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    print_error "Must be on main branch to create a release"
    exit 1
fi

# Check if working directory is clean
if ! git diff-index --quiet HEAD --; then
    print_error "Working directory is not clean. Please commit or stash changes."
    exit 1
fi

# Pull latest changes
print_status "Pulling latest changes from origin..."
git pull origin main

# Run tests
print_status "Running tests..."
cargo test --all-features

# Run clippy
print_status "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
print_status "Checking code formatting..."
cargo fmt --all -- --check

# Update version in Cargo.toml
print_status "Updating version in Cargo.toml..."
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Update Cargo.lock
print_status "Updating Cargo.lock..."
cargo check

# Update version in man page
print_status "Updating version in man page..."
sed -i.bak "s/^\.TH COMMI 1 \".*\"/\.TH COMMI 1 \"$(date +'%B %Y')\" \"commi $VERSION\"/" docs/commi.1
rm docs/commi.1.bak

# Build release
print_status "Building release..."
cargo build --release

# Create git tag
print_status "Creating git tag v$VERSION..."
git add Cargo.toml Cargo.lock docs/commi.1
git commit -m "chore: bump version to $VERSION"
git tag -a "v$VERSION" -m "Release version $VERSION"

# Push changes and tag
print_status "Pushing changes and tag to origin..."
git push origin main
git push origin "v$VERSION"

print_success "Release $VERSION created successfully!"
print_status "GitHub Actions will now build and publish the release automatically."
print_status "Monitor the progress at: https://github.com/Mahmoud-Emad/commi/actions"
