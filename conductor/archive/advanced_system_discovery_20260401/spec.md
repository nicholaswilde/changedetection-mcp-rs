# Specification - Advanced System Discovery

## Functional Requirements
- Implement `get_full_spec` in `api/mod.rs` to fetch from `/api/v1/full-spec`.
- Expose `get_full_spec` as an MCP tool in `mcp/mod.rs`.
- Handle the YAML response and return it as a string or parsed object.
- Provide a way to discover available fields added by plugins.

## Technical Requirements
- Add `get_full_spec` method to `Client` struct.
- Update `McpServer::list_tools` to include the new tool.
- Update `McpServer::handle_method` to route the request.
- Add unit tests for successful and failed retrieval.
- Add live test to verify actual spec format.
