# changedetection-mcp-rs

A Model Context Protocol (MCP) server for [ChangeDetection.io](https://changedetection.io/), built in Rust.

## Features

This server provides the following MCP tools to interact with a ChangeDetection.io instance:

- `list_watches`: Retrieve a list of all current website watches.
- `get_watch_details`: Fetch detailed information for a specific watch using its UUID.
- `create_watch`: Add a new URL to track for changes.
- `delete_watch`: Remove an existing watch.
- `trigger_check`: Manually force a re-check for a specific watch.

## Prerequisites

- A running [ChangeDetection.io](https://changedetection.io/) instance.
- An API Key from the ChangeDetection.io settings (ensure "API Access Token Enabled" is checked).

## Installation

### From Source

```bash
git clone https://github.com/nicholaswilde/changedetection-mcp-rs.git
cd changedetection-mcp-rs
cargo build --release
```

## Configuration

The server requires the following environment variables:

- `CHANGEDETECTION_API_KEY`: Your ChangeDetection.io API token.
- `CHANGEDETECTION_BASE_URL`: The base URL of your ChangeDetection.io instance (e.g., `http://localhost:5000` or `https://cd.example.com`).

## Usage

### Claude Desktop

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

### MCP Inspector

To test the server using the MCP Inspector:

```bash
CHANGEDETECTION_API_KEY=your_key CHANGEDETECTION_BASE_URL=your_url npx @modelcontextprotocol/inspector ./target/release/changedetection-mcp-rs
```

## License

MIT
