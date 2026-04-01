# Specification

## Problem

Integration tests for the API client rely on a limited set of `wiremock` responses, which do not cover all potential edge cases and error scenarios from the `changedetection.io` API.

## Goals

- Expand the use of `wiremock` to simulate various API responses, including error codes, slow responses, and malformed JSON.
- Create a reusable mock server setup for testing.
- Ensure the API client is robust against all possible API behaviors.

## Non-Goals

- Mocking the MCP SDK itself (handled by other tracks or unit tests).
- Implementing a full replica of the `changedetection.io` API.
