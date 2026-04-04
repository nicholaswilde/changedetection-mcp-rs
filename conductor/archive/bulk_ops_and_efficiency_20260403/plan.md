# Implementation Plan: Bulk Operations and Efficiency

## Phase 1: Bulk Re-checks
- [x] **Task: Implement Trigger All Action** bb10d36
    - [x] Add `TriggerAll` action to `WatchAction`.
    - [x] Implement `Client::trigger_recheck_all` (with tag filter support).
- [x] **Task: Verification - Bulk Re-checks** bb10d36
    - [x] Add integration tests for tag-specific bulk triggers.

## Phase 2: State Management
- [x] **Task: Implement Mark as Viewed Action** 4983709
    - [x] Add `MarkAsViewed` action to `WatchAction`.
    - [x] Implement handler to update `last_viewed`.
- [x] **Task: Verification - State Management** 4983709
    - [x] Add integration tests for viewing status updates.

## Phase 3: Bulk Retention
- [x] **Task: Implement Bulk Retention Action** 4983709
    - [x] Add `SetBulkLimit` to `HistoryAction`.
    - [x] Implement handler to apply limits to tags or all watches.
- [x] **Task: Verification - Bulk Retention** 4983709
    - [x] Add integration tests for bulk history limit application.
