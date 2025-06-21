The Happy Path: New Contributor Setup
Step 1: Discovery and Initial Setup (2-3 minutes)
bash# Contributor finds the project on GitHub and wants to contribute
git clone https://github.com/ciphr-org/ciphr.git
cd ciphr

# First thing they do - read the README
cat README.md
What they see:
markdown# Ciphr - Plain Text Accounting

## Quick Start for Contributors

1. Install Devbox: `curl -fsSL https://get.jetpack.io/devbox | bash`
2. Enter development environment: `devbox shell`
3. Validate setup: `just setup`
4. Run tests: `just test`
5. See all commands: `just --list`

Need help? Check CONTRIBUTING.md or ask in Discussions!
Step 2: Install Devbox (1-5 minutes, first time only)
bash# If they don't have devbox installed
curl -fsSL https://get.jetpack.io/devbox | bash

# Output they'd see:
# Installing Devbox...
# Devbox installed successfully!
# Add to PATH: export PATH="$HOME/.local/bin:$PATH"
Potential friction: This requires shell restart or PATH update, which might confuse some users.
Step 3: Enter Development Environment (30 seconds - 5 minutes)
bashdevbox shell
What they see (first time):
Starting a devbox shell...
Installing packages from devbox.json:
  ✓ rust@latest (downloading... 45MB)
  ✓ nodejs@18 (downloading... 23MB)  
  ✓ git@latest (cached)
  ✓ just@latest (downloading... 2MB)
  ✓ cargo-watch@latest (downloading... 8MB)
  ✓ cargo-audit@latest (downloading... 12MB)
  ✓ cargo-tarpaulin@latest (downloading... 15MB)
  ✓ mdbook@latest (downloading... 6MB)

Ciphr development environment ready! 🚀
rust 1.75.0
just 1.14.0

Welcome to Ciphr development! Run 'just --list' to see available commands.
Subsequent times (cached):
Starting a devbox shell...
Ciphr development environment ready! 🚀
rust 1.75.0
just 1.14.0
Step 4: Validate Setup (10-30 seconds)
bashjust setup
What they see:
🔧 Validating development environment...
✓ Rust toolchain available
✓ Git configuration valid
✓ Project dependencies resolved
✓ Test environment functional
✓ Linting tools ready
✓ Documentation tools available

🎉 Development environment ready! Here's what you can do:

Development Commands:
  just dev     - Start development mode with file watching
  just test    - Run the full test suite
  just lint    - Check code style and quality
  just docs    - Generate and serve documentation

Setup complete in 12.3s
Step 5: Explore Available Commands (10 seconds)
bashjust --list
What they see:
Available commands:
  setup          Validate development environment setup
  dev            Start development mode with auto-reload
  test           Run all tests (unit, integration, property-based)
  test-unit      Run only unit tests
  test-integration Run only integration tests  
  lint           Run clippy and formatting checks
  lint-fix       Automatically fix linting issues
  docs           Build and serve documentation locally
  docs-build     Build documentation without serving
  release        Create a new release (maintainers only)
  clean          Clean build artifacts and caches
  audit          Check for security vulnerabilities
  bench          Run performance benchmarks
Step 6: Make Changes and Test (ongoing)
bash# Start development mode
just dev

# In another terminal, make changes to code
# The watcher automatically rebuilds and re-runs tests

# Before committing, run full validation
just test
just lint
Real-World Friction Points
Issue 1: Slow Initial Download
Scenario: Contributor on slow internet or behind corporate firewall
bashdevbox shell
# Hangs for 10+ minutes downloading packages
Resolution: The design includes fallback instructions:
bash# If devbox is too slow, fallback manual setup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
npm install -g just
# ... etc (documented in CONTRIBUTING.md)
Issue 2: Platform-Specific Problems
Scenario: Windows user hits path issues or permission problems
Error: Failed to create devbox environment
Path contains spaces or special characters
Resolution: Clear error messages with Windows-specific guidance:
❌ Setup failed on Windows

Common fixes:
1. Ensure no spaces in your project path
2. Run from Git Bash or WSL2 (recommended)
3. See Windows setup guide: docs/windows-setup.md
4. Ask for help in GitHub Discussions with your error details
Issue 3: Version Conflicts
Scenario: They already have conflicting Rust/Node versions
Warning: Existing Rust installation detected
Devbox will use its own version in this shell
Your system Rust: 1.65.0
Devbox Rust: 1.75.0
Resolution: Clear isolation messaging so they understand this is intentional.
Time Investment Reality Check
First-time setup (including devbox install): 5-15 minutes

Devbox installation: 1-3 minutes
Package downloads: 2-10 minutes (depending on internet)
Environment validation: 10-30 seconds
Understanding workflow: 2-5 minutes reading docs

Subsequent sessions: 10-30 seconds

Cached devbox shell startup
Immediate productivity

What Makes This Experience Good

Single Entry Point: devbox shell gets them everything they need
Clear Validation: just setup confirms everything works before they start
Discoverable Commands: just --list shows all available tools
Automated Quality: Tests and lints run automatically, preventing broken PRs
Helpful Errors: When things go wrong, error messages include next steps

What Could Still Go Wrong

Devbox Adoption: Contributors unfamiliar with devbox might be hesitant
Download Size: Initial package downloads could be 100MB+
Corporate Networks: Firewalls might block Nix package downloads
Windows Compatibility: Despite testing, Windows edge cases always exist
Cognitive Load: New contributors need to learn just commands on top of project specifics

The PR Workflow
Once set up, contributing becomes smooth:
bash# Create feature branch
git checkout -b fix-budget-calculation

# Make changes with live reload
just dev

# Validate before commit
just test && just lint

# Commit (git hooks validate commit message format)
git commit -m "fix: correct budget calculation rounding"

# Push and create PR
git push origin fix-budget-calculation
The CI pipeline automatically runs the same just test and just lint commands, so contributors get the same feedback locally that the CI will provide.