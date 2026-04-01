# Specification

## Problem

As the number of watches grows, retrieving all watches to find a specific one consumes too many tokens. An AI needs a more efficient way to query for specific watches.

## Goals

- Implement `GET /api/v1/search` to allow querying watches by URL or title.
- Expose `search_watches` as an MCP tool to optimize LLM interactions.

## Non-Goals

- Implementing custom search indexing; this relies entirely on the upstream ChangeDetection.io API search capabilities.