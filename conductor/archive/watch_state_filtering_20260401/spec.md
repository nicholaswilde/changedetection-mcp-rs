# Specification - Watch Filtering by State

## Goal
Improve the usability of `list_watches` by allowing users to filter watches by their current state.

## Technical Requirements
- **Feature:** Update `list_watches` MCP tool.
- **Arguments:**
  - `state`: String (optional) - The state to filter by (e.g., "paused", "unpaused", "error").
- **API Implementation:**
  - The API might not support direct state filtering via query params.
  - If the API doesn't support it, the MCP server must filter the results locally after fetching from the API.
- **Response:** Filtered list of watches.

## Success Criteria
- [ ] `list_watches` MCP tool supports the `state` parameter.
- [ ] Filtering works as expected.
- [ ] Integration tests verify filtering for each state.
