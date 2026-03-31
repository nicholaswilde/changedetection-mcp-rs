# Specification: Add Command Line Arguments

## Overview
Implement command-line argument parsing for `changedetection-mcp-rs` to provide essential utility flags and configuration overrides. This includes showing the application version and providing a way to specify critical settings directly from the terminal.

## Functional Requirements
- **Display Version:**
  - Support `--version` and `-V` flags.
  - Display the current application version as defined in `Cargo.toml`.
  - The application must exit immediately after displaying the version.
- **Configuration Path:**
  - Support `--config <path>` and `-c <path>` to specify the path to a configuration file.
- **Log Level:**
  - Support `--log-level <level>` and `-l <level>` (e.g., `info`, `debug`, `error`, `warn`, `trace`).
- **API Key Override:**
  - Support `--api-key <key>` and `-k <key>` to provide the ChangeDetection.io API key.
- **Help Message:**
  - Provide a standard help message using `--help` and `-h`.

## Non-Functional Requirements
- **Performance:** Argument parsing should be fast and have minimal impact on startup time.
- **Robustness:** Invalid arguments should result in a clear error message and non-zero exit code.
- **Consistency:** Follow standard CLI conventions for short and long flags.

## Acceptance Criteria
- Running `changedetection-mcp-rs --version` or `changedetection-mcp-rs -V` prints the version and exits.
- Running `changedetection-mcp-rs --help` or `changedetection-mcp-rs -h` prints a detailed help message.
- Providing a `--config` path correctly points the app to that file.
- Providing a `--log-level` updates the tracing/logging level.
- Providing an `--api-key` overrides any environment variable or file-based API key.

## Out of Scope
- Implementation of the configuration file loading logic itself (if not already present).
- Complex command sub-commands (only top-level flags for now).
