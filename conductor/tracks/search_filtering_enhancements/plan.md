# Implementation Plan: Search & Filtering Enhancements

## Tasks
- [ ] **API Implementation**: Add methods to `Client` in `src/api/mod.rs` for filtering watches by error and processor.
- [ ] **MCP Schema Update**: Define argument structs and tool definitions in `src/mcp/mod.rs`.
- [ ] **Handler Integration**: Map the new MCP methods in `McpServer::handle_method`.
- [ ] **Verification**: Add integration tests in `tests/api_search_filtering_test.rs` and `tests/mcp_search_filtering_test.rs`.
