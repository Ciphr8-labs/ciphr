# A collection of useful project-specific commands.

# Default task runs a quick check
default: build

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
    @echo " auditing dependencies..."
    @cargo audit

# Setup development environment
setup:
    @echo " installing cargo tools..."
    @cargo install cargo-audit
    @cargo install cargo-deny
    @cargo install mdbook

# Generate documentation
doc:
    cargo doc --open

# List all available tasks
list:
    just --list 