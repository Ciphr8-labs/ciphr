# Environment Setup

This project uses [Devbox](https://www.jetpack.io/devbox/) to manage development environments. This approach ensures a consistent and reproducible setup for all contributors, regardless of their operating system.

## Prerequisites

Before you begin, you must have the following tools installed on your system:

1.  **Nix Package Manager**: Devbox is built on top of Nix. Follow the [official Nix installation guide](https://nixos.org/download.html).
2.  **Devbox**: Follow the [official Devbox installation guide](https://www.jetpack.io/devbox/docs/installing_devbox/).

## One-Command Setup

Once you have cloned the repository and installed the prerequisites, you can set up the complete development environment with a single command from the project's root directory:

```bash
devbox shell
```

This command will:
1.  Read the `devbox.json` file.
2.  Download and install all the specified tools (like Rust, just, etc.) into an isolated, project-specific environment.
3.  Run the `init_hook`, which automatically initializes the Rust toolchain and validates the environment.

You are now in a shell with all the necessary tools to build, test, and contribute to Ciphr.

## Validating the Environment

The environment is validated automatically every time you run `devbox shell`. You can also run the validation script manually at any time:

```bash
./scripts/validate-environment.sh
```

If all checks pass, you are ready for development.

## Troubleshooting

### General
- **Slow initial setup**: The first time you run `devbox shell`, it may take several minutes to download the Nix packages. Subsequent runs will be much faster due to caching.
- **`command not found`**: Ensure you are inside the Devbox shell (`devbox shell`) before running any project-specific commands like `cargo` or `just`.

### Platform-Specific

**macOS:**
- If you encounter issues with the Nix installation, ensure you have the command-line tools for Xcode installed (`xcode-select --install`).

**Windows:**
- It is highly recommended to use WSL2 (Windows Subsystem for Linux) for the best experience. Devbox and Nix work seamlessly within a WSL2 environment.

**Linux:**
- Ensure your distribution's package manager has `curl`, `git`, and `build-essential` (or an equivalent package group) installed before attempting to install Nix. 