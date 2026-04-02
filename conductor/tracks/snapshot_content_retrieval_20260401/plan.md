# Implementation Plan - Snapshot Content Retrieval

This plan follows the project's TDD-based workflow.

## Phase 1: API Client Implementation
1. [x] Create a new test file `tests/api_snapshot_content_test.rs` with failing tests for `get_snapshot_content`. [4dad0e7]
2. [x] Implement `get_snapshot_content` in `src/api/mod.rs`. [4dad0e7]
3. [x] Verify tests pass in `tests/api_snapshot_content_test.rs`. [4dad0e7]

## Phase 2: MCP Tool Integration
4. [ ] Add `GetSnapshotContentArgs` struct to `src/mcp/mod.rs`. [ ]
5. [ ] Add `get_snapshot_content` to the `handle_method` match block in `src/mcp/mod.rs`. [ ]
6. [ ] Add `get_snapshot_content` to the `tools/list` response in `src/mcp/mod.rs`. [ ]
7. [ ] Create integration tests in `tests/mcp_snapshot_content_test.rs` ensuring the tool is exposed and works. [ ]
8. [ ] Verify all tests pass. [ ]
