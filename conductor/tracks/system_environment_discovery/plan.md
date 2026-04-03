# Implementation Plan: System & Environment Discovery

## Tasks
- [x] **API Implementation**: Add methods to `Client` in `src/api/mod.rs` for fetching lists of fetchers, proxies, and global settings.
- [x] **MCP Schema Update**: Define argument structs and tool definitions in `src/mcp/mod.rs`.
- [x] **Handler Integration**: Map the new MCP methods in `McpServer::handle_method`.
- [x] **Verification**: Add integration tests in `tests/api_system_discovery_test.rs` and `tests/mcp_system_discovery_test.rs`.
