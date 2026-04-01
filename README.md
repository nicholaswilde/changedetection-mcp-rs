# 🔄 ChangeDetection.io MCP Server (Rust) 🤖

[![task](https://img.shields.io/badge/Task-Enabled-brightgreen?style=for-the-badge&logo=task&logoColor=white)](https://taskfile.dev/#/)

> [!WARNING]
> This project is currently in active development (v0.1.0) and is **not production-ready**.
> Features may change, and breaking changes may occur without notice. **Use this MCP server at your own risk.**

A Rust implementation of a [ChangeDetection.io](https://changedetection.io/) [MCP (Model Context Protocol) server](https://modelcontextprotocol.io/docs/getting-started/intro). This server connects to a ChangeDetection.io instance and exposes tools to monitor website changes via the Model Context Protocol.

## ✨ Features

- **Standard Tools:**
    - `list_watches`: List all watches in ChangeDetection.io.
    - `get_watch_details`: Get details of a specific watch.
    - `create_watch`: Create a new watch.
    - `delete_watch`: Delete a specific watch.
    - `trigger_check`: Trigger a re-check for a specific watch.
- **Multi-Transport Support:**
  - **Stdio:** Default transport for local integrations (e.g., Claude Desktop).
  - **HTTP/JSON-RPC:** Remote transport for testing and external clients.
- **Robust Configuration:** Supports configuration via CLI arguments and environment variables.
- **Security & Privacy:**
  - **API Key Authentication:** Connects to ChangeDetection.io using the `x-api-key` header.

## 🛠️ Build

To build the project, you need a Rust toolchain installed.

### Local Build

```bash
# Build in release mode
task build:local
```

The binary will be available at `target/release/changedetection-mcp-rs`.

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
# Run unit tests
task test

# Run Hurl integration tests (requires running server)
task test:hurl

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

Contributions are welcome! Please follow standard Rust coding conventions and ensure all tests pass (`task check`) before submitting features.

## ⚖️ License

[Apache License 2.0](LICENSE)

## ✍️ Author

This project was started in 2026 by [Nicholas Wilde](https://github.com/nicholaswilde/).
