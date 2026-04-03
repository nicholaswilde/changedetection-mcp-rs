# Implementation Plan - Explicit State Management

This plan follows the project's TDD-based workflow.

## Phase 1: API Client Support
1. [x] Create a new test file `tests/api_state_management_test.rs` with failing tests for each tool. [815b968]
2. [x] Research specific endpoints for pause/mute or implement via `update_watch` in `src/api/mod.rs`. [815b968]
3. [x] Implement necessary methods in `src/api/mod.rs`. [815b968]
4. [x] Verify tests pass in `tests/api_state_management_test.rs`. [815b968]

## Phase 2: MCP Tool Integration
5. [ ] Add `PauseWatchArgs` (and others as needed) to `src/mcp/mod.rs`. [ ]
6. [ ] Add all four tools to the `handle_method` match block in `src/mcp/mod.rs`. [ ]
7. [ ] Add all four tools to the `tools/list` response in `src/mcp/mod.rs`. [ ]
8. [ ] Create integration tests in `tests/mcp_state_management_test.rs`. [ ]
9. [ ] Verify all tests pass. [ ]
