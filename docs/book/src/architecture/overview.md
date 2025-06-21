# Architecture Overview

The Ciphr development environment and infrastructure are designed with a **Layered Architecture** to ensure a clear separation of concerns. This design promotes maintainability, scalability, and developer productivity.

The layers are organized as follows:

1.  **Infrastructure Layer**: This is the foundation, containing Devbox for environment management, the CI/CD pipeline, and other external tools that the project relies on.
2.  **Configuration Layer**: This layer manages all configuration, including environment settings, tool configurations, and the feature flag system. It provides a consistent interface for all other components to access configuration data.
3.  **Development Tools Layer**: This layer includes all the tools and frameworks that developers interact with daily, such as the testing framework, linters, and documentation generators.
4.  **Application Interface Layer**: This is the top layer that developers use to interact with the system. It includes the CLI, the `justfile` for running tasks, and the overall development workflows.

This layered approach ensures that changes in one area (e.g., the CI/CD provider) have minimal impact on other areas (e.g., the testing framework), making the system more robust and easier to evolve over time. 