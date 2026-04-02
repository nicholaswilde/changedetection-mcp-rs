# Implementation Plan - Live Test: History & Diffs

## Steps
1. [x] **Develop Live Test Case**: (e7a9d94)
   - Open `tests/live.rs`.
   - Implement `test_live_history_diffs`.
   - Pick a watch with existing history.
   - Fetch history.
   - Fetch diff between latest two snapshots.
2. **Execute and Verify**:
   - Run `task test:live`.
   - Inspect diff output for correctness.
3. **Verify API Parameters**:
   - Ensure diff format parameter (text/markdown) is correctly applied.
