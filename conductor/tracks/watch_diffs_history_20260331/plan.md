# Implementation Plan

1. [ ] **API Client Extension:**
   - Add `get_watch_history(uuid: &str)` to the `Client` in `src/api/mod.rs`.
   - Add `get_watch_diff(uuid: &str, from: &str, to: &str)` to the `Client` in `src/api/mod.rs`.
   
2. [ ] **API Client Tests:**
   - Update `tests/api_client_test.rs` with mock endpoints for these new methods.

3. [ ] **MCP Tool Definitions:**
   - Add `get_watch_history` tool and schema.
   - Add `get_watch_diff` tool and schema.
   - Wire them into `handle_method` in `src/mcp/mod.rs`.

4. [ ] **MCP Server Tests:**
   - Write integration tests in `tests/mcp_server_test.rs` to verify the new tools.