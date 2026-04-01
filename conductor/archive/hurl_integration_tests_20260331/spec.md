# Specification

## Problem

Testing the MCP server's end-to-end behavior is currently limited to unit and integration tests within Rust, which may not capture all real-world interaction scenarios from an external consumer's perspective.

## Goals

- Implement a suite of integration tests using `hurl`.
- Provide a way to test the API from an external perspective.
- Automate these tests in the CI pipeline.

## Non-Goals

- Replacing existing Rust integration tests.
- Performance testing or load testing.
