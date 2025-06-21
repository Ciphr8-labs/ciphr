# Crate-level Breakdown

The Ciphr workspace is organized as a collection of crates, each with a distinct responsibility. This modular design promotes code reuse, separation of concerns, and efficient compilation.

| Crate | Description |
|---|---|
| `ciphr-cli` | The main command-line interface for the application. |
| `ciphr-config` | Manages all configuration, including loading from files, environment variables, and providing a unified view to the rest of the application. |
| `ciphr-dev-env` | Contains tools and scripts specific to the development environment. |
| `ciphr-feature-flags` | Implements the feature flag evaluation engine, allowing for runtime toggling of features. |
| `ciphr-logging` | Provides structured logging capabilities for the entire application. |
| `ciphr-monitoring` | Defines traits and mocks for application monitoring and error reporting. |
| `ciphr-test-utils` | Contains testing utilities, harnesses, and mocks to support robust and isolated testing. | 