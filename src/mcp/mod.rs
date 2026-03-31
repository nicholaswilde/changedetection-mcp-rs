use crate::api::Client;
use async_trait::async_trait;
use mcp_sdk_rs::error::{Error, ErrorCode};
use mcp_sdk_rs::server::{Server, ServerHandler};
use mcp_sdk_rs::transport::stdio::StdioTransport;
use mcp_sdk_rs::types::{ClientCapabilities, Implementation, ServerCapabilities};
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
        let (stdin_tx, stdin_rx) = mpsc::channel::<String>(32);
        let (stdout_tx, mut stdout_rx) = mpsc::channel::<String>(32);

        let transport = Arc::new(StdioTransport::new(stdin_rx, stdout_tx));
        let handler = Arc::new(self);
        let server = Server::new(transport, handler);

        // Handle stdin
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdin());
            let mut line = String::new();
            while let Ok(n) = reader.read_line(&mut line).await {
                if n == 0 { break; }
                let _ = stdin_tx.send(line.trim().to_string()).await;
                line.clear();
            }
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
        Ok(ServerCapabilities::default())
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
            _ => Err(Error::protocol(ErrorCode::MethodNotFound, format!("Method not found: {}", method))),
        }
    }
}
