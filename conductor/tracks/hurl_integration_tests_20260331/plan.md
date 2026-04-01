# Implementation Plan

1. [x] **Create Test Directory:** 791b4d1
   - Create a `tests/hurl/` directory to store `.hurl` test files.

2. [x] **Implement HTTP/SSE Transport:** 8404538
   - Integrate `axum` and `mcp-sdk-rs` SSE support.
   - Update `McpServer` to handle both `stdio` and `HTTP/SSE` transports.
   - Add CLI arguments to choose the transport and port.

3. [x] **Implement Core Hurl Tests:** 69746f5
   - Write Hurl scripts for:
     - Listing MCP tools.
     - Calling a sample tool (e.g., `list_watches`).
     - Error handling (e.g., calling a non-existent tool).

4. [x] **Update Taskfile:** 69746f5
   - Add a `test-hurl` task to `Taskfile.yml` to run all Hurl tests.

5. [x] **CI Integration:** 8122d10
   - Update `.github/workflows/` to run Hurl tests as part of the CI pipeline.

6. [x] **Documentation:** 7ab4e46
   - Add instructions in `README.md` or a new `TESTING.md` on how to run and write Hurl tests.
