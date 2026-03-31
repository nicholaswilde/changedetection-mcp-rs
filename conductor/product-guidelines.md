# Product Guidelines: changedetection-mcp-rs

## 1. Code Style & Architecture (Rust)
* **Safety First**: Utilize Rust's ownership and borrowing systems to prevent memory leaks and data races.
* **Async Efficiency**: Use `tokio` for high-concurrency, asynchronous operations, particularly for network-bound tasks with the ChangeDetection.io API.
* **Error Handling**: Implement custom error types using crates like `thiserror` or `anyhow` for clear, actionable error messages.
* **Modularity**: Organize code into clean abstractions (e.g., `api/`, `mcp/`, `config/`) to ensure long-term maintainability.
* **Documentation**: Maintain comprehensive inline documentation (using `///` comments) for all public items, enforced by `#![deny(missing_docs)]` in `src/lib.rs`. The `README.md` must be updated whenever MCP tools or functions are added or modified to ensure the AI tool list remains accurate.

## 2. Model Context Protocol (MCP) Standards
* **Token Optimization**: Every tool and response MUST be designed to minimize token usage for the LLM. Prefer high-level, functional abstractions over granular API data.
* **High-Signal Tools**: Each tool should provide a concise, high-signal output. Avoid returning massive JSON blobs unless specifically requested.
* **Clear Tool Descriptions**: Use descriptive tool names and comprehensive "tool definition" descriptions to help LLMs understand when and how to use each tool.
* **Parameter Validation**: Rigorously validate all tool inputs before attempting any external API calls.
* **Transparent Logging**: Provide detailed, levels-based logging (using `tracing` or `log`) for troubleshooting MCP/ChangeDetection.io interactions.

## 3. Security & Privacy
* **Secure Secret Management**: Never log API keys, passwords, or other sensitive information. Utilize environment variables or secure configuration files for credentials.
* **Strict Permission Checks**: Ensure the MCP server only performs operations that are explicitly authorized via the provided API credentials.
* **Local-First Privacy**: Prioritize `stdio` transport to keep data flow entirely within the user's local machine unless remote `SSE` is explicitly required.

## 4. User Experience (UX) for CLI & Logs
* **Actionable Logs**: Logs should clearly indicate when the MCP server is connecting, failing, or successfully executing a command on a ChangeDetection.io instance.
* **Informative Stats**: Monitoring tools should return a human-readable summary of watch states and changes, not just raw API responses.
* **Clean Configuration**: Use a single, well-structured configuration file (e.g., `config.toml`) that is easy to understand and modify.
