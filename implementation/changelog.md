# Changelog

## [2025-06-21] - Dev Env Setup - IN_PROGRESS

### Implemented
- **Release Automation**:
    - Created a `release.yml` workflow using `release-plz` to automate release creation.
    - The workflow handles semantic versioning, changelog generation, and creates a GitHub release.
    - Configured `release-plz.toml` to build the `cli` binary for Linux, macOS, and Windows.
- **Performance Benchmarking**:
    - Set up `criterion` for performance benchmarking.
    - Added initial benchmarks for configuration loading and building.
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
- **Testing Framework**:
    - Integrated `proptest` for property-based testing.
    - The `ciphr-test-utils` crate now includes utilities for file system mocking, test harnesses, and property testing.
- **Monitoring Framework**:
    - Created a `ciphr-monitoring` crate to handle error tracking and event monitoring.
    - Defined a `MonitoringService` trait and a `MockMonitoringService` for local development.
- **Advanced Feature Flags**:
    - Implemented a `UserSegmentEvaluator` to target features to specific user groups.
    - Enhanced the `PercentageRolloutEvaluator` to use consistent hashing, ensuring users have a stable feature experience.
- **Community Workflow Automation**:
    - Added a workflow to welcome first-time contributors.
    - Set up the `all-contributors` bot for automated contributor recognition.
    - Implemented an automatic labeling workflow for pull requests based on file paths.

### Technical Decisions
- **Task Runner (`just`)**: Chose `just` for its simplicity and Makefile-like syntax to run project scripts.
- **Environment Management (`Devbox`)**: Chose Devbox to ensure a reproducible development environment powered by Nix.
- **Architecture (`Modular Workspace`)**: Adopted a workspace structure with multiple crates to promote modularity and separation of concerns.
- **Release Automation (`release-plz`)**: Chose `release-plz` to automate the release process for the workspace. It handles version calculation, changelog generation, and artifact creation, simplifying the process of creating consistent releases.
- **Configuration (`Layered & Builder`)**: Designed the config crate with a `ConfigurationProvider` trait that supports layering and a fluent `AppConfigBuilder` for ergonomic construction.
- **Logging (`tracing`)**: Chose the `tracing` crate for structured, context-aware diagnostics.
- **Feature Flags (`Strategy Pattern & Hashing`)**: The feature flag system uses the Strategy pattern for extensibility. For percentage rollouts, it now uses SipHasher to ensure consistent, sticky experiences for users.
- **CI/CD (`GitHub Actions`)**: Chose GitHub Actions for its tight integration, multi-platform matrix, and ability to have separate, focused workflows (CI, Security, Docs).
- **Security Scanning (`cargo-audit` & `cargo-deny`)**: A dedicated security workflow automates vulnerability and license checking. The separation keeps the main CI loop fast while ensuring security is not neglected.
- **Documentation (`mdBook`)**: A dedicated workflow automatically builds the `mdBook` documentation, ensuring it's always up-to-date.
- **Test Harness (`ciphr-test-utils`)**: Implemented a `TestHarness` to provide isolated, temporary directories for file system-dependent tests.
- **Benchmarking (`criterion`)**: Added `criterion` to the project to enable performance benchmarking. This will help identify and track performance regressions in critical code paths over time.
- **Git Hooks (`Shared Directory`)**: Adopted a shared `.githooks` directory to ensure all contributors use the same version-controlled quality gates.
- **Testing Framework (`proptest`, `criterion`)**: Established a comprehensive testing foundation. `criterion` is used for performance benchmarking, and `proptest` is integrated for property-based testing, allowing for more robust and exhaustive tests.
- **Monitoring (`Trait-based`)**: Established a trait-based approach for monitoring and error reporting (`MonitoringService`). This decouples the application from any specific monitoring service and allows for a mock implementation in tests.
- **Community Automation (`GitHub Actions`)**: Set up several workflows to improve the contributor experience, including a welcome message for first-timers, automatic PR labeling based on modified files, and the `all-contributors` bot for recognizing all types of contributions.

### Current State
- A manual release workflow is in place to automate versioning, changelogs, and releases.
- Performance benchmarks are in place for the `config` crate.
- A dedicated documentation pipeline builds the `mdBook`.
- A dedicated security pipeline runs vulnerability and license checks.
- A basic, multi-platform CI pipeline automatically runs tests, lints, and format checks.
- The `ciphr-config` crate now supports layered configurations from multiple sources.
- The `ciphr-test-utils` crate has a test harness and file utilities.
- The `ciphr-logging` and `ciphr-feature-flags` crates have their foundational logic.
- The `ciphr-monitoring` crate has a `MonitoringService` trait and a `MockMonitoringService` for local development.
- Core development environment and workflow automation are in place.
- Tasks #001-019 are complete, with the exception of some sub-items in #018.

### Next Steps
- Implement production monitoring and observability setup (Task #020).
- Fully integrate the monitoring service throughout the application (revisiting Task #018).

### Technical Debt
- The community metrics and feedback collection processes are not yet automated.
- The feature flag system does not yet support A/B testing or analytics.
- The release workflow does not yet publish to package registries like crates.io.
- The `ciphr-config` crate does not yet support environment variable overrides or watching for file changes.
- The newly created crates contain only template code from `cargo new`. They need to be populated with their respective logic.