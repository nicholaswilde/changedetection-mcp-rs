# 🔄 ChangeDetection.io MCP Server (Rust) 🤖

[![task](https://img.shields.io/badge/Task-Enabled-brightgreen?style=for-the-badge&logo=task&logoColor=white)](https://taskfile.dev/#/)

> [!WARNING]
> This project is currently in active development (v0.1.0) and is **not production-ready**.
> Features may change, and breaking changes may occur without notice. **Use this MCP server at your own risk.**

A Rust implementation of a [ChangeDetection.io](https://changedetection.io/) [MCP (Model Context Protocol) server](https://modelcontextprotocol.io/docs/getting-started/intro). This server connects to a ChangeDetection.io instance and exposes tools to monitor website changes via the Model Context Protocol.

## ✨ Features

The server provides a consolidated set of tools for efficient interaction, optimized for lower token usage and better context management.

- **`watch_ops`**: Comprehensive watch management.
    - **Actions:** `List`, `Search`, `Get`, `Create`, `Update`, `Delete`, `Trigger`, `Pause`, `Unpause`, `Mute`, `Unmute`, `Import`, `SetSelectors`, `SetFetcher`, `ConfigureNotifications`, `ListErrors`, `ListByProcessor`.
- **`tag_ops`**: Management of watch tags.
    - **Actions:** `List`, `Create`, `Get`, `Update`, `Delete`.
- **`notification_ops`**: Global notification endpoint management.
    - **Actions:** `List`, `Add`, `Update`, `Delete`.
- **`history_ops`**: Watch history, snapshots, and diffs.
    - **Actions:** `GetHistory`, `GetDiff`, `GetContent`, `GetScreenshot`, `ListAll`, `SetLimit`, `GetInfo`.
- **`system_ops`**: System discovery and global settings.
    - **Actions:** `GetInfo`, `GetSpec`, `ListFetchers`, `ListProxies`, `GetSettings`, `ListProcessors`.
- **`maintenance_ops`**: System maintenance tasks.
    - **Actions:** `Backup`, `Export`.

### 📚 MCP Resources

The server exposes key data through the MCP Resources protocol, allowing LLMs to read content directly via URIs.

- **`watches://{uuid}/latest`**: Access the most recent snapshot content for a specific watch.
- **`system://openapi-spec`**: Retrieve the full OpenAPI specification for the ChangeDetection.io API.

### 🚀 Optimization Features

- **Consolidated Tools:** Reduces tool discovery overhead by grouping related operations.
- **Pagination:** Supports `page` and `per_page` parameters for list operations.
- **Field Selection:** Allows requesting only specific fields to minimize response size.
- **Multi-Transport Support:** Stdio (default) and HTTP/JSON-RPC.

## 🛠️ Build

To build the project, you need a Rust toolchain installed.

### Local Build

```bash
# Build in release mode
task build:local
```

### Cross-Compilation

```bash
# Build for amd64 (Linux)
task build:amd64

# Build for arm64 (Linux)
task build:arm64
```

## 🚀 Usage

### ⌨️ Command Line Interface

The server is configured via CLI arguments or environment variables.

```bash
# Run via Stdio (default)
./target/release/changedetection-mcp-rs --api-key your_key

# Run via HTTP
./target/release/changedetection-mcp-rs --transport http --port 3000 --api-key your_key
```

### 🤖 Configuration Example (Claude Desktop)

Add the following to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "changedetection": {
      "command": "/path/to/changedetection-mcp-rs/target/release/changedetection-mcp-rs",
      "env": {
        "CHANGEDETECTION_API_KEY": "your_api_key_here",
        "CHANGEDETECTION_BASE_URL": "http://your_instance_url:5000"
      }
    }
  }
}
```

## 🧪 Testing

The project uses [go-task](https://taskfile.dev/) for development tasks.

```bash
# Run all CI checks (fmt, lint, unit tests)
task test:ci

# Run unit tests only
task test

# Run MCP integration tests
task test:integration

# Run MCP Inspector (requires npx)
task inspector
```

### 📊 Coverage

The project uses `cargo-llvm-cov` for code coverage analysis.

```bash
# Show coverage summary in console
task coverage
```

## 🤝 Contributing

Contributions are welcome! Please follow standard Rust coding conventions and ensure all tests pass (`task test:ci`) before submitting features.

## ⚖️ License

[Apache License 2.0](LICENSE)

## ✍️ Author

This project was started in 2026 by [Nicholas Wilde](https://github.com/nicholaswilde/).
