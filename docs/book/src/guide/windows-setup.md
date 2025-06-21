# Windows Setup Guide

While Devbox aims to provide a consistent experience across all platforms, Windows can sometimes present unique challenges due to its different shell environments and file system behavior.

## Recommended Environment: WSL2 or Git Bash

For the smoothest experience on Windows, we **strongly recommend** using one of the following:

1.  **Windows Subsystem for Linux (WSL2)**: This provides a full Linux environment directly within Windows and is the most reliable way to avoid platform-specific issues.
2.  **Git Bash**: This provides a Bash emulation layer that is generally more compatible with the scripts and tools used in this project than PowerShell or Command Prompt.

## Common Issues & Fixes

### 1. Paths with Spaces

**Error**: You might see errors related to "command not found" or "invalid path".

**Cause**: Many shell scripts and tools do not handle spaces in file paths well. For example, a path like `C:\Users\Your Name\Projects\ciphr` can cause problems.

**Fix**: Clone the project into a path that does not contain any spaces (e.g., `C:\Projects\ciphr`).

### 2. Line Endings

**Error**: Scripts fail with strange syntax errors.

**Cause**: Git on Windows might be configured to check out files with Windows-style line endings (`CRLF`) instead of Unix-style (`LF`), which can break shell scripts.

**Fix**: Configure Git to not change line endings for this repository:
```bash
git config core.autocrlf false
```
You may need to re-clone the repository after changing this setting.

### 3. Permission Errors

**Error**: You get "Permission denied" when running scripts.

**Cause**: File permissions might not be set correctly.

**Fix**: Ensure that scripts are executable. This is less common when using WSL2 or Git Bash but can happen.
```bash
chmod +x ./scripts/validate-environment.sh
```

If you encounter an issue not listed here, please [ask for help in GitHub Discussions](https://github.com/Ciphr8-labs/ciphr/discussions). 