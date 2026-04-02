# Specification - Live Test: Tag Management

## Functional Requirements
- Verify `list_tags` retrieves all configured tags.
- Verify `create_tag` successfully adds a new tag and returns a UUID.
- Verify `get_tag_details` retrieves detailed information for a specific tag.
- Verify `update_tag` successfully modifies tag settings (e.g., notification settings).
- Verify `delete_tag` removes a tag and ensures it no longer exists.

## Target Instance
- URL: `https://cd.l.nicholaswilde.io` (configured via `CHANGEDETECTION_BASE_URL` in `.env`)
