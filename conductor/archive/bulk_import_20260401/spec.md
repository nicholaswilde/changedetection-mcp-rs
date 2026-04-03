# Specification - Bulk Import

## Goal
Provide a way to bulk import watches via an MCP tool using the `POST /api/v1/import` endpoint.

## Technical Requirements
- **MCP Tool Name:** `import_watches`
- **Arguments:**
  - `urls`: List of Strings (required) - The URLs to import.
  - `tag`: String (optional) - The tag to assign to the imported watches.
- **API Endpoint:** `POST /api/v1/import`
- **Response:** Summary of the import (success/failure per URL).

## Success Criteria
- [ ] `api::Client` has an `import_watches` method.
- [ ] `mcp::server` exposes the `import_watches` tool.
- [ ] Integration tests verify successful bulk import.
- [ ] Error handling for malformed import data.
