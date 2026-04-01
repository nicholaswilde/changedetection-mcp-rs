# Specification

## Problem

The AI currently lacks visibility into the version and health status of the upstream ChangeDetection.io instance it is connecting to, which can be critical for troubleshooting.

## Goals

- Implement `GET /api/v1/systeminfo` to retrieve server version and stats.
- Expose `get_system_info` as an MCP tool.

## Non-Goals

- Complex health checks that require multiple API calls.