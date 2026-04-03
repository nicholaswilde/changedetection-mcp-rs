# Implementation Plan - Visual Snapshot Capture

This plan follows the project's TDD-based workflow.

## Phase 1: API Client Implementation
1. [x] Create a new test file `tests/api_screenshot_test.rs` with failing tests for `get_watch_screenshot`. [edd5774]
2. [x] Implement `get_watch_screenshot` in `src/api/mod.rs` (likely returning bytes or a base64 string). [edd5774]
3. [x] Verify tests pass in `tests/api_screenshot_test.rs`. [edd5774]

## Phase 2: MCP Tool Integration
4. [ ] Add `GetWatchScreenshotArgs` struct to `src/mcp/mod.rs`. [ ]
5. [ ] Add `get_watch_screenshot` to the `handle_method` match block in `src/mcp/mod.rs`. [ ]
6. [ ] Add `get_watch_screenshot` to the `tools/list` response in `src/mcp/mod.rs`. [ ]
7. [ ] Create integration tests in `tests/mcp_screenshot_test.rs`. [ ]
8. [ ] Verify all tests pass. [ ]
