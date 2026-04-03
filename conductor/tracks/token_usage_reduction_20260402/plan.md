# Implementation Plan - MCP Token Optimization

## Phase 1: Research & Schema Definition [checkpoint: c226695]
- [x] **Task: Map Existing Tools to Categories** 2db5b37
    - [ ] Create a mapping of all 39 current tools to the 5 proposed category tools (`watch_ops`, `tag_ops`, `notification_ops`, `history_ops`, `system_ops`).
- [x] **Task: Define Consolidated Argument Structs** 555ff03
    - [ ] Design the input schemas for each category tool, including the `action` enum and shared `pagination`/`fields` parameters.
- [x] **Task: Conductor - User Manual Verification 'Phase 1: Research & Schema Definition' (Protocol in workflow.md)** c226695

## Phase 2: Core Implementation
- [x] **Task: Implement Pagination & Field Selection Helpers** 6c4ca3e
    - [ ] Write utility functions in `src/mcp/mod.rs` or a new module to handle JSON field filtering and pagination of HashMaps/Vecs.
- [x] **Task: Define New Category Tools** d9d3a27
    - [ ] Update `src/mcp/mod.rs` with the new tool definitions and schemas.
- [x] **Task: Implement Consolidated Handler** d9d3a27
    - [ ] Refactor `McpServer::handle_method` to dispatch the new category tools to the existing `Client` methods.
- [x] **Task: Conductor - User Manual Verification 'Phase 2: Core Implementation' (Protocol in workflow.md)** d9d3a27

## Phase 3: Testing & Migration
- [ ] **Task: Update Unit Tests**
    - [ ] Update tests in `tests/mcp_server_test.rs` and other MCP-related test files to use the new category tools.
- [ ] **Task: Add Optimization Tests**
    - [ ] Write new tests verifying that pagination and field selection correctly reduce the response size.
- [ ] **Task: Deprecate/Remove Old Tools**
    - [ ] Remove the individual tool definitions and handlers from `src/mcp/mod.rs`.
- [ ] **Task: Conductor - User Manual Verification 'Phase 3: Testing & Migration' (Protocol in workflow.md)**

## Phase 4: Final Polish & Documentation
- [ ] **Task: Optimize Descriptions**
    - [ ] Perform a final audit of all tool and parameter descriptions to ensure they are as token-efficient as possible.
- [ ] **Task: Update Project Documentation**
    - [ ] Update any documentation that references the individual MCP tools.
- [ ] **Task: Conductor - User Manual Verification 'Phase 4: Final Polish & Documentation' (Protocol in workflow.md)**
