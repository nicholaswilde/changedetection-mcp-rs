# Implementation Plan [checkpoint: b8c1d4f]

1. [x] **API Client Extension:** (9216d32)
   - Add `update_watch(uuid: &str, payload: serde_json::Value)` to the `Client` in `src/api/mod.rs`.
   
2. [x] **API Client Tests:** (9216d32)
   - Write tests in `tests/api_client_test.rs` using `wiremock` to verify the `PUT` request.

3. [x] **MCP Tool Definitions:** (cf64547)
   - Add `update_watch` tool definition with a flexible schema for updating properties.
   - Implement handler in `src/mcp/mod.rs`.

4. [x] **MCP Server Tests:** (cf64547)
   - Add integration tests for the `update_watch` MCP tool.