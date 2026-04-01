# Implementation Plan

1. [x] **Add Schemars Dependency:** f053109
   - Add `schemars` to the `dependencies` section in `Cargo.toml`.

2. [x] **Derive JsonSchema for Parameter Structs:** 7412080
   - Update all MCP tool parameter structs to derive `JsonSchema`.

3. [x] **Implement Schema Generation Logic:** 7412080
   - Implement a mechanism (helper function or trait) to generate `serde_json::Value` schemas from `JsonSchema` types.

4. [x] **Refactor Tool Registration:** 7412080
   - Update `src/mcp/mod.rs` to use the automated schema generation during tool registration.

5. [x] **Consistency Check:** 7412080
   - Add a test case to verify that the generated schemas are consistent with the expected MCP tool format.
