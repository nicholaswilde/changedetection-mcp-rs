# Specification: Analysis and Visualization

## Goal
To enhance the AI's ability to analyze and visualize tracked content through granular diffs, technical metadata, and visual assets like favicons.

## Requirements
- **Enhanced Diffing**: Support `word_diff`, `changesOnly`, and `ignoreWhitespace` parameters in `HistoryAction::GetDiff` to reduce token usage and focus on content.
- **Technical Metadata**: Expose `content-type`, `fetch_time`, and `MD5` hashes in `history_ops`.
- **Favicon Retrieval**: New action `GetFavicon` in `watch_ops` to retrieve watch favicons.

## Success Criteria
- LLMs can request word-level diffs showing only changes.
- Snapshot metadata is available for fetch quality auditing.
- Watch favicons can be retrieved as binary or base64 data.
