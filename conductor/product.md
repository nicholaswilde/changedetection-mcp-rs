# Initial Concept

An MCP server built from rust that interacts with a local instance of ChangeDetection.io API

# Product Definition: changedetection-mcp-rs

## Vision
To provide a seamless interface between Model Context Protocol (MCP) clients and ChangeDetection.io, enabling AI models to monitor website changes, retrieve tracked data, and manage watchlists directly through the ChangeDetection.io API.

## Core Purpose
The `changedetection-mcp-rs` server acts as a bridge, translating MCP tool calls into ChangeDetection.io API requests. This allows users to leverage AI to automate web monitoring and data analysis tasks.

## Target Audience
- Developers using MCP-compatible AI clients (like Claude Desktop).
- Data analysts who need to monitor web changes via ChangeDetection.io.
- Users looking to integrate AI-driven web monitoring into their workflows.

## Key Features (Draft)
- **List Watches:** Retrieve a list of all current watches, optionally filtered by tag or operational state (paused, active, error).
- **Search Watches:** Efficiently find specific watches by URL or title using search queries, optimizing token usage.
- **System Information:** Retrieve ChangeDetection.io server version and statistics for health monitoring and troubleshooting.
- **System & Environment Discovery:** Tools for discovering instance-level configurations, including supported fetchers, configured proxies, global settings, and the full OpenAPI specification.
- **Tag Management:** Complete CRUD operations for tags, enabling organized watch groups.
- **Get Watch Details:** Fetch detailed information and the latest changes for a specific watch. Filter watches by tag.
- **Bulk Import:** Efficiently add multiple URLs as new watches in a single operation.
- **Notification Management:** Complete CRUD operations for global notification endpoints (Apprise-compatible).
- **Advanced Watch Configuration:** Specialized tools for fine-grained configuration of selectors (CSS, XPath, JSONPath), fetching engines (e.g., Playwright), and per-watch notification settings.
- **Explicit State Management:** High-intent tools to pause/unpause watches and mute/unmute notifications.
- **Create/Update Watch:** Programmatically add new URLs to track or modify existing watch configurations.
- **Delete Watch:** Remove watches that are no longer needed.
- **Visual Snapshots:** Retrieve base64 encoded screenshots of monitored pages (requires browser fetcher).
- **Trigger Re-check:** Manually force a check for changes on a specific watch.
- **Get Snapshots:** Retrieve historical snapshots of tracked pages.
- **Processor Discovery:** List available change detection processors (e.g., restock_diff, text_json_diff).
- **Advanced Snapshot Analysis:** Tools for retrieving technical metadata (content-length, content-type) for snapshots and bulk-listing history across multiple watches.
- **Specialized Filtering:** High-intent tools for identifying problematic watches (filtering by error state) or auditing instance configurations (filtering by change detection processor).
- **Retention Management:** Dedicated tools to manage watch history size and retention limits.
- **System Maintenance:** High-intent tools for triggering system-wide backups and performing full watch configuration exports for data portability and safety.
- **MCP Resources:** Support for MCP Resources, allowing LLMs to directly read watch snapshots (`watches://{uuid}/latest`) and the full OpenAPI specification (`system://openapi-spec`) using standard URI schemes.

## Design Principles
- **Rust-powered Efficiency:** Leverage Rust's performance and safety for a reliable server.
- **Resilient API Communication:** Implement robust retry, caching, and configurable timeout strategies to ensure reliable interaction with ChangeDetection.io, even under adverse network conditions.
- **MCP Compliance:** Adhere strictly to the Model Context Protocol for maximum compatibility.
- **Automated Schema Synchronization:** Use automated tools to ensure that MCP tool definitions and JSON schemas are always in sync with the underlying Rust implementation.
- **Advanced Observability:** Implement structured logging (JSON) and request correlation (Request-IDs) to simplify debugging and monitoring in complex environments.
- **End-to-End Testability:** Maintain a suite of scriptable integration tests to verify the server's behavior from an external consumer's perspective.
- **Secure API Interaction:** Securely handle ChangeDetection.io API keys and connection settings.
- **Developer-Centric:** Provide clear tool definitions and informative error messages.
