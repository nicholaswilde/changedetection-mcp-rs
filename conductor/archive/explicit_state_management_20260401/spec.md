# Specification - Explicit State Management

## Goal
Provide specific MCP tools to control watch state (pause/unpause) and notification status (mute/unmute).

## Technical Requirements
- **MCP Tools:**
  - `pause_watch`: Pause a watch (stop checking for changes).
  - `unpause_watch`: Resume checking for changes on a watch.
  - `mute_notifications`: Stop sending notifications for a watch.
  - `unmute_notifications`: Resume sending notifications for a watch.
- **Arguments:**
  - `uuid`: String (required) - The UUID of the watch.
- **API Strategy:**
  - Pausing/Unpausing might use the existing `update_watch` method or specific endpoints if available in the OpenAPI spec.
  - Muting/Unmuting notifications for a watch.
- **Response:** Success status.

## Success Criteria
- [ ] `api::Client` supports the required state changes.
- [ ] `mcp::server` exposes all four tools.
- [ ] Integration tests verify state transitions.
- [ ] Correct behavior for non-existent watches.
