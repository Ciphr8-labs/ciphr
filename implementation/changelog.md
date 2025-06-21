# Changelog

## [2025-06-21] - Dev Env Setup - IN_PROGRESS

### Implemented
- **Documentation Pipeline**:
    - Created a dedicated `docs.yml` workflow to automatically build the project's `mdBook` documentation.
    - The generated book is uploaded as a build artifact for future deployment.
- **CI/CD Pipeline**:
    - Created a dedicated `security.yml` workflow for security-related checks.
    - The security workflow runs `cargo audit` for vulnerability scanning and `cargo deny` for license and dependency checks.
    - Created a basic GitHub Actions workflow (`ci.yml`) that runs tests, lints, and format checks on every push and pull request to `main`.
    - Configured the CI pipeline to run on Ubuntu, macOS, and Windows.
- **Test Utilities (`ciphr-test-utils`)**:
    - Implemented the initial `TestHarness` to provide isolated file system environments for tests.
    - Added a `create_file_with_content` helper function.
- **Configuration Crate (`ciphr-config`)**:
    - Implemented a `LayeredConfigurationProvider` to merge multiple configuration sources.
    - Refactored `AppConfig` fields to be `Option`-based to support robust merging.
    - Implemented a fluent `AppConfigBuilder` for programmatic configuration construction.
    - Implemented a `FileConfigurationProvider` to load settings from TOML files.
- **Logging Crate (`ciphr-logging`)**:
    - Refactored the `init` module to return a `Layer` instead of initializing a global subscriber.
    - Implemented a structured logging system using the `tracing` ecosystem.
- **Feature Flag Crate (`ciphr-feature-flags`)**:
    - Implemented a basic, extensible feature flag evaluation engine using the Strategy Pattern.
- **Development Workflow & Tooling**:
    - Created a `justfile` with common development tasks.
    - Created shared Git hooks (`pre-commit`, `commit-msg`, `pre-push`).
    - Added a `.github/pull_request_template.md`.
    - Created a `devbox.json` for a consistent, cross-platform development environment.
- **Core Workspace & Crates**:
    - Initialized a Rust workspace and a modular crate structure.
    - Added foundational documentation (`architecture.md`, `contributing.md`, `environment-setup.md`).

### Technical Decisions
- **Task Runner (`just`)**: Chose `just` for its simplicity and Makefile-like syntax to run project scripts.
- **Environment Management (`Devbox`)**: Chose Devbox to ensure a reproducible development environment powered by Nix.
- **Architecture (`Modular Workspace`)**: Adopted a workspace structure with multiple crates to promote modularity and separation of concerns.
- **Git Hooks (`Shared Directory`)**: Adopted a shared `.githooks` directory to ensure all contributors use the same version-controlled quality gates.
- **Configuration (`Layered & Builder`)**: Designed the config crate with a `ConfigurationProvider` trait that supports layering and a fluent `AppConfigBuilder` for ergonomic construction.
- **Logging (`tracing`)**: Chose the `tracing` crate for structured, context-aware diagnostics.
- **Feature Flags (`Strategy Pattern`)**: Used the strategy pattern to create a decoupled and extensible evaluation engine.
- **CI/CD (`GitHub Actions`)**: Chose GitHub Actions for its tight integration, multi-platform matrix, and ability to have separate, focused workflows (CI, Security, Docs).
- **Security Scanning (`cargo-audit` & `cargo-deny`)**: A dedicated security workflow automates vulnerability and license checking, keeping the main CI loop fast.
- **Documentation (`mdBook`)**: A dedicated workflow automatically builds the `mdBook` documentation, ensuring it's always up-to-date.
- **Test Harness**: Implemented a `TestHarness` to provide isolated, temporary directories for file system-dependent tests.

### Current State
- A documentation pipeline automatically builds the project's `mdBook`.
- A dedicated security pipeline runs vulnerability and license checks.
- A basic, multi-platform CI pipeline automatically runs tests, lints, and format checks.
- The `ciphr-config` crate now supports layered configurations from multiple sources.
- The `ciphr-test-utils` crate has a test harness and file utilities.
- The `ciphr-logging` and `ciphr-feature-flags` crates have their foundational logic.
- Core development environment and workflow automation are in place.
- Tasks #001-005, #007, #008, #010, #011, #012, #013, and #014 are complete. Task #009 is in progress.

### Next Steps
- Implement performance benchmarking (Task #015).
- Continue implementation of the testing framework (Task #009).

### Technical Debt
- The `ciphr-config` crate does not yet support environment variable overrides or watching for file changes.
- The newly created crates contain only template code from `cargo new`. They need to be populated with their respective logic.