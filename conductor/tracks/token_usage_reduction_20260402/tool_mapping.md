# MCP Tool Consolidation Mapping

This document maps the 39 existing fine-grained MCP tools to the 5 proposed category tools.

## 1. `watch_ops`
Handles CRUD, search, triggers, and state management for watches.
- `list_watches`
- `search_watches`
- `get_watch_details`
- `create_watch`
- `update_watch`
- `delete_watch`
- `trigger_check`
- `pause_watch`
- `unpause_watch`
- `mute_notifications`
- `unmute_notifications`
- `import_watches`
- `set_watch_selectors`
- `set_watch_fetcher`
- `configure_watch_notifications`
- `find_watches_by_error`
- `list_watches_by_processor`

## 2. `tag_ops`
Handles CRUD and details for tags.
- `list_tags`
- `create_tag`
- `get_tag_details`
- `update_tag`
- `delete_tag`

## 3. `notification_ops`
Handles management of global notification endpoints.
- `list_notifications`
- `add_notification`
- `update_notifications`
- `delete_notification`

## 4. `history_ops`
Handles snapshots, diffs, screenshots, and retention.
- `get_watch_history`
- `get_watch_diff`
- `get_snapshot_content`
- `get_watch_screenshot`
- `list_all_history`
- `set_history_limit`
- `get_snapshot_info`

## 5. `system_ops`
Handles discovery, settings, specs, and status.
- `get_system_info`
- `get_full_spec`
- `list_fetchers`
- `list_proxies`
- `get_global_settings`
- `list_processors`
