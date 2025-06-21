# Changelog

## [2025-06-21] - Dev Env Setup - IN_PROGRESS

### Implemented
- **CI/CD Pipeline**:
    - Created a basic GitHub Actions workflow (`ci.yml`) that runs tests on every push and pull request to `main`.
    - Configured the pipeline to run on Ubuntu, macOS, and Windows to ensure cross-platform compatibility.
    - Added steps for formatting (`cargo fmt`), linting (`cargo clippy`), and security scanning (`cargo audit`) to enforce code quality.
- **Test Utilities (`ciphr-test-utils`)**:
    - Implemented the initial `TestHarness` to provide isolated file system environments for tests using `tempfile`.
    - Added a `create_file_with_content` helper function to the `test-utils` crate.
- **Configuration Crate (`ciphr-config`)**:
    - Implemented a fluent `AppConfigBuilder` to allow for programmatic and testable configuration construction.
    - Implemented a `FileConfigurationProvider` to load settings from TOML files.
    - Implemented core configuration types (`AppConfig`, `LogLevel`, `LogFormat`).
    - Defined a `ConfigurationProvider` trait for loading config from various sources.
    - Created a comprehensive `ConfigError` enum for error handling.
- **Logging Crate (`ciphr-logging`)**:
    - Refactored the `init` module to return a `Layer` instead of initializing a global subscriber, giving consumers control over initialization and resolving complex testing issues.
    - Implemented a structured logging system using the `tracing` ecosystem.
    - Created a `JsonFormatter` for outputting logs in a machine-readable format.
    - Defined `RequestContext` to enable log correlation via request IDs.
- **Feature Flag Crate (`ciphr-feature-flags`)**:
    - Implemented a basic, extensible feature flag evaluation engine.
    - Defined a `FeatureFlagEvaluator` trait to support multiple evaluation strategies (Strategy Pattern).
    - Created a `PercentageRolloutEvaluator` for percentage-based rollouts.
- **Development Workflow (`justfile`)**:
    - Created a `justfile` with common development tasks (`test`, `fmt`, `clippy`, `build`, etc.).
    - Added `cargo-audit` to the dev environment for the `just audit` task.
- **Git Workflow Automation**:
    - Created shared Git hooks (`pre-commit`, `commit-msg`, `pre-push`) to automate formatting, linting, testing, and commit message validation.
    - Added a `.github/pull_request_template.md` to standardize PRs.
    - Wrote `docs/contributing.md` to explain the development workflow and contribution guidelines.
- **Devbox Environment**: Created a `devbox.json` to provide a consistent, cross-platform development environment with `rustup`, `just`, and `git`.
- **Core Workspace & Crates**:
    - Initialized a Rust workspace with a root `Cargo.toml`.
    - Created a modular crate structure under `crates/` (`cli`, `config`, `dev-env`, `feature-flags`, `logging`, `test-utils`).
    - Added foundational documentation (`architecture.md`, `environment-setup.md`).

### Technical Decisions
- **Task Runner (`just`)**: Chose `just` for its simplicity and Makefile-like syntax to run project scripts.
- **Environment Management (`Devbox`)**: Chose Devbox to ensure a reproducible development environment powered by Nix.
- **Architecture (`Modular Workspace`)**: Adopted a workspace structure with multiple crates to promote modularity and separation of concerns.
- **Git Hooks (`Shared Directory`)**: Adopted a shared `.githooks` directory to ensure all contributors use the same version-controlled quality gates.
- **Configuration (`Trait-based & Builder`)**: Designed the config crate with a `ConfigurationProvider` trait for flexibility and a fluent `AppConfigBuilder` for ergonomic, testable construction.
- **Logging (`tracing`)**: Chose the `tracing` crate for structured, context-aware diagnostics suitable for modern observability platforms.
- **Feature Flags (`Strategy Pattern`)**: Used the strategy pattern (`FeatureFlagEvaluator` trait) to create a decoupled and extensible evaluation engine.
- **CI/CD (`GitHub Actions`)**: Chose GitHub Actions for its tight integration with the source repository and multi-platform testing matrix.
- **Test Harness**: Implemented a `TestHarness` to provide isolated, temporary directories for file system-dependent tests.

### Current State
- A fluent builder is available for creating `AppConfig` instances.
- A basic, multi-platform CI pipeline automatically runs tests, lints, and security scans.
- The `ciphr-test-utils` crate has an initial test harness for isolated testing.
- The `ciphr-logging` crate provides configurable structured logging.
- The `ciphr-feature-flags` crate has a foundational evaluation engine.
- Core development environment and workflow automation are in place.
- Tasks #001-005, #007, #008, #010, and #011 are complete. Task #009 is in progress.

### Next Steps
- Implement environment-specific configuration overrides (Task #012).
- Continue implementation of the testing framework (Task #009).

### Technical Debt
- The `ciphr-config` crate does not yet support environment variable overrides or watching for file changes.
- The newly created crates contain only template code from `cargo new`. They need to be populated with their respective logic.