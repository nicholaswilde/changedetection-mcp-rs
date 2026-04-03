# Specification - Snapshot Content Retrieval

## Goal
Provide a way to retrieve the full, raw content of a specific watch snapshot via an MCP tool.

## Technical Requirements
- **MCP Tool Name:** `get_snapshot_content`
- **Arguments:**
  - `uuid`: String (required) - The UUID of the watch.
  - `timestamp`: String (required) - The timestamp of the snapshot.
- **API Endpoint:** `GET /api/v1/watch/{uuid}/history/{timestamp}`
- **Response:** The raw content of the snapshot as a string.

## Success Criteria
- [ ] `api::Client` has a `get_snapshot_content` method.
- [ ] `mcp::server` exposes the `get_snapshot_content` tool.
- [ ] Integration tests verify successful content retrieval.
- [ ] Error handling for non-existent snapshots.
