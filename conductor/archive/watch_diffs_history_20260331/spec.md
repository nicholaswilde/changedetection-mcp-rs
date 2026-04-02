# Specification

## Problem

Currently, the MCP server can list watches and trigger checks, but it cannot analyze what actually changed between checks. This limits the AI's ability to summarize web page changes effectively.

## Goals

- Implement `GET /api/v1/watch/{uuid}/history` to retrieve a list of historical snapshots.
- Implement `GET /api/v1/watch/{uuid}/difference/{from_timestamp}/{to_timestamp}` to retrieve the diff between two snapshots.
- Expose these as MCP tools so LLMs can analyze changes.

## Non-Goals

- Storing or parsing the snapshot contents locally; the server acts purely as a passthrough.