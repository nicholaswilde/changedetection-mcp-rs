# Implementation Plan

1. [ ] **API Client Extension:**
   - Add `list_tags()`, `create_tag(name: &str)`, etc. to the `Client` in `src/api/mod.rs`.

2. [ ] **API Client Tests:**
   - Write tests in `tests/api_client_test.rs` using `wiremock`.

3. [ ] **MCP Tool Definitions:**
   - Add `list_tags`, `create_tag`, etc.
   - Add handler to `src/mcp/mod.rs`.

4. [ ] **MCP Server Tests:**
   - Add integration tests for the tag management MCP tools.