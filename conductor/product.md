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
- **Watch Operations (`watch_ops`):** Comprehensive management of watches, including listing (with state/tag filtering), searching, detailed retrieval, creation, updates, deletion, manual triggering, pausing/unpausing, and muting/unmuting notifications. Supports bulk imports and advanced configurations for selectors and fetchers.
- **Tag Management (`tag_ops`):** Complete CRUD operations for tags to organize and categorize watches efficiently.
- **Notification Management (`notification_ops`):** Manage global, system-wide notification endpoints (Apprise-compatible) with operations to list, add, update, and delete service URLs.
- **History & Analysis (`history_ops`):** Tools for deep analysis of watch history, including snapshot listing, diff generation (text/markdown/html), content retrieval, visual snapshot capture (screenshots), and retention limit management.
- **System Discovery (`system_ops`):** Discover instance-level capabilities, including server info, OpenAPI specifications, available fetchers, configured proxies, global settings, and change detection processors.
- **System Maintenance (`maintenance_ops`):** Critical tasks for data portability and safety, including system-wide backups and full watch configuration exports.
- **MCP Resources:** Support for MCP Resources, allowing LLMs to directly read watch snapshots (`watches://{uuid}/latest`) and the full OpenAPI specification (`system://openapi-spec`) using standard URI schemes.
- **Token Usage Optimization:** Built-in pagination and field selection for all list operations to minimize token consumption and improve LLM context efficiency.

## Design Principles
- **Rust-powered Efficiency:** Leverage Rust's performance and safety for a reliable server.
- **Resilient API Communication:** Implement robust retry, caching, and configurable timeout strategies to ensure reliable interaction with ChangeDetection.io, even under adverse network conditions.
- **MCP Compliance:** Adhere strictly to the Model Context Protocol for maximum compatibility.
- **Automated Schema Synchronization:** Use automated tools to ensure that MCP tool definitions and JSON schemas are always in sync with the underlying Rust implementation.
- **Advanced Observability:** Implement structured logging (JSON) and request correlation (Request-IDs) to simplify debugging and monitoring in complex environments.
- **End-to-End Testability:** Maintain a suite of scriptable integration tests to verify the server's behavior from an external consumer's perspective.
- **Secure API Interaction:** Securely handle ChangeDetection.io API keys and connection settings.
- **Developer-Centric:** Provide clear tool definitions and informative error messages.
