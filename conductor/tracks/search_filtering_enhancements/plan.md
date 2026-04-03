# Implementation Plan: Search & Filtering Enhancements

## Tasks
- [x] **API Implementation**: Add methods to `Client` in `src/api/mod.rs` for filtering watches by error and processor. ba63300
- [x] **MCP Schema Update**: Define argument structs and tool definitions in `src/mcp/mod.rs`. f2398ff
- [x] **Handler Integration**: Map the new MCP methods in `McpServer::handle_method`. 2c2c7a5
- [ ] **Verification**: Add integration tests in `tests/api_search_filtering_test.rs` and `tests/mcp_search_filtering_test.rs`.
