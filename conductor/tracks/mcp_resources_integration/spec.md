# Specification: MCP Integration Features (Resources)

## Goal
To extend the MCP server with "Resources," allowing LLMs to directly read watch snapshots and system specifications using standard URI schemes.

## Requirements
- **`watches://{uuid}/latest` Resource**:
    - Action: Provide the most recent snapshot content of the specified watch.
- **`system://openapi-spec` Resource**:
    - Action: Expose the full OpenAPI specification as a static resource.

## Success Criteria
- Resources are successfully registered and exposed in the MCP server.
- LLMs can successfully retrieve snapshot data and API specifications using the URI format.
- Integration tests verify the availability and content of the new resources.
