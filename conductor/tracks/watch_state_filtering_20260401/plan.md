# Implementation Plan - Watch Filtering by State

This plan follows the project's TDD-based workflow.

## Phase 1: Local Filtering Implementation
1. [x] Update `ListWatchesArgs` in `src/mcp/mod.rs` to include the optional `state` field. [b543c8d]
2. [x] Create a new test file `tests/mcp_watch_filtering_test.rs` with failing tests for state filtering. [b543c8d]
3. [x] Research if watch state is returned in the API response or if it requires additional processing. [b543c8d]
4. [x] Implement filtering logic in the `handle_method` match block for `list_watches` in `src/mcp/mod.rs`. [b543c8d]
5. [x] Verify tests pass in `tests/mcp_watch_filtering_test.rs`. [b543c8d]

## Phase 2: Documentation and Cleanup
6. [ ] Update the `tools/list` description for `list_watches` to document the new `state` parameter. [ ]
7. [ ] Ensure all tests pass. [ ]
