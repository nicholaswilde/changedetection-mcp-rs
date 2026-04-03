# Specification - Processor Discovery

## Goal
Provide a way to discover available processors/filters in ChangeDetection.io via an MCP tool.

## Technical Requirements
- **MCP Tool Name:** `list_processors`
- **Arguments:** None.
- **API Endpoint:** Identify the endpoint for listing processors (e.g., `/api/v1/processors` or similar).
- **Response:** List of processors with their names and descriptions.

## Success Criteria
- [ ] `api::Client` has a `list_processors` method.
- [ ] `mcp::server` exposes the `list_processors` tool.
- [ ] Integration tests verify listing of processors.
