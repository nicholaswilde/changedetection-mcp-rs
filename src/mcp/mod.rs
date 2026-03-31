use crate::api::Client;
use async_trait::async_trait;
use mcp_sdk_rs::error::{Error, ErrorCode};
use mcp_sdk_rs::server::{Server, ServerHandler};
use mcp_sdk_rs::transport::stdio::StdioTransport;
use mcp_sdk_rs::types::{ClientCapabilities, Implementation, ServerCapabilities, Tool};
use std::sync::Arc;
use tokio::io::{stdin, stdout, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::mpsc;

pub struct McpServer {
    client: Arc<Client>,
}

impl McpServer {
    pub fn new(client: Client) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("Starting MCP server...");
        let (stdin_tx, stdin_rx) = mpsc::channel::<String>(32);
        let (stdout_tx, mut stdout_rx) = mpsc::channel::<String>(32);

        let transport = Arc::new(StdioTransport::new(stdin_rx, stdout_tx));
        let handler = Arc::new(self);
        let server = Server::new(transport, handler);

        // Keep stdin_tx alive by passing it to the task and keeping the task running
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdin());
            let mut line = String::new();
            while let Ok(n) = reader.read_line(&mut line).await {
                if n == 0 { break; }
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    // mcp-sdk-rs StdioTransport::receive unwraps serde_json::from_str,
                    // so we MUST ensure it is valid JSON before sending it to the channel.
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                        let mut msg = trimmed.to_string();
                        // mcp-sdk-rs expects "initialized" notification, but protocol says "notifications/initialized"
                        if val.get("method").and_then(|v| v.as_str()) == Some("notifications/initialized") {
                            msg = msg.replace("notifications/initialized", "initialized");
                        }
                        
                        if let Err(e) = stdin_tx.send(msg).await {
                            tracing::error!("Failed to send to stdin channel: {}", e);
                            break;
                        }
                    } else {
                        tracing::warn!("Ignored non-JSON input: {}", trimmed);
                    }
                }
                line.clear();
            }
            tracing::info!("Stdin reader task stopped.");
        });

        // Handle stdout
        tokio::spawn(async move {
            let mut writer = BufWriter::new(stdout());
            while let Some(message) = stdout_rx.recv().await {
                let _ = writer.write_all(message.as_bytes()).await;
                let _ = writer.write_all(b"\n").await;
                let _ = writer.flush().await;
            }
        });

        server.start().await?;
        tracing::info!("MCP server stopped.");
        Ok(())
    }
}

#[async_trait]
impl ServerHandler for McpServer {
    async fn initialize(
        &self,
        _implementation: Implementation,
        _capabilities: ClientCapabilities,
    ) -> Result<ServerCapabilities, Error> {
        let mut capabilities = ServerCapabilities::default();
        capabilities.tools = Some(serde_json::json!({}));
        Ok(capabilities)
    }

    async fn shutdown(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn handle_method(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Error> {
        match method {
            "list_watches" => {
                let watches = self.client.list_watches().await
                    .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                Ok(serde_json::to_value(watches)?)
            }
            "get_watch_details" => {
                let uuid = params.as_ref().and_then(|p| p.get("uuid")).and_then(|v| v.as_str())
                    .ok_or_else(|| Error::protocol(ErrorCode::InvalidParams, "Missing 'uuid' parameter"))?;
                let watch = self.client.get_watch_details(uuid).await
                    .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                Ok(serde_json::to_value(watch)?)
            }
            "create_watch" => {
                let url = params.as_ref().and_then(|p| p.get("url")).and_then(|v| v.as_str())
                    .ok_or_else(|| Error::protocol(ErrorCode::InvalidParams, "Missing 'url' parameter"))?;
                let tag = params.as_ref().and_then(|p| p.get("tag")).and_then(|v| v.as_str());
                let result = self.client.create_watch(url, tag).await
                    .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                Ok(serde_json::to_value(result)?)
            }
            "delete_watch" => {
                let uuid = params.as_ref().and_then(|p| p.get("uuid")).and_then(|v| v.as_str())
                    .ok_or_else(|| Error::protocol(ErrorCode::InvalidParams, "Missing 'uuid' parameter"))?;
                let result = self.client.delete_watch(uuid).await
                    .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                Ok(serde_json::to_value(result)?)
            }
            "trigger_check" => {
                let uuid = params.as_ref().and_then(|p| p.get("uuid")).and_then(|v| v.as_str())
                    .ok_or_else(|| Error::protocol(ErrorCode::InvalidParams, "Missing 'uuid' parameter"))?;
                let result = self.client.trigger_check(uuid).await
                    .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                Ok(serde_json::to_value(result)?)
            }
            "tools/list" => {
                let tools = vec![
                    Tool {
                        name: "list_watches".to_string(),
                        description: "List all watches in ChangeDetection.io".to_string(),
                        input_schema: None,
                        annotations: None,
                    },
                    Tool {
                        name: "get_watch_details".to_string(),
                        description: "Get details of a specific watch".to_string(),
                        input_schema: None,
                        annotations: None,
                    },
                    Tool {
                        name: "create_watch".to_string(),
                        description: "Create a new watch".to_string(),
                        input_schema: None,
                        annotations: None,
                    },
                    Tool {
                        name: "delete_watch".to_string(),
                        description: "Delete a specific watch".to_string(),
                        input_schema: None,
                        annotations: None,
                    },
                    Tool {
                        name: "trigger_check".to_string(),
                        description: "Trigger a re-check for a specific watch".to_string(),
                        input_schema: None,
                        annotations: None,
                    },
                ];
                Ok(serde_json::json!({ "tools": tools }))
            }
            _ => Err(Error::protocol(ErrorCode::MethodNotFound, format!("Method not found: {}", method))),
        }
    }
}
