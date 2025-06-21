# ADR-001: Use Devbox for Development Environment Management
**Date**: 2025-06-19  
**Status**: accepted

#### Context
The project requires consistent development environments across Windows, macOS, and Linux platforms. Team members have different system configurations, and onboarding new contributors should be friction-free. The solution must be reliable, fast, and not require specialized knowledge of virtualization or containerization.

#### Decision
Adopt Devbox for development environment management, providing hermetic, reproducible environments based on Nix packages without requiring Docker or complex setup procedures.

#### Consequences
**Positive**:
- Consistent tool versions across all platforms and developers
- Fast environment setup with cached packages
- No virtualization overhead or Docker complexity
- Declarative environment configuration in version control
- Strong isolation preventing system configuration conflicts

**Negative**:
- Nix learning curve for advanced customization
- Potential cold-start delays when downloading packages
- Limited to tools available in Nix packages (though extensive)
- Debugging environment issues requires Nix knowledge 