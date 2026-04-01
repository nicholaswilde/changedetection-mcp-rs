# Implementation Plan

1. [x] **Add Tracing Dependencies:** (174ec66)
   - Add `tracing-appender` for file logging.
   - Add `tracing-serde` for JSON log serialization.

2. [ ] **Enhance Tracing Subscriber:**
   - Configure the subscriber to support both human-readable (stdout) and structured (file/JSON) outputs.

3. [ ] **Implement Request Correlation:**
   - Add a middleware or context layer to inject and propagate a `Request-Id` in all logs related to a single operation.

4. [ ] **Update CLI Configuration:**
   - Add `--log-format` and `--log-file` arguments to the CLI to allow users to configure observability at runtime.

5. [ ] **Validation:**
   - Verify that logs are correctly formatted and that correlation IDs are consistent across log entries for a single request.
