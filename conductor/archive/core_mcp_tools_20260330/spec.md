# Specification - core_mcp_tools_20260330

## Goal
Implement a functional MCP server in Rust that allows AI models to interact with a local ChangeDetection.io instance.

## Scope
- Implementation of the core API client for ChangeDetection.io.
- Implementation of the following MCP tools:
    - `list_watches`
    - `get_watch_details`
    - `create_watch`
    - `delete_watch`
    - `trigger_check`
- Support for `stdio` transport.
- Comprehensive unit and integration testing.

## Technical Requirements
- Language: Rust
- Runtime: Tokio
- HTTP Client: Reqwest
- Serialization: Serde
- Testing: Cargo test, Testcontainers (optional for integration)

## Constraints
- Must follow the project's code style and product guidelines.
- Must achieve >80% code coverage.
