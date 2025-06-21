# ADR-04: Adopt just Task Runner Over Make
**Date**: 2025-06-19  
**Status**: accepted

#### Context
Development workflow requires consistent task execution across platforms for testing, building, linting, and documentation generation. Make has platform compatibility issues and complex syntax. The solution must be simple for contributors to understand and extend.

#### Decision
Use just as the primary task runner for all development workflows, providing cross-platform compatibility with simple, readable syntax.

#### Consequences
**Positive**:
- Excellent cross-platform compatibility (Windows, macOS, Linux)
- Simple, readable syntax easier for contributors to understand
- Better error messages than Make
- Support for modern features like command-line arguments and dependencies
- Fast execution with minimal overhead

**Negative**:
- Additional dependency to install (though handled by Devbox)
- Less ubiquitous than Make, requiring documentation for contributors
- Smaller ecosystem of examples and tutorials
- Migration effort if switching to different task runner later 