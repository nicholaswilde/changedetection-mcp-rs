# Tech Stack - changedetection-mcp-rs

## Core Language & Protocol
- **Rust**: Primary language for performance and safety.
- **Model Context Protocol (MCP)**: For interacting with LLMs.
- **Transports**: Supports both `stdio` (local) and `HTTP/SSE` (remote).

## Build & Development Tools
- Task Runner: `go-task` (via `Taskfile.yml`) for build, test, and deployment automation.
- Cross-Compilation: `cross` for building `amd64`, `arm64`.
- Version Control: `git` for configuration backup and management.
- Package Manager: `cargo` (Rust standard).
- MCP Testing: `mcp-inspector` for verifying MCP tool definitions and responses.

## Configuration Management
- Formats: Optimized support for **TOML** (YAML/JSON disabled for size).
- Hierarchy: Configuration via CLI arguments, environment variables, and config files (e.g., `config.toml`).
- Multi-Instance Support: Built-in logic for managing multiple ChangeDetection.io instances.

## Security & Secrets
- **Secrets Management**: Secure credential resolution from environment variables and configuration files.
- **Authentication**: Bearer Token for HTTP transport security; API Key support for ChangeDetection.io backend communication.

## Testing & Quality Assurance
- **Code Coverage**: `cargo-llvm-cov` for detailed analysis.
- **Integration Testing**: Automated end-to-end testing using `testcontainers-rs` and real ChangeDetection.io Docker instances.
- **CI/CD**: Integration with GitHub Actions and Coveralls.io.

## Containerization
- **Docker**: Standard `Dockerfile` for containerized deployment.
- **Orchestration**: `compose.yaml` for local development and integration testing.

## Principal Rust Dependencies (Inferred)
- **tokio**: Asynchronous runtime.
- **tokio-retry**: Exponential backoff and retry strategy for async tasks.
- **reqwest**: For communicating with the ChangeDetection.io REST API (v0.13.2).
- **reqwest-middleware**: Middleware support for reqwest (v0.5.1).
- **reqwest-retry**: Retry strategy for reqwest (v0.9.1).
- **http-cache-reqwest**: HTTP caching for reqwest (v1.0.0-alpha.5).
- **schemars**: Automated JSON schema generation from Rust types (v1.2.1).
- **serde**, **serde_json**, **toml**: For configuration and API parsing.
- **clap**: For robust CLI argument parsing.
- **tracing** / **log**: For configurable logging levels.
- **anyhow**: Flexible error handling.
- **thiserror**: Custom error types with derive macros.
- **axum**: Web framework for HTTP/SSE transport (if implemented).
- **config**: Hierarchical configuration management.
- **dashmap**: Concurrent associative array.
- **futures**: Utilities for asynchronous programming.
- **testcontainers**: Programmatic Docker lifecycle management for tests.
- **testcontainers-modules**: Ready-to-use Docker images for testcontainers.
- **async-trait**: Support for asynchronous functions in traits.
- **uuid**: Unique identifier generation.
- **url**: URL parsing and manipulation.
- **async-recursion**: Procedural macro for recursive async functions.
