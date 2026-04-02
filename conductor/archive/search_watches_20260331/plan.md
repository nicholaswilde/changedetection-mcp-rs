# Implementation Plan [checkpoint: 17722e1]

1. [x] **API Client Extension:** (df5a90d)
   - Add `search_watches(query: &str)` to the `Client` in `src/api/mod.rs`.

2. [x] **API Client Tests:** (df5a90d)
   - Write tests in `tests/api_client_test.rs` using `wiremock`.

3. [x] **MCP Tool Definitions:** (2cda2b4)
   - Add `search_watches` tool and schema.
   - Add handler to `src/mcp/mod.rs`.

4. [x] **MCP Server Tests:** (2cda2b4)
   - Add integration tests for the `search_watches` MCP tool.