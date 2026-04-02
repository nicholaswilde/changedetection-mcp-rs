# Specification - Live Test: Search & Filtering

## Functional Requirements
- Verify `search_watches` returns correct results based on query strings.
- Verify `list_watches` correctly filters results when providing a tag name.
- Test edge cases for search queries (special characters, empty results).
- Ensure results contain expected fields (UUID, URL, title).

## Target Instance
- URL: `https://cd.l.nicholaswilde.io` (configured via `CHANGEDETECTION_BASE_URL` in `.env`)
