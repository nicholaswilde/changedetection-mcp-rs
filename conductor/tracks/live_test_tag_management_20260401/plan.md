# Implementation Plan - Live Test: Tag Management

## Steps
1. **[x] Develop Live Test Case**: ed6fc2e
   - Open `tests/live.rs`.
   - Implement `test_live_tag_lifecycle` function.
   - Include steps for Create -> Get -> Update -> Delete.
2. **Configure Environment**:
   - Ensure `.env` has valid `CHANGEDETECTION_BASE_URL` and `CHANGEDETECTION_API_KEY`.
3. **Execute and Verify**:
   - Run `task test:live`.
   - Confirm all tag-related tests pass.
4. **Clean Up**:
   - Ensure no leftover test tags remain on the live instance if the test fails.
