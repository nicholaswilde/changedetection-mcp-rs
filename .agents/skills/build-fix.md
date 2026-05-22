# /build-fix <architecture>

Builds the project for the specified architecture (defaults to amd64), fixes compilation issues, and runs the test-fix skill upon success.

## Description
This skill compiles the ChangeDetection.io MCP server for a specific architecture, handles any compilation or cross-compilation issues, and ensures code quality by running subsequent validation tests.

## Protocol

1. **Determine Target Architecture:**
   - The target architecture is provided via the first argument `<architecture>` (defaults to `amd64` if empty or unspecified).
   - Valid architectures:
     - **amd64** (default): Run `task build:amd64` (x86_64-unknown-linux-gnu)
     - **arm64**: Run `task build:arm64` (aarch64-unknown-linux-gnu)
     - **armv7**: Run `task build:armv7` (armv7-unknown-linux-gnueabihf)
     - **local**: Run `task build:local` (Native build)

2. **Execute Build:**
   - Propose and run the build command for the selected architecture using the terminal/shell tool.

3. **Analyze and Fix Errors:**
   - If the build fails:
     - Capture and analyze compiler error logs.
     - Identify resolution strategies for cross-compilation errors (e.g., `cross` toolchain configuration, missing target-specific crates, target dependencies).
     - **IMPORTANT:** Do NOT modify `Taskfile.yml` to bypass errors. Edit code files, crate configurations, or build parameters instead.
     - Iterate build executions and surgical fixes until the compilation passes.

4. **Verify Overall Integrity:**
   - Once the build succeeds, run the `/test-fix` skill to ensure overall project integrity (formatting, linting, unit/integration testing).

5. **Completion:**
   - Confirm the successful build and report the results of the subsequent test phase.
