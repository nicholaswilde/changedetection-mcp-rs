# Implementation Plan: McpServer Integration Tests

## Phase 1: Test Scaffolding and Wiremock Tests [checkpoint: 88cefaf]
- [x] Task: Create `tests/mcp_server_test.rs` and set up the `wiremock` infrastructure. ef25b57
- [x] Task: Write Tests: Implement integration tests for the `ServerHandler` implementation using a mocked `Client` and `wiremock`. f9a058b
    - [x] `list_watches`
    - [x] `get_watch_details`
    - [x] `create_watch`
    - [x] `delete_watch`
    - [x] `trigger_check`
- [x] Task: Write Tests: Implement tests for error handling (e.g., missing parameters, API errors). 2a885dc
- [x] Task: Write Tests: Implement tests for the `tools/list` method and verify tool definitions. f9a058b
- [x] Task: Conductor - User Manual Verification 'Phase 1' (Protocol in workflow.md) 2a885dc


## Phase 2: Testcontainers Integration (Optional/Advanced) [skipped]
- [ ] Task: Research and set up `testcontainers-rs` with a ChangeDetection.io image (if feasible within the environment).
- [ ] Task: Write Tests: Implement a test case that verifies the full lifecycle (initialization, one method call, shutdown) using a real container.
- [ ] Task: Conductor - User Manual Verification 'Phase 2' (Protocol in workflow.md)

## Phase 3: Final Verification and Coverage [checkpoint: 30e98cb]
- [x] Task: Run all tests and verify that `cargo test` passes. 2a885dc
- [x] Task: Verify: Ensure code coverage for `src/mcp/mod.rs` has increased and meets the project's goals (>80%). 2a885dc
- [x] Task: Conductor - User Manual Verification 'Phase 3' (Protocol in workflow.md) 2a885dc
