# /test-fix

Runs the full CI test suite (fmt, lint, clippy, cargo tests) and autonomously fixes failures.

## Description
This skill ensures the ChangeDetection.io MCP server codebase conforms to formatting, linting, quality, and testing standards. It actively detects violations and applies automatic or surgical corrections.

## Protocol

1. **Execute CI Checks:**
   - Run `task test:ci` to execute formatting checks, linting, cargo tests, and clippy:
     - Formatting: `cargo fmt -- --check`
     - Linting & Clippy: `task lint` / `task clippy`
     - Unit & Integration Tests: `cargo test --profile ci`

2. **Analyze Failures:**
   - If any check fails, capture the output and identify the category:
     - **Formatting**: Code formatting style mismatch.
     - **Linting/Clippy**: Code quality warnings or compiler-recommended lints.
     - **Unit/Integration Tests**: Test assertion failures, unexpected panics, or mock configuration issues.

3. **Apply Corrections:**
   - **Formatting**: Run `task fmt` to automatically fix formatting.
   - **Lints/Clippy**: Surgical corrections in the code. Address the lint recommendations directly, or add `#[allow(...)]` attributes if a lint is a false positive.
   - **Tests**: Debug code logic, update test assertions, or correct mock objects as needed.
   - **CRITICAL:** Do NOT modify `Taskfile.yml` to bypass or disable any checks.

4. **Verify and Re-Test:**
   - Re-run `task test:ci` to ensure that all checks pass cleanly after your modifications.
   - Optionally, run `task test:integration` if integration tests need separate verification.

5. **Report:**
   - Provide a concise summary of the checks executed and any fixes applied.
