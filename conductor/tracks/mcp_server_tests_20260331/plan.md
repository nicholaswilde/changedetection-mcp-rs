# Implementation Plan: McpServer Integration Tests

## Phase 1: Test Scaffolding and Wiremock Tests
- [ ] Task: Create `tests/mcp_server_test.rs` and set up the `wiremock` infrastructure.
- [ ] Task: Write Tests: Implement integration tests for the `ServerHandler` implementation using a mocked `Client` and `wiremock`.
    - [ ] `list_watches`
    - [ ] `get_watch_details`
    - [ ] `create_watch`
    - [ ] `delete_watch`
    - [ ] `trigger_check`
- [ ] Task: Write Tests: Implement tests for error handling (e.g., missing parameters, API errors).
- [ ] Task: Write Tests: Implement tests for the `tools/list` method and verify tool definitions.
- [ ] Task: Conductor - User Manual Verification 'Phase 1' (Protocol in workflow.md)

## Phase 2: Testcontainers Integration (Optional/Advanced)
- [ ] Task: Research and set up `testcontainers-rs` with a ChangeDetection.io image (if feasible within the environment).
- [ ] Task: Write Tests: Implement a test case that verifies the full lifecycle (initialization, one method call, shutdown) using a real container.
- [ ] Task: Conductor - User Manual Verification 'Phase 2' (Protocol in workflow.md)

## Phase 3: Final Verification and Coverage
- [ ] Task: Run all tests and verify that `cargo test` passes.
- [ ] Task: Verify: Ensure code coverage for `src/mcp/mod.rs` has increased and meets the project's goals (>80%).
- [ ] Task: Conductor - User Manual Verification 'Phase 3' (Protocol in workflow.md)
