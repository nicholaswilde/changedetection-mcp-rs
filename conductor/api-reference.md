# ChangeDetection.io API v1 Reference

Comprehensive documentation for the ChangeDetection.io REST API (v0.1.6).

## Overview

The ChangeDetection.io API allows you to manage page watches, group tags, and notifications programmatically.

### Connection URL
The API is accessible at the `/api/v1/` path of your instance.
*   **Local:** `http://localhost:5000/api/v1/`
*   **Hosted:** `https://<your-login-url>/api/v1/`

### Authentication
Most requests require an API Key passed in the HTTP header:
`x-api-key: YOUR_API_KEY`

**Where to find your API key:** Dashboard -> Settings -> API tab.

---

## Watch Management

### List All Watches
Returns a concise list of available monitors.
*   **Method:** `GET`
*   **Endpoint:** `/watch`
*   **Query Parameters:**
    *   `recheck_all` (string): Set to `"1"` to force recheck of all watches.
    *   `tag` (string): Filter results by tag name.
*   **Response (200):** JSON object where keys are Watch UUIDs.

### Create a New Watch
Creates a single web page monitor.
*   **Method:** `POST`
*   **Endpoint:** `/watch`
*   **Request Body (JSON) - Required Fields:**
    *   `url` (string): The URL to monitor.
*   **Request Body (JSON) - Optional Fields:**
    *   `title` (string): Custom title.
    *   `tag` (string) / `tags` (array): Tag UUIDs.
    *   `paused` / `notification_muted` (boolean).
    *   `method` (string): `GET`, `POST`, `DELETE`, `PUT`.
    *   `fetch_backend` (string): `system`, `html_requests`, `html_webdriver`, etc.
    *   `time_between_check` (object): Interval settings (weeks, days, hours, minutes, seconds).
    *   `processor` (string): `text_json_diff` (default) or `restock_diff`.
    *   `include_filters` / `subtractive_selectors` (array of strings): CSS/XPath selectors.
*   **Response (200):** Success message.

### Get Single Watch
Retrieve full information for a specific watch.
*   **Method:** `GET`
*   **Endpoint:** `/watch/{uuid}`
*   **Path Parameters:**
    *   `uuid` (string): Watch unique ID.
*   **Query Parameters:**
    *   `recheck` (string): `"1"` or `"true"` to trigger a check.
    *   `paused` (string): `"paused"` or `"unpaused"`.
    *   `muted` (string): `"muted"` or `"unmuted"`.
*   **Response (200):** Full Watch JSON object.

### Update Watch
Update an existing watch configuration.
*   **Method:** `PUT`
*   **Endpoint:** `/watch/{uuid}`
*   **Request Body:** JSON (same structure as Create Watch).
*   **Special Field:** `last_viewed` (integer): Set to a timestamp higher than `last_changed` to mark as viewed.
*   **Response (200):** Success message.

### Delete Watch
Permanently remove a watch and its history.
*   **Method:** `DELETE`
*   **Endpoint:** `/watch/{uuid}`
*   **Response (200):** Success message.

---

## Watch History & Snapshots

### Get Watch History
List all historical snapshots for a watch.
*   **Method:** `GET`
*   **Endpoint:** `/watch/{uuid}/history`
*   **Response (200):** JSON object mapping Unix timestamps to snapshot file paths.

### Get Difference Between Two Snapshots
Generate a comparison between two points in time.
*   **Method:** `GET`
*   **Endpoint:** `/watch/{uuid}/difference/{from_timestamp}/{to_timestamp}`
*   **Path Parameters:**
    *   `from_timestamp`: Unix timestamp or `"previous"`.
    *   `to_timestamp`: Unix timestamp or `"latest"`.
*   **Query Parameters:**
    *   `format` (string): `text` (default), `html`, `htmlcolor`, `markdown`.
    *   `word_diff` (boolean string): Enable word-level granularity.
    *   `changesOnly` (boolean string): Hide surrounding context.
*   **Response (200):** Formatted diff content.

### Get Single Snapshot
Retrieve the text or HTML content of a specific check.
*   **Method:** `GET`
*   **Endpoint:** `/watch/{uuid}/history/{timestamp}`
*   **Path Parameters:** `timestamp` (Unix timestamp or `"latest"`).
*   **Query Parameters:** `html=1` to retrieve the raw HTML instead of text.
*   **Response (200):** Snapshot content.

---

## Group / Tag Management

### List All Tags
*   **Method:** `GET`
*   **Endpoint:** `/tags`
*   **Response (200):** JSON list of tag objects.

### Create Tag
*   **Method:** `POST`
*   **Endpoint:** `/tag`
*   **Request Body (JSON):** `title` (required), `notification_urls`, `notification_muted`, `overrides_watch`.
*   **Response (201):** Returns the new Tag UUID.

### Get/Update/Delete Tag
*   **Endpoints:**
    *   `GET /tag/{uuid}`: Retrieve info or trigger recheck via `?recheck=true`.
    *   `PUT /tag/{uuid}`: Update tag settings.
    *   `DELETE /tag/{uuid}`: Remove tag.

---

## Notifications

Manage global notification endpoints (uses [Apprise](https://github.com/caronc/apprise) syntax).

*   **GET `/notifications`**: List configured URLs.
*   **POST `/notifications`**: Add new URLs.
*   **PUT `/notifications`**: Replace all URLs.
*   **DELETE `/notifications`**: Remove specific URLs.

---

## Search & Import

### Search Watches
*   **Method:** `GET`
*   **Endpoint:** `/search`
*   **Query Parameters:**
    *   `q` (required): Search query for URLs and titles.
    *   `tag`: Filter by tag name.

### Import Watch URLs
Bulk import URLs with shared configuration.
*   **Method:** `POST`
*   **Endpoint:** `/import`
*   **Query Parameters:** Any watch configuration field (e.g., `tag_uuids`, `proxy`, `fetch_backend`).
*   **Request Body:** `text/plain` list of line-separated URLs.
*   **Response (200):** Array of created Watch UUIDs.

---

## System Information

### Get System Info
Retrieve instance status and statistics.
*   **Method:** `GET`
*   **Endpoint:** `/systeminfo`
*   **Response (200):** JSON containing `watch_count`, `tag_count`, `uptime`, and `version`.

### Get Full Live API Spec
Returns the dynamically generated OpenAPI specification, including fields added by installed processor plugins.
*   **Method:** `GET`
*   **Endpoint:** `/full-spec`
*   **Authentication:** None required.
*   **Format:** YAML.
