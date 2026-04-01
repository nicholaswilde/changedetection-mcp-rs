# Implementation Plan

1. [ ] **API Client Extension:**
   - Add `search_watches(query: &str)` to the `Client` in `src/api/mod.rs`.

2. [ ] **API Client Tests:**
   - Write tests in `tests/api_client_test.rs` using `wiremock`.

3. [ ] **MCP Tool Definitions:**
   - Add `search_watches` tool and schema.
   - Add handler to `src/mcp/mod.rs`.

4. [ ] **MCP Server Tests:**
   - Add integration tests for the `search_watches` MCP tool.