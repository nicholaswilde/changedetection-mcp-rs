# Specification

## Problem

Manually maintaining JSON schemas for MCP tools is error-prone and can lead to desynchronization between the Rust implementation and the tool definitions.

## Goals

- Use `schemars` to automatically generate JSON schemas from Rust structs.
- Ensure tool schemas are always in sync with the actual parameters expected by the MCP server.
- Simplify the process of adding or updating MCP tools.

## Non-Goals

- Completely removing the need for tool metadata beyond parameters.
- Implementing a full Rust-to-JSON-Schema macro for all types.
