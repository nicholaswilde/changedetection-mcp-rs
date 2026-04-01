# Specification

## Problem

Watches can be tagged, but there is no way to manage the tags themselves (e.g., listing all available tags, or creating new ones directly).

## Goals

- Implement `GET /api/v1/tag` to list all tags.
- Implement `GET /api/v1/tag/{uuid}`, `POST /api/v1/tag`, `PUT /api/v1/tag/{uuid}`, and `DELETE /api/v1/tag/{uuid}` for complete tag management.
- Expose these as MCP tools (e.g., `list_tags`, `create_tag`).

## Non-Goals

- Implementing bulk assignment of tags to multiple watches at once (unless natively supported by the upstream API).