# Changelog

## [2025-06-21] - Dev Env Setup - IN_PROGRESS

### Implemented
- **Logging Crate (`ciphr-logging`)**:
    - Implemented a structured logging system using the `tracing` ecosystem.
    - Created a `JsonFormatter` for outputting logs in a machine-readable format.
    - Added an `init_logging` function to configure and initialize the global subscriber based on `AppConfig`.
    - Defined `RequestContext` to enable log correlation via request IDs.
    - Added comprehensive error types for the logging system.
    - Updated `AppConfig` in `ciphr-config` to include `log_format`.
    - Added unit tests to verify initialization with different formats.
- **Feature Flag Crate (`ciphr-feature-flags`)**:
    - Implemented a basic, extensible feature flag evaluation engine.
    - Defined a `FeatureFlagEvaluator` trait to support multiple evaluation strategies (Strategy Pattern).
    - Created a `PercentageRolloutEvaluator` for percentage-based rollouts.
    - Added a `FeatureFlagManager` to manage flag state and evaluation.
    - Included an `EvaluationContext` for passing user or request data.
    - Added dependencies (`uuid`, `rand`) for evaluation context and randomized rollouts.
    - Added unit tests for the percentage rollout strategy.
- **Development Workflow (`justfile`)**:
    - Created a `justfile` with common development tasks (`test`, `fmt`, `clippy`, `build`, etc.).
    - Added `cargo-audit` to the dev environment for the `just audit` task.
- **Configuration Crate (`ciphr-config`)**:
    - Implemented a `FileConfigurationProvider` to load settings from TOML files.
    - Added `#[serde(default)]` to allow optional fields in the config.
    - Implemented core configuration types (`AppConfig`, `LogLevel`).
    - Defined a `ConfigurationProvider` trait for loading config from various sources.
    - Created a comprehensive `ConfigError` enum for error handling.
    - Added unit tests for types, traits, and the new file loader.
    - The `FileConfigurationProvider` uses a trait-based design.
    - Rationale: This decouples configuration consumers from the loading mechanism, allowing for future extensions (e.g., environment variables, remote services) without altering application code.
- **Git Workflow Automation**:
    - Created shared Git hooks (`pre-commit`, `commit-msg`, `pre-push`) to automate formatting, linting, testing, and commit message validation.
    - Added a `.github/pull_request_template.md` to standardize PRs.
    - Wrote `docs/contributing.md` to explain the development workflow and contribution guidelines.
- **Devbox Environment**: Created a `devbox.json` to provide a consistent, cross-platform development environment with `rustup`, `just`, and `git`.
- **Validation Script**: Added an executable script (`scripts/validate-environment.sh`) to verify tool availability and versions.
- **Setup Documentation**: Wrote `docs/environment-setup.md` with clear instructions for new contributors.
- **Rust Workspace**: Initialized a Rust workspace with a root `Cargo.toml`.
- **Crate Structure**: Created a modular crate structure under `crates/` (`cli`, `config`, `dev-env`, `feature-flags`, `logging`, `test-utils`).
- **Architecture Docs**: Added `docs/architecture.md` to document the workspace structure.
- **Structured Logging (`tracing`)**: Chose the `tracing` crate as the foundation for logging and observability.
    - Rationale: `tracing` provides structured, context-aware diagnostics. Unlike traditional logging, it captures information about the execution flow, which is invaluable for debugging complex, asynchronous applications. The use of custom formatters allows us to produce JSON logs suitable for modern log aggregation platforms.
- **Feature Flag Engine (Strategy Pattern)**: The core of the feature flag system is the `FeatureFlagEvaluator` trait.
    - Rationale: This allows for different evaluation methods (e.g., on/off, percentage, user targeting) to be developed and swapped without changing the code that consumes the flags. It promotes extensibility and adheres to the Open/Closed Principle.

### Technical Decisions
- **Task Runner (`just`)**: Chose `just` as the command runner for its simplicity, Makefile-like syntax, and cross-platform compatibility.
- **Rationale**: `just` provides a convenient way to document and run project-specific commands, improving developer experience and ensuring consistency in how tasks are executed.
- **Separation of Concerns (Config)**: The `load` and `validate` functions in the `ConfigurationProvider` trait are kept separate.
- **Rationale**: This gives consumers of the crate more control. They can choose to load a configuration without immediately validating it, or re-validate a modified configuration object without reloading it from the source.
- **Config Trait-based Design**: The `ConfigurationProvider` trait was chosen to decouple the configuration consumers from the loading mechanism.
- **Rationale**: This allows for easy extension in the future. We can add providers for environment variables, remote configuration services (like Consul or etcd), or different file formats without changing the application code that consumes the configuration.
- **Git Hooks**: Adopted a shared `.githooks` directory instead of individual `.git/hooks` setups.
- **Rationale**: This ensures that all contributors use the same quality gates, making the process of enabling them as simple as a single `git config` command. It keeps workflow automation version-controlled and consistent.
- **Environment Management**: Chose Devbox to ensure a reproducible development environment.
- **Rationale**: Devbox, powered by Nix, provides an isolated and consistent environment across macOS, Linux, and Windows (via WSL2), which is crucial for a distributed open-source team and community contributors. It simplifies onboarding to a single `devbox shell` command.
- **Architecture**: Adopted a workspace structure with multiple crates.
- **Rationale**: This promotes modularity, clear separation of concerns, and code reuse, which is essential for a large open-source project. It allows different components to be developed and tested independently.
- **Trade-offs**: Slightly more complex initial setup compared to a single crate, but significant long-term benefits in maintainability and scalability.

### Current State
- The `ciphr-logging` crate provides configurable structured logging.
- The `ciphr-feature-flags` crate has a foundational evaluation engine.
- Common development tasks are automated and documented in a `justfile`.
- The `ciphr-config` crate can now load configuration from TOML files.
- The `ciphr-config` crate has its foundational types, traits, and tests implemented.
- Automated quality gates (format, lint, test, commit message validation) are available via Git hooks.
- Contribution guidelines and PR templates are in place.
- A reproducible development environment can be launched with `devbox shell`.
- The Rust workspace is successfully set up and validated with `cargo check`.
- Core crate structure is in place for future development.
- Architectural and setup documentation has been created.
- Tasks #001, #002, #003, #004, #005, #007, and #008 from `tasks.md` are complete.

### Next Steps
- Set up the testing framework and utilities (Task #009).
- Implement structured logging infrastructure with Tracing (Task #008).

### Technical Debt
- The `ciphr-config` crate does not yet support environment variable overrides or watching for file changes.
- The newly created crates contain only template code from `cargo new`. They need to be populated with their respective logic.
- No actual code implementation has started yet.
