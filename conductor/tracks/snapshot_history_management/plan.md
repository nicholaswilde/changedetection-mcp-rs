# Implementation Plan: Snapshot & History Management

## Tasks
- [x] **API Implementation**: Add methods to `Client` in `src/api/mod.rs` for metadata retrieval, bulk history listing, and history limit setting. d3b079c
- [x] **MCP Schema Update**: Define argument structs and tool definitions in `src/mcp/mod.rs`. 4e16b64
- [x] **Handler Integration**: Map the new MCP methods in `McpServer::handle_method`. 11deb88
- [x] **Verification**: Add integration tests in `tests/api_snapshot_management_test.rs` and `tests/mcp_snapshot_management_test.rs`. 11deb88
