# Implementation Plan: Advanced Watch Configuration

## Tasks
- [x] **API Implementation**: Add specialized methods to `Client` in `src/api/mod.rs` for selectors, fetchers, and notifications. 82a0fc9
- [ ] **MCP Schema Update**: Define argument structs and tool definitions in `src/mcp/mod.rs`.
- [ ] **Handler Integration**: Map the new MCP methods to the corresponding `Client` methods in `McpServer::handle_method`.
- [ ] **Verification**: Add integration tests in `tests/api_watch_config_test.rs` and `tests/mcp_watch_config_test.rs`.
