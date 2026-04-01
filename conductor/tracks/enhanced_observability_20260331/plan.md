# Implementation Plan

1. [x] **Add Tracing Dependencies:** (174ec66)
   - Add `tracing-appender` for file logging.
   - Add `tracing-serde` for JSON log serialization.

2. [x] **Enhance Tracing Subscriber:** (256b57f)
   - Configure the subscriber to support both human-readable (stdout) and structured (file/JSON) outputs.

3. [x] **Implement Request Correlation:** (3a9ab1f)
   - Add a middleware or context layer to inject and propagate a `Request-Id` in all logs related to a single operation.

4. [ ] **Update CLI Configuration:**
   - Add `--log-format` and `--log-file` arguments to the CLI to allow users to configure observability at runtime.

5. [ ] **Validation:**
   - Verify that logs are correctly formatted and that correlation IDs are consistent across log entries for a single request.
