# /live-test

Runs the live CI suite against a real ChangeDetection.io instance using credentials from `.env`.

## Description
This skill verifies the ChangeDetection.io MCP server against a live running instance using environment variables for authentication.

## Protocol

1. **Verify Environment Configuration:**
   - Check if a `.env` file exists in the workspace.
   - Verify that it contains the `CHANGEDETECTION_API_KEY`.
   - **WARNING:** Do NOT print, display, or leak the API key or any secrets from the `.env` file in any response, log, or workspace output.

2. **Execute Live CI Suite:**
   - Execute `task test:live` to run the `tests/live.rs` integration suite.
   - This suite covers:
     - System status and version retrieval.
     - MCP tool execution (`list_watches`, `get_watch_details`, `create_watch`, `delete_watch`, `trigger_check`).

3. **Analyze and Report Results:**
   - Capture and summarize the test output.
   - If tests fail, investigate potential issues (e.g., live instance unreachable, network latency, invalid API keys).
