# Implementation Plan - Live Test: Tag Management

## Steps
1. **[x] Develop Live Test Case**: ed6fc2e
   - Open `tests/live.rs`.
   - Implement `test_live_tag_lifecycle` function.
   - Include steps for Create -> Get -> Update -> Delete.
2. **[x] Configure Environment**: 6aa4ff8
   - Ensure `.env` has valid `CHANGEDETECTION_BASE_URL` and `CHANGEDETECTION_API_KEY`.
3. **[x] Execute and Verify**: 6aa4ff8
   - Run `task test:live`.
   - Confirm all tag-related tests pass.
4. **[x] Clean Up**: 6aa4ff8
   - Ensure no leftover test tags remain on the live instance if the test fails.
