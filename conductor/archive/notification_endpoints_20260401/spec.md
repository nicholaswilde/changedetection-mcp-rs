# Specification - Notification Endpoints

## Functional Requirements
- Implement `list_notifications` to fetch from `/api/v1/notifications`.
- Implement `add_notification` (POST) to add new Apprise-compatible URLs.
- Implement `update_notifications` (PUT) to replace the entire set.
- Implement `delete_notification` (DELETE) for individual removal.

## Technical Requirements
- Update `Client` in `api/mod.rs` with notification methods.
- Update `McpServer` in `mcp/mod.rs` to expose and handle these tools.
- Ensure correct header management for non-JSON responses if needed.
- Add unit tests for CRUD operations on notifications.
- Add live tests in `tests/live.rs` with non-intrusive URLs (e.g., `mailto://test@example.com`).
