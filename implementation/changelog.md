# Changelog

## [2025-06-21] - Dev Env Setup - IN_PROGRESS

### Implemented
- **Devbox Environment**: Created a `devbox.json` to provide a consistent, cross-platform development environment with `rustup`, `just`, and `git`.
- **Validation Script**: Added an executable script (`scripts/validate-environment.sh`) to verify tool availability and versions.
- **Setup Documentation**: Wrote `docs/environment-setup.md` with clear instructions for new contributors.
- **Rust Workspace**: Initialized a Rust workspace with a root `Cargo.toml`.
- **Crate Structure**: Created a modular crate structure under `crates/` (`cli`, `config`, `dev-env`, `feature-flags`, `logging`, `test-utils`).
- **Architecture Docs**: Added `docs/architecture.md` to document the workspace structure.

### Technical Decisions
- **Environment Management**: Chose Devbox to ensure a reproducible development environment.
- **Rationale**: Devbox, powered by Nix, provides an isolated and consistent environment across macOS, Linux, and Windows (via WSL2), which is crucial for a distributed open-source team and community contributors. It simplifies onboarding to a single `devbox shell` command.
- **Architecture**: Adopted a workspace structure with multiple crates.
- **Rationale**: This promotes modularity, clear separation of concerns, and code reuse, which is essential for a large open-source project. It allows different components to be developed and tested independently.
- **Trade-offs**: Slightly more complex initial setup compared to a single crate, but significant long-term benefits in maintainability and scalability.

### Current State
- A reproducible development environment can be launched with `devbox shell`.
- The Rust workspace is successfully set up and validated with `cargo check`.
- Core crate structure is in place for future development.
- Architectural and setup documentation has been created.
- Tasks #001 and #002 from `tasks.md` are complete.

### Next Steps
- Proceed to Task #003: Git Workflow Automation and Quality Gates.
- Start implementing core functionality within the newly created crates, beginning with `ciphr-config` (Task #004).

### Technical Debt
- The newly created crates contain only template code from `cargo new`. They need to be populated with their respective logic.
- No actual code implementation has started yet.
