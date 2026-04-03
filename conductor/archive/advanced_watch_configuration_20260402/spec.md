# Specification: Advanced Watch Configuration

## Goal
To provide specialized tools for fine-grained watch configuration, moving beyond generic JSON updates to high-intent, type-safe operations for selectors, fetchers, and notifications.

## Requirements
- **`set_watch_selectors` Tool**:
    - Input: `uuid`, `css_filter` (optional), `xpath_filter` (optional), `json_filter` (optional).
    - Action: Update the specific filter fields in the watch configuration.
- **`set_watch_fetcher` Tool**:
    - Input: `uuid`, `fetcher` (e.g., "html_webdriver", "html_requests", "playwright").
    - Action: Switch the fetching engine for the specified watch.
- **`configure_watch_notifications` Tool**:
    - Input: `uuid`, `notification_urls` (Vec<String>), `notification_title` (optional), `notification_body` (optional).
    - Action: Configure per-watch notification settings, overriding or supplementing global defaults.

## Success Criteria
- Each tool is successfully implemented in `src/api/mod.rs` and exposed in `src/mcp/mod.rs`.
- Integration tests verify that settings are correctly applied to a mock or live ChangeDetection.io instance.
- LLM can successfully use these tools to configure a watch without knowing the internal JSON structure of the `UpdateWatch` API.
