# Implementation Plan

1. [x] **API Client Extension:** (0a683b4)
   - Add `get_system_info()` to the `Client` in `src/api/mod.rs`.

2. [x] **API Client Tests:** (0a683b4)
   - Write tests in `tests/api_client_test.rs` using `wiremock`.

3. [x] **MCP Tool Definitions:** (db0a181)
   - Add `get_system_info` tool and schema.
   - Add handler to `src/mcp/mod.rs`.

4. [x] **MCP Server Tests:** (47bc4eb)
   - Add integration tests for the `get_system_info` MCP tool.