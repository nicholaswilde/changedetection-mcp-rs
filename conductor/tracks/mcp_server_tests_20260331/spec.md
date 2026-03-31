# Specification: Implement Integration Tests for McpServer

## Overview
Implement a comprehensive suite of integration tests for the `McpServer` component to ensure its correctness in handling MCP tool calls and its integration with the `api::Client`.

## Functional Requirements
- **Method Mapping:** Verify that calling MCP methods (`list_watches`, `get_watch_details`, `create_watch`, `delete_watch`, `trigger_check`) correctly triggers the corresponding `api::Client` methods.
- **Error Handling:** Ensure that invalid parameters (e.g., missing `uuid`) or API errors are correctly handled and returned as MCP protocol errors.
- **Tool Definitions:** Verify that the `tools/list` method returns the correct tool definitions (names, descriptions, and schemas).
- **Full Lifecycle:** Test the JSON-RPC lifecycle, including initialization, method calls, and shutdown.

## Non-Functional Requirements
- **Test Isolation:** Use both `wiremock` for fast, predictable API mocking and `testcontainers-rs` for end-to-end verification against a real ChangeDetection.io instance (as per `tech-stack.md`).
- **Organization:** Locate the new tests in a dedicated `tests/mcp_server_test.rs` file.

## Acceptance Criteria
- A new test file `tests/mcp_server_test.rs` is created.
- Tests cover all supported MCP methods.
- Tests cover error cases (e.g., missing parameters, API errors).
- `cargo test` passes for all tests.
- Coverage for `src/mcp/mod.rs` is improved.

## Out of Scope
- Tests for other transports (e.g., HTTP/SSE) unless already implemented.
- Performance or load testing.
