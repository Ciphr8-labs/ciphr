# Contributing to Ciphr

First off, thank you for considering contributing to Ciphr! It's people like you that make Ciphr such a great tool.

Following these guidelines helps to communicate that you respect the time of the developers managing and developing this open source project. In return, they should reciprocate that respect in addressing your issue or assessing patches and features.

## Code of Conduct
This project and everyone participating in it is governed by the [Ciphr Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?
You can contribute in several ways, including reporting bugs, suggesting enhancements, and submitting pull requests for code changes.

### Reporting Bugs
If you find a bug, please create an issue in our [issue tracker](https://github.com/ciphr-dev/ciphr/issues), and be sure to include as much information as possible.

### Suggesting Enhancements
If you have an idea for an enhancement, please create an issue in the tracker. This is a great way to get feedback on your idea before you start working on it.

### Pull Requests
We welcome pull requests! Please follow these steps to make your contribution as smooth as possible:

1.  Fork the repository and create your branch from `main`.
2.  Set up the development environment using `devbox shell`.
3.  **Enable the shared Git hooks** by running this command in the repository root:
    ```bash
    git config core.hooksPath .githooks
    ```
    This is crucial for ensuring your commits meet our quality standards.
4.  Make your changes, ensuring you add tests for any new code.
5.  Ensure the test suite passes (`cargo test`).
6.  Format your code (`cargo fmt`) and check for linter warnings (`cargo clippy`).
7.  Commit your changes using a descriptive commit message that follows our [Conventional Commits](#conventional-commits) standard.
8.  Push your branch and submit a pull request, filling out the PR template.

## Development Workflow

### Conventional Commits
All commit messages must follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. This helps us automate versioning and changelog generation.

The format is:
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

-   **type**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
-   **scope**: `config`, `flags`, `logging`, `testing`, `ci`, `docs`, `security`, etc.

**Example:**
```
feat(config): add support for TOML configuration files
```

### Git Hooks
We use shared Git hooks located in the `.githooks/` directory to enforce quality gates automatically. Please enable them as described above.

-   **pre-commit**: Runs `cargo fmt`, `cargo clippy`, and `cargo test` to ensure code quality before a commit is made.
-   **commit-msg**: Validates that your commit message adheres to the Conventional Commits standard.
-   **pre-push**: Runs `cargo check` as a final quick verification before you push your code.

### Code Style
We follow the standard Rust style guidelines, enforced by `rustfmt`. The `pre-commit` hook will handle this automatically for you. 