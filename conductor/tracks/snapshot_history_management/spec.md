# Specification: Snapshot & History Management (Refined)

## Goal
To provide tools for analyzing and managing watch history, focusing on metadata retrieval, bulk history listing, and retention management.

## Requirements
- **`get_snapshot_metadata` Tool**:
    - Input: `uuid`, `timestamp`.
    - Output: Technical metadata such as HTTP status, response time, and content length for that specific snapshot.
- **`list_all_history` Tool**:
    - Input: `tag` (optional).
    - Action: Retrieve the history list for all watches, or all watches matching a specific tag.
    - Output: A map of watch UUIDs to their history lists.
- **`set_history_limit` Tool**:
    - Input: `uuid`, `limit` (integer).
    - Action: Update the `history_snapshot_max_length` field for the specified watch to manage its history size.

## Success Criteria
- Tools are successfully implemented in `src/api/mod.rs` and exposed in `src/mcp/mod.rs`.
- Integration tests verify accurate metadata retrieval and limit setting.
- LLMs can effectively audit history sizes and technical snapshot performance.
