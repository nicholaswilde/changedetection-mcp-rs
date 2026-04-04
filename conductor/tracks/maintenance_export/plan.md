# Implementation Plan: Maintenance & Export

## Tasks
- [x] **API Implementation**: Add methods to `Client` in `src/api/mod.rs` for triggering backups and exporting configurations. [46de072]
- [ ] **MCP Schema Update**: Define argument structs and tool definitions in `src/mcp/mod.rs`.
- [ ] **Handler Integration**: Map the new MCP methods in `McpServer::handle_method`.
- [ ] **Verification**: Add integration tests in `tests/api_maintenance_test.rs` and `tests/mcp_maintenance_test.rs`.
