# Architecture

This document outlines the architecture of the Ciphr project, which is organized as a Rust workspace with multiple crates. This approach promotes modularity, separation of concerns, and reusability.

## Workspace Structure

The project is a single workspace defined in the root `Cargo.toml`. All individual crates are located in the `crates/` directory.

```
ciphr/
├── Cargo.toml      # Workspace configuration
├── crates/         # Source code for all crates
│   ├── cli/            # Command-Line Interface
│   ├── config/         # Configuration management
│   ├── dev-env/        # Development environment utilities
│   ├── feature-flags/  # Feature flag management
│   ├── logging/        # Logging and tracing setup
│   └── test-utils/     # Shared utilities for testing
├── docs/
└── implementation/
```

## Crate Responsibilities

### `cli` (Binary)
- **Purpose**: The main entry point for the command-line application.
- **Responsibilities**:
    - Parsing command-line arguments.
    - Orchestrating calls to other library crates to execute commands.
    - Handling user-facing output.

### `config` (Library)
- **Purpose**: Provides a type-safe configuration management system.
- **Responsibilities**:
    - Defining configuration structures.
    - Loading configuration from files (e.g., TOML).
    - Validating configuration and providing sensible defaults.
    - Handling environment-specific overrides.

### `dev-env` (Library)
- **Purpose**: Contains tools and scripts for managing the development environment.
- **Responsibilities**:
    - Validating the development environment setup.
    - Automating development tasks.
    - Ensuring a consistent environment for all contributors.

### `feature-flags` (Library)
- **Purpose**: Implements a feature flag system for controlling feature rollout.
- **Responsibilities**:
    - Evaluating the state of feature flags.
    - Supporting different rollout strategies (e.g., percentage-based).
    - Providing an interface for application components to check feature flags.

### `logging` (Library)
- **Purpose**: Sets up and configures structured logging for the entire application.
- **Responsibilities**:
    - Initializing the `tracing` subscriber.
    - Supporting different log formats (e.g., JSON, human-readable).
    - Configuring log levels and output destinations.

### `test-utils` (Library)
- **Purpose**: Provides shared utilities and helpers for writing tests across the workspace.
- **Responsibilities**:
    - Test setup and teardown logic.
    - Mocking and stubbing utilities.
    - Common assertions and test fixtures. 