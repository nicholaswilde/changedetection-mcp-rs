# Specification - Live Test: History & Diffs

## Functional Requirements
- Verify `get_watch_history` lists available snapshots for a watch.
- Verify `get_watch_diff` generates a text/markdown diff between two points.
- Verify diff formats (text, markdown) work as intended.
- Ensure handling of "previous" and "latest" aliases in timestamps.

## Target Instance
- URL: `https://cd.l.nicholaswilde.io` (configured via `CHANGEDETECTION_BASE_URL` in `.env`)
