# Implementation Plan - Live Test: Search & Filtering

## Steps
1. **[x] Develop Live Test Case**: 105916f
   - Open `tests/live.rs`.
   - Implement `test_live_search_filtering`.
   - Test `search_watches` with query for existing and non-existing titles.
   - Test `list_watches` with tag filtering.
2. **[x] Execute and Verify**: 35a221e
   - Run `task test:live`.
   - Ensure the output shows expected matches.
3. **[x] Task Completion**: de0889a
   - Document any discrepancies between API spec and live responses.
