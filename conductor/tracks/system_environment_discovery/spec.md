# Specification: System & Environment Discovery

## Goal
To provide tools for discovering instance-level configurations, including available fetchers, proxy lists, and global system settings.

## Requirements
- **`list_fetchers` Tool**:
    - Output: List of supported fetching engines.
- **`list_proxies` Tool**:
    - Output: List of available proxies and their configurations.
- **`get_global_settings` Tool**:
    - Output: Global settings such as default intervals, global filters, and system limits.

## Success Criteria
- Tools are implemented in `src/api/mod.rs` and `src/mcp/mod.rs`.
- Integration tests verify accurate retrieval of system-wide configurations.
- LLMs can understand the instance environment to recommend better watch configurations.
