# Specification: Snapshot & History Management

## Goal
To provide tools for managing historical data of watches, including deletion of snapshots, triggering re-indexing, and retrieving technical metadata for specific snapshots.

## Requirements
- **`delete_snapshot` Tool**:
    - Input: `uuid`, `timestamp`.
    - Action: Delete a specific historical snapshot.
- **`reindex_watch` Tool**:
    - Input: `uuid`.
    - Action: Trigger a full re-indexing of historical data for the watch.
- **`get_snapshot_metadata` Tool**:
    - Input: `uuid`, `timestamp`.
    - Output: Technical metadata such as HTTP status, response time, and headers.

## Success Criteria
- Tools are implemented in `src/api/mod.rs` and `src/mcp/mod.rs`.
- Integration tests confirm that snapshots are correctly deleted and metadata is accurate.
- LLMs can manage and query watch history details more precisely.
