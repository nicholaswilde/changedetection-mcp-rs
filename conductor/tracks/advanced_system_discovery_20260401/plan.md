# Implementation Plan - Advanced System Discovery

## Steps
1. **[x] API Implementation**: f35b089
   - Update `src/api/mod.rs`.
   - Add `get_full_spec` method returning `Result<String, ApiError>`.
2. **[x] MCP Integration**: e086c8d
   - Update `src/mcp/mod.rs`.
   - Add `get_full_spec` to `list_tools`.
   - Add handling for `get_full_spec` in `handle_method`.
3. **[x] Unit Testing**: f35b089
   - Add unit tests for the new method in `src/api/mod.rs` (using WireMock if possible or simple mocks).
4. **Live Verification**:
   - Add `test_live_get_full_spec` to `tests/live.rs`.
   - Run `task test:live` and verify YAML structure.
