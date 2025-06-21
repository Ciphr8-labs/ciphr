# Development Workflow

This document outlines the standard development process, from running tests to committing code. All tasks are managed through our `justfile`.

## Common Tasks

The `just` command runner provides simple, cross-platform commands for all common development activities.

| Command | Description |
|---|---|
| `just setup` | Validates the development environment to ensure all tools and dependencies are ready. |
| `just dev` | Starts a development session with live-reloading. Files are watched for changes, and tests are re-run automatically. |
| `just build` | Compiles the entire workspace. |
| `just test` | Runs all unit and integration tests. |
| `just fmt` | Formats all code according to project standards. |
| `just clippy` | Lints the code for common errors and style issues. |
| `just check` | Checks the code without producing a build artifact. |
| `just audit` | Runs a security audit on all dependencies. |
| `just clean` | Removes all build artifacts. |
| `just docs` | Builds the mdBook documentation. |

## Git Workflow

1.  **Create a Feature Branch**:
    Branches should follow the convention `feature/task-name` or `fix/issue-description`.

2.  **Make Commits**:
    Commit messages must follow the [Conventional Commits](https://www.conventionalcommits.org/) standard. This is enforced by a pre-commit hook.
    ```
    feat(config): Add support for layered configuration
    ```

3.  **Run Quality Checks**:
    Before pushing, always run local checks:
    ```bash
    just fmt
    just clippy
    just test
    ```
    These checks are also enforced by a `pre-push` git hook.

4.  **Open a Pull Request**:
    Push your branch to GitHub and open a Pull Request against the `main` branch. The PR template will guide you through the required information.

5.  **Review and Merge**:
    All PRs require at least one approving review and must pass all CI checks before they can be merged. 