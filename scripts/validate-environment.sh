#!/bin/bash
#
# This script validates that all necessary tools for the development environment
# are installed and available in the current path.

set -e # Exit immediately if a command exits with a non-zero status.

echo "🔎 Validating development environment..."

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Tool validation checks
validate_tool() {
    local tool_name=$1
    local version_command=$2
    
    echo -n "Checking for $tool_name..."
    if ! command_exists "$tool_name"; then
        echo "❌ Not found."
        echo "Error: $tool_name is not installed or not in your PATH." >&2
        exit 1
    fi

    # Capture version and check for command failure
    local version
    if ! version=$($version_command 2>&1); then
        echo "❌ Error executing '$version_command'."
        echo "Error output:" >&2
        echo "$version" >&2
        exit 1
    fi
    
    echo "✅ Found ($version)"
}

# Perform validations
validate_tool "rustc" "rustc --version"
validate_tool "cargo" "cargo --version"
validate_tool "just" "just --version"
validate_tool "git" "git --version"

echo "✅ Environment validation successful!" 