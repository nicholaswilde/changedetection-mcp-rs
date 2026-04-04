# Specification - MCP Token Optimization

## Overview
Reduce the Model Context Protocol (MCP) token usage by consolidating the existing 39 fine-grained tools into a small set of high-level category tools. Additionally, implement response-size optimizations such as pagination and field selection to minimize tokens consumed by LLM interactions.

## Functional Requirements
1.  **Tool Consolidation (Category Tools)**:
    -   Group the 39 existing tools into broad categories:
        -   `watch_ops`: CRUD, search, triggers, and state management for watches.
        -   `tag_ops`: CRUD and details for tags.
        -   `notification_ops`: Management of global notification endpoints.
        -   `history_ops`: Snapshots, diffs, analysis, and retention.
        -   `system_ops`: Discovery, settings, specs, and status.
    -   Each category tool will take an `action` parameter to specify the intended operation.

2.  **Response Optimization**:
    -   **Pagination**: Add `page` and `per_page` (or similar) parameters to all list-returning operations.
    -   **Field Selection**: Add a `fields` parameter (array of strings) to allow the caller to request only specific fields in the JSON response.

3.  **Description Optimization**:
    -   Audit and shorten all tool and parameter descriptions to be as concise as possible while maintaining clarity for the LLM.

## Technical Requirements
-   Update `src/mcp/mod.rs` to define the new category-based argument structs and tool definitions.
-   Refactor `McpServer::handle_method` to route consolidated calls to the appropriate `Client` methods in `src/api/mod.rs`.
-   Update the `Client` methods in `src/api/mod.rs` (if necessary) to support field filtering and pagination logic (or implement it in the MCP layer).
-   Ensure backward compatibility or provide a clear path for migration (since this is an optimization track, we will replace the existing tools).

## Acceptance Criteria
-   The number of exposed MCP tools is reduced significantly (target: < 10).
-   LLMs can successfully perform all previous operations using the new consolidated tools.
-   Token usage for common operations (like listing watches) is demonstrably lower when using field selection and pagination.
-   All existing integration tests are updated and passing.
