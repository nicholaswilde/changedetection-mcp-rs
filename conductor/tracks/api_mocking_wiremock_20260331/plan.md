# Implementation Plan

1. [x] **Centralize Mocking Helpers:** (dfbadf7)
   - Create a common module in `tests/` for shared `wiremock` setup and helper functions to reduce code duplication.

2. [x] **Test Edge Cases (HTTP Errors):** (ce88d13)
   - Add tests for all possible HTTP error codes returned by the API (e.g., 401 Unauthorized, 403 Forbidden, 404 Not Found, 500 Internal Server Error).

3. [x] **Simulate Network Conditions:** (67da0d2)
   - Use `wiremock` to simulate network timeouts and slow responses to verify the client's timeout handling.

4. [x] **Handle Malformed Responses:** (e7dae23)
   - Add tests where the mock server returns malformed or unexpected JSON to ensure the client fails gracefully.

5. [x] **Verification:**
   - Run the updated test suite and ensure all edge-case scenarios are covered and passing.
