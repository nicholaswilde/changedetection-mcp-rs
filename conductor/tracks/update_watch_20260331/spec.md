# Specification

## Problem

Users can create and delete watches but cannot modify existing watches. If a watch is too noisy, they have to delete it and create a new one instead of simply updating the CSS filter or interval.

## Goals

- Implement `PUT /api/v1/watch/{uuid}` to update watch configurations (e.g., URL, title, tags).
- Expose `update_watch` as an MCP tool.

## Non-Goals

- Complex bulk-update operations.