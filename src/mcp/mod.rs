use crate::api::Client;
use crate::cli::Transport;
use async_trait::async_trait;
use axum::{
    extract::{State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use mcp_sdk_rs::error::{Error, ErrorCode};
use mcp_sdk_rs::server::{Server, ServerHandler};
use mcp_sdk_rs::transport::stdio::StdioTransport;
use mcp_sdk_rs::types::{ClientCapabilities, Implementation, ServerCapabilities, Tool, ToolSchema};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{stdin, stdout, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::mpsc;

#[derive(JsonSchema, Deserialize, Debug)]
pub struct ListWatchesArgs {
    /// Optional tag to filter watches
    pub tag: Option<String>,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct GetWatchDetailsArgs {
    /// The UUID of the watch
    pub uuid: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct CreateWatchArgs {
    /// The URL to watch
    pub url: String,
    /// Optional tag to assign to the watch
    pub tag: Option<String>,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct DeleteWatchArgs {
    /// The UUID of the watch to delete
    pub uuid: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct TriggerCheckArgs {
    /// The UUID of the watch to trigger a check for
    pub uuid: String,
}

pub fn get_schema<T: JsonSchema>() -> ToolSchema {
    let schema = schema_for!(T);
    let schema_val = serde_json::to_value(&schema).expect("Failed to serialize schema");
    
    ToolSchema {
        properties: schema_val.get("properties").cloned(),
        required: schema_val.get("required").and_then(|v| {
            v.as_array().map(|a| {
                a.iter()
                    .filter_map(|s| s.as_str().map(|s| s.to_string()))
                    .collect()
            })
        }),
    }
}

pub struct McpServer {
    client: Arc<Client>,
}

impl McpServer {
    pub fn new(client: Client) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    pub async fn run(self, transport_type: Transport, host: &str, port: u16) -> anyhow::Result<()> {
        match transport_type {
            Transport::Stdio => self.run_stdio().await,
            Transport::Http => self.run_http(host, port).await,
        }
    }

    async fn run_stdio(self) -> anyhow::Result<()> {
        tracing::info!("Starting MCP server via stdio...");
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
                if n == 0 {
                    break;
                }
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                        let mut msg = trimmed.to_string();
                        if val.get("method").and_then(|v| v.as_str())
                            == Some("notifications/initialized")
                        {
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

    async fn run_http(self, host: &str, port: u16) -> anyhow::Result<()> {
        tracing::info!("Starting MCP server via HTTP on {}:{}...", host, port);
        
        let state = Arc::new(self);
        
        let app = Router::new()
            .route("/", post(http_rpc_handler))
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;
        axum::serve(listener, app).await?;
        
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<serde_json::Value>,
    id: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct RpcResponse {
    jsonrpc: String,
    result: Option<serde_json::Value>,
    error: Option<RpcError>,
    id: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct RpcError {
    code: i32,
    message: String,
}

async fn http_rpc_handler(
    State(server): State<Arc<McpServer>>,
    Json(payload): Json<RpcRequest>,
) -> (StatusCode, Json<RpcResponse>) {
    match server.handle_method(&payload.method, payload.params).await {
        Ok(result) => (
            StatusCode::OK,
            Json(RpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(result),
                error: None,
                id: payload.id,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(RpcError {
                    code: -32000, // General internal error
                    message: e.to_string(),
                }),
                id: payload.id,
            }),
        ),
    }
}

#[async_trait]
impl ServerHandler for McpServer {
    async fn initialize(
        &self,
        _implementation: Implementation,
        _capabilities: ClientCapabilities,
    ) -> Result<ServerCapabilities, Error> {
        let capabilities = ServerCapabilities {
            tools: Some(serde_json::json!({})),
            ..Default::default()
        };
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
        let params_tokens = params.as_ref().map_or(0, |p| p.to_string().len());
        
        let result = async {
            match method {
                "list_watches" => {
                    let args: ListWatchesArgs = serde_json::from_value(params.unwrap_or(serde_json::json!({})))?;
                    let watches = self
                        .client
                        .list_watches(args.tag.as_deref())
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(watches)?)
                }
                "get_watch_details" => {
                    let args: GetWatchDetailsArgs = serde_json::from_value(params.ok_or_else(|| {
                        Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                    })?)?;
                    let watch = self
                        .client
                        .get_watch_details(&args.uuid)
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(watch)?)
                }
                "create_watch" => {
                    let args: CreateWatchArgs = serde_json::from_value(params.ok_or_else(|| {
                        Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                    })?)?;
                    let result = self
                        .client
                        .create_watch(&args.url, args.tag.as_deref())
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "delete_watch" => {
                    let args: DeleteWatchArgs = serde_json::from_value(params.ok_or_else(|| {
                        Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                    })?)?;
                    let result = self
                        .client
                        .delete_watch(&args.uuid)
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "trigger_check" => {
                    let args: TriggerCheckArgs = serde_json::from_value(params.ok_or_else(|| {
                        Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                    })?)?;
                    let result = self
                        .client
                        .trigger_check(&args.uuid)
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "tools/list" => {
                    let tools = vec![
                        Tool {
                            name: "list_watches".to_string(),
                            description: "List all watches in ChangeDetection.io".to_string(),
                            input_schema: Some(get_schema::<ListWatchesArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "get_watch_details".to_string(),
                            description: "Get details of a specific watch".to_string(),
                            input_schema: Some(get_schema::<GetWatchDetailsArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "create_watch".to_string(),
                            description: "Create a new watch".to_string(),
                            input_schema: Some(get_schema::<CreateWatchArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "delete_watch".to_string(),
                            description: "Delete a specific watch".to_string(),
                            input_schema: Some(get_schema::<DeleteWatchArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "trigger_check".to_string(),
                            description: "Trigger a re-check for a specific watch".to_string(),
                            input_schema: Some(get_schema::<TriggerCheckArgs>()),
                            annotations: None,
                        },
                    ];
                    Ok(serde_json::json!({ "tools": tools }))
                }
                _ => Err(Error::protocol(
                    ErrorCode::MethodNotFound,
                    format!("Method not found: {}", method),
                )),
            }
        }.await;

        let result_tokens = result.as_ref().map_or(0, |r| r.to_string().len());
        tracing::info!(
            "Token usage: (params: {}, result: {})",
            params_tokens,
            result_tokens
        );

        result
    }
}
