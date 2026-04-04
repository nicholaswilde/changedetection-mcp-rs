# Implementation Plan: Analysis and Visualization

## Phase 1: Enhanced Diffing
- [x] **Task: Implement Advanced Diff Parameters** 3ac29f6
    - [x] Update `HistoryOpsArgs` with `word_diff`, `changes_only`, and `ignore_whitespace`.
    - [x] Update `Client::get_watch_diff` to support the new query parameters.
- [x] **Task: Verification - Diffs** 3ac29f6
    - [x] Add integration tests for surgical diffing.

## Phase 2: Technical Metadata
- [x] **Task: Implement Metadata Retrieval** 2fa18cc
    - [x] Add `GetSnapshotMetadata` to `HistoryAction`.
    - [x] Implement handler to return MD5, content-type, etc.
- [x] **Task: Verification - Metadata** 2fa18cc
    - [x] Add integration tests for metadata retrieval.

## Phase 3: Favicons
- [x] **Task: Implement Favicon Action** 86fc38c
    - [x] Add `GetFavicon` to `WatchAction`.
    - [x] Implement `Client::get_watch_favicon`.
- [x] **Task: Verification - Favicons** 86fc38c
    - [x] Add integration tests for favicon retrieval.
