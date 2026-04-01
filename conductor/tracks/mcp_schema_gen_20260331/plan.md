# Implementation Plan

1. [x] **Add Schemars Dependency:** f053109
   - Add `schemars` to the `dependencies` section in `Cargo.toml`.

2. [ ] **Derive JsonSchema for Parameter Structs:**
   - Update all MCP tool parameter structs to derive `JsonSchema`.

3. [ ] **Implement Schema Generation Logic:**
   - Implement a mechanism (helper function or trait) to generate `serde_json::Value` schemas from `JsonSchema` types.

4. [ ] **Refactor Tool Registration:**
   - Update `src/mcp/mod.rs` to use the automated schema generation during tool registration.

5. [ ] **Consistency Check:**
   - Add a test case to verify that the generated schemas are consistent with the expected MCP tool format.
