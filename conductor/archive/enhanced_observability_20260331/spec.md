# Specification

## Problem

The current `tracing` setup is basic and lacks structured logging and advanced filtering, making it difficult to debug complex issues in production and perform log analysis.

## Goals

- Improve the `tracing` setup with advanced subscribers.
- Implement structured logging (e.g., JSON format).
- Add correlation IDs to trace requests throughout the server.
- Allow dynamic configuration of log levels and formats via CLI.

## Non-Goals

- Implementing a full-blown distributed tracing system like Jaeger or Honeycomb (at this stage).
- Custom log storage.
