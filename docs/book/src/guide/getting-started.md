# Getting Started

Welcome to Ciphr! This guide provides everything you need to set up your development environment and start contributing. Our goal is to make the process as fast and simple as possible.

## Quick Start: One-Command Setup

The entire development environment is managed by [Devbox](https://www.jetpack.io/devbox/), which ensures you have the exact same tool versions as every other contributor.

1.  **Install Devbox**:
    ```bash
    curl -fsSL https://get.jetpack.io/devbox | bash
    ```

2.  **Clone the Repository**:
    ```bash
    git clone https://github.com/Ciphr8-labs/ciphr.git
    cd ciphr
    ```

3.  **Launch the Environment**:
    ```bash
    devbox shell
    ```
    The first time you run this, Devbox will download and install all the necessary tools via Nix. This may take a few minutes. Subsequent launches will be nearly instant. Once inside the shell, you have access to the complete development toolchain.

## Environment Validation

To ensure your environment is configured correctly, run the `setup` command:

```bash
just setup
```

This script checks that all required tools are present, dependencies are resolved, and everything is ready for development. If any checks fail, it will provide troubleshooting guidance.

## Potential Issues & Solutions

### Slow Initial Download
**Problem**: The first time you run `devbox shell`, it may need to download several hundred megabytes of tools, which can be slow on some networks.

**Solution**:
- **Be Patient**: This is a one-time cost. Subsequent runs will be fast.
- **Manual Installation**: If Devbox is not a viable option, we provide a (community-supported) manual setup guide. *(Note: This is not yet written, but we can add it if needed).*

### Platform-Specific Problems
**Problem**: You may encounter issues specific to your operating system, especially on Windows.

**Solution**:
- We have a dedicated [Windows Setup Guide](./windows-setup.md) for common issues.
- Ensure your project path does not contain spaces or special characters.
- When in doubt, using WSL2 on Windows provides the most seamless experience.

### Version Conflicts
**Problem**: You have a different version of Rust or Node.js installed on your system.

**Solution**: This is not a problem! Devbox creates an isolated, "hermetic" environment. The tools inside the `devbox shell` are separate from your system-wide tools, which prevents any version conflicts.

## Development Workflow

With your environment ready, you can run common development tasks using the `just` command runner. See the [Development Workflow](development-workflow.md) page for a full list of available commands. A few key commands are:

- `just test`: Run all tests.
- `just build`: Build the project.
- `just fmt`: Format all code.
- `just clippy`: Lint the code. 