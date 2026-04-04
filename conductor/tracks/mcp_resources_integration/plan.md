# Implementation Plan: MCP Integration Features (Resources)

## Tasks
- [x] **Resource Definition**: Define the resource logic in `src/mcp/mod.rs` to handle URI resolution for snapshots and the OpenAPI spec. [12681e2]
- [x] **Capabilities Update**: Ensure `ServerCapabilities` includes resource support. [12681e2]
- [x] **Handler Implementation**: Map the resource URIs to the corresponding `Client` calls. [12681e2]
- [x] **Verification**: Add integration tests to verify resource retrieval via MCP. [12681e2]
