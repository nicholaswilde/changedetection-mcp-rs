# Specification - Visual Snapshot Capture

## Goal
Provide a way to retrieve a visual representation (screenshot) of a watch via an MCP tool.

## Technical Requirements
- **MCP Tool Name:** `get_watch_screenshot`
- **Arguments:**
  - `uuid`: String (required) - The UUID of the watch.
- **API Endpoint:** `GET /api/v1/watch/{uuid}/screenshot`
- **Response:** The image data (might need to be base64 encoded for MCP).

## Success Criteria
- [ ] `api::Client` has a `get_watch_screenshot` method.
- [ ] `mcp::server` exposes the `get_watch_screenshot` tool.
- [ ] Integration tests verify successful screenshot retrieval.
- [ ] Correct behavior for watches without snapshots or screenshots.
