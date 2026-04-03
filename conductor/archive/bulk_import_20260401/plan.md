# Implementation Plan - Bulk Import

This plan follows the project's TDD-based workflow.

## Phase 1: API Client Implementation [checkpoint: b1164cc]
1. [x] Create a new test file `tests/api_import_test.rs` with failing tests for `import_watches`. [0b00fed]
2. [x] Implement `import_watches` in `src/api/mod.rs`. [0b00fed]
3. [x] Verify tests pass in `tests/api_import_test.rs`. [0b00fed]

## Phase 2: MCP Tool Integration [checkpoint: a12394b]
4. [x] Add `ImportWatchesArgs` struct to `src/mcp/mod.rs`. [2b9e46d]
5. [x] Add `import_watches` to the `handle_method` match block in `src/mcp/mod.rs`. [2b9e46d]
6. [x] Add `import_watches` to the `tools/list` response in `src/mcp/mod.rs`. [2b9e46d]
7. [x] Create integration tests in `tests/mcp_import_test.rs` ensuring the tool is exposed and works. [2b9e46d]
8. [x] Verify all tests pass. [2b9e46d]
