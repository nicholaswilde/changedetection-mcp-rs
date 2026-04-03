# Implementation Plan: Snapshot & History Management

## Tasks
- [ ] **API Implementation**: Add methods to `Client` in `src/api/mod.rs` for snapshot deletion, re-indexing, and metadata retrieval.
- [ ] **MCP Schema Update**: Define argument structs and tool definitions in `src/mcp/mod.rs`.
- [ ] **Handler Integration**: Map the new MCP methods in `McpServer::handle_method`.
- [ ] **Verification**: Add integration tests in `tests/api_snapshot_management_test.rs` and `tests/mcp_snapshot_management_test.rs`.
