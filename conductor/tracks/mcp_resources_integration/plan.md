# Implementation Plan: MCP Integration Features (Resources)

## Tasks
- [ ] **Resource Definition**: Define the resource logic in `src/mcp/mod.rs` to handle URI resolution for snapshots and the OpenAPI spec.
- [ ] **Capabilities Update**: Ensure `ServerCapabilities` includes resource support.
- [ ] **Handler Implementation**: Map the resource URIs to the corresponding `Client` calls.
- [ ] **Verification**: Add integration tests to verify resource retrieval via MCP.
