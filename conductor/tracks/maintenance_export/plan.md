# Implementation Plan: Maintenance & Export

## Tasks
- [x] **API Implementation**: Add methods to `Client` in `src/api/mod.rs` for triggering backups and exporting configurations. [46de072]
- [x] **MCP Schema Update**: Define argument structs and tool definitions in `src/mcp/mod.rs`. [39a7a73]
- [x] **Handler Integration**: Map the new MCP methods in `McpServer::handle_method`. [39a7a73]
- [x] **Verification**: Add integration tests in `tests/api_maintenance_test.rs` and `tests/mcp_maintenance_test.rs`. [b2653d2]

[checkpoint: 7265ec9]
