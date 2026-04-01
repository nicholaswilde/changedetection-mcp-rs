# Implementation Plan

1. [x] **Create Test Directory:** 791b4d1
   - Create a `tests/hurl/` directory to store `.hurl` test files.

2. [ ] **Implement Core Hurl Tests:**
   - Write Hurl scripts for:
     - Listing MCP tools.
     - Calling a sample tool (e.g., `list_watches`).
     - Error handling (e.g., calling a non-existent tool).

3. [ ] **Update Taskfile:**
   - Add a `test-hurl` task to `Taskfile.yml` to run all Hurl tests.

4. [ ] **CI Integration:**
   - Update `.github/workflows/` to run Hurl tests as part of the CI pipeline.

5. [ ] **Documentation:**
   - Add instructions in `README.md` or a new `TESTING.md` on how to run and write Hurl tests.
