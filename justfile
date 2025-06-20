# A collection of useful project-specific commands.

# Default task runs a quick check
default: check

# Lint all crates
clippy:
    cargo clippy --all-targets -- -D warnings

# Format all crates
fmt:
    cargo fmt --all

# Run all tests
test:
    cargo test --all

# Check the project for errors without building
check:
    cargo check --all

# Build the project
build:
    cargo build

# Build the project in release mode
build-release:
    cargo build --release

# Clean the target directory
clean:
    cargo clean

# Run security audit for dependencies
audit:
    cargo audit

# Generate documentation
doc:
    cargo doc --open

# List all available tasks
list:
    just --list 