use crate::api::Client;
use crate::cli::Transport;
use async_trait::async_trait;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use mcp_sdk_rs::error::{Error, ErrorCode};
use mcp_sdk_rs::server::{Server, ServerHandler};
use mcp_sdk_rs::transport::stdio::StdioTransport;
use mcp_sdk_rs::types::{ClientCapabilities, Implementation, ServerCapabilities, Tool, ToolSchema};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{stdin, stdout, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::mpsc;
use tracing::Instrument;

#[derive(JsonSchema, Deserialize, Debug)]
pub struct ListWatchesArgs {
    /// Optional tag to filter watches
    pub tag: Option<String>,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct SearchWatchesArgs {
    /// The search query (URL or title)
    pub query: String,
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
pub struct UpdateWatchArgs {
    /// The UUID of the watch to update
    pub uuid: String,
    /// The URL to watch
    pub url: Option<String>,
    /// Optional title for the watch
    pub title: Option<String>,
    /// Optional tag to assign to the watch
    pub tag: Option<String>,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct DeleteWatchArgs {
    /// The UUID of the watch to delete
    pub uuid: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct CreateTagArgs {
    /// The title of the tag
    pub title: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct GetTagDetailsArgs {
    /// The UUID of the tag
    pub uuid: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct UpdateTagArgs {
    /// The UUID of the tag to update
    pub uuid: String,
    /// The title of the tag
    pub title: Option<String>,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct DeleteTagArgs {
    /// The UUID of the tag to delete
    pub uuid: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct AddNotificationArgs {
    /// The Apprise-compatible URL to add (e.g., mailto://test@example.com)
    pub notification_url: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct UpdateNotificationsArgs {
    /// The list of Apprise-compatible URLs to replace the current set
    pub notification_urls: Vec<String>,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct DeleteNotificationArgs {
    /// The Apprise-compatible URL to delete
    pub notification_url: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct TriggerCheckArgs {
    /// The UUID of the watch to trigger a check for
    pub uuid: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct WatchUuidArgs {
    /// The UUID of the watch
    pub uuid: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct GetWatchHistoryArgs {
    /// The UUID of the watch
    pub uuid: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct GetWatchDiffArgs {
    /// The UUID of the watch
    pub uuid: String,
    /// The timestamp of the source snapshot
    pub from_timestamp: String,
    /// The timestamp of the target snapshot
    pub to_timestamp: String,
    /// The format of the diff (e.g., "text", "markdown")
    pub format: Option<String>,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct GetSnapshotContentArgs {
    /// The UUID of the watch
    pub uuid: String,
    /// The timestamp of the snapshot
    pub timestamp: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct ImportWatchesArgs {
    /// The list of URLs to import
    pub urls: Vec<String>,
    /// The tag to assign to the imported watches
    pub tag: Option<String>,
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
        let request_id = uuid::Uuid::new_v4().to_string();
        let span = tracing::info_span!("mcp_request", %method, %request_id);

        async move {
            let params_tokens = params.as_ref().map_or(0, |p| p.to_string().len());

            let result = match method {
                "list_watches" => {
                    let args: ListWatchesArgs =
                        serde_json::from_value(params.unwrap_or(serde_json::json!({})))?;
                    let watches = self
                        .client
                        .list_watches(args.tag.as_deref())
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(watches)?)
                }
                "search_watches" => {
                    let args: SearchWatchesArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let watches =
                        self.client.search_watches(&args.query).await.map_err(|e| {
                            Error::protocol(ErrorCode::InternalError, e.to_string())
                        })?;
                    Ok(serde_json::to_value(watches)?)
                }
                "get_watch_details" => {
                    let args: GetWatchDetailsArgs =
                        serde_json::from_value(params.ok_or_else(|| {
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
                    let args: CreateWatchArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result = self
                        .client
                        .create_watch(&args.url, args.tag.as_deref())
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "update_watch" => {
                    let mut payload: serde_json::Value =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;

                    let uuid = payload
                        .get("uuid")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| Error::protocol(ErrorCode::InvalidParams, "Missing uuid"))?
                        .to_string();

                    // Remove uuid from payload to avoid sending it in the body
                    if let Some(map) = payload.as_object_mut() {
                        map.remove("uuid");
                    }

                    let result = self
                        .client
                        .update_watch(&uuid, payload)
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "delete_watch" => {
                    let args: DeleteWatchArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result =
                        self.client.delete_watch(&args.uuid).await.map_err(|e| {
                            Error::protocol(ErrorCode::InternalError, e.to_string())
                        })?;
                    Ok(serde_json::to_value(result)?)
                }
                "list_tags" => {
                    let result =
                        self.client.list_tags().await.map_err(|e| {
                            Error::protocol(ErrorCode::InternalError, e.to_string())
                        })?;
                    Ok(serde_json::to_value(result)?)
                }
                "create_tag" => {
                    let args: CreateTagArgs = serde_json::from_value(params.ok_or_else(|| {
                        Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                    })?)?;
                    let result =
                        self.client.create_tag(&args.title).await.map_err(|e| {
                            Error::protocol(ErrorCode::InternalError, e.to_string())
                        })?;
                    Ok(serde_json::to_value(result)?)
                }
                "get_tag_details" => {
                    let args: GetTagDetailsArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result =
                        self.client.get_tag_details(&args.uuid).await.map_err(|e| {
                            Error::protocol(ErrorCode::InternalError, e.to_string())
                        })?;
                    Ok(serde_json::to_value(result)?)
                }
                "update_tag" => {
                    let mut payload: serde_json::Value =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let uuid = payload
                        .get("uuid")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| Error::protocol(ErrorCode::InvalidParams, "Missing uuid"))?
                        .to_string();
                    if let Some(map) = payload.as_object_mut() {
                        map.remove("uuid");
                    }
                    let result =
                        self.client.update_tag(&uuid, payload).await.map_err(|e| {
                            Error::protocol(ErrorCode::InternalError, e.to_string())
                        })?;
                    Ok(serde_json::to_value(result)?)
                }
                "delete_tag" => {
                    let args: DeleteTagArgs = serde_json::from_value(params.ok_or_else(|| {
                        Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                    })?)?;
                    let result =
                        self.client.delete_tag(&args.uuid).await.map_err(|e| {
                            Error::protocol(ErrorCode::InternalError, e.to_string())
                        })?;
                    Ok(serde_json::to_value(result)?)
                }
                "trigger_check" => {
                    let args: TriggerCheckArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result =
                        self.client.trigger_check(&args.uuid).await.map_err(|e| {
                            Error::protocol(ErrorCode::InternalError, e.to_string())
                        })?;
                    Ok(serde_json::to_value(result)?)
                }
                "get_watch_history" => {
                    let args: GetWatchHistoryArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result = self
                        .client
                        .get_watch_history(&args.uuid)
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "get_watch_diff" => {
                    let args: GetWatchDiffArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result = self
                        .client
                        .get_watch_diff(
                            &args.uuid,
                            &args.from_timestamp,
                            &args.to_timestamp,
                            args.format.as_deref(),
                        )
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "get_snapshot_content" => {
                    let args: GetSnapshotContentArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result = self
                        .client
                        .get_snapshot_content(&args.uuid, &args.timestamp)
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "import_watches" => {
                    let args: ImportWatchesArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result = self
                        .client
                        .import_watches(args.urls, args.tag.as_deref())
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "pause_watch" => {
                    let args: WatchUuidArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result = self
                        .client
                        .set_watch_state(&args.uuid, "paused", "paused")
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "unpause_watch" => {
                    let args: WatchUuidArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result = self
                        .client
                        .set_watch_state(&args.uuid, "paused", "unpaused")
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "mute_notifications" => {
                    let args: WatchUuidArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result = self
                        .client
                        .set_watch_state(&args.uuid, "muted", "muted")
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "unmute_notifications" => {
                    let args: WatchUuidArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result = self
                        .client
                        .set_watch_state(&args.uuid, "muted", "unmuted")
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "get_system_info" => {
                    let info =
                        self.client.get_system_info().await.map_err(|e| {
                            Error::protocol(ErrorCode::InternalError, e.to_string())
                        })?;
                    Ok(serde_json::to_value(info)?)
                }
                "get_full_spec" => {
                    let spec =
                        self.client.get_full_spec().await.map_err(|e| {
                            Error::protocol(ErrorCode::InternalError, e.to_string())
                        })?;
                    Ok(serde_json::to_value(spec)?)
                }
                "list_notifications" => {
                    let result = self
                        .client
                        .list_notifications()
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "add_notification" => {
                    let args: AddNotificationArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result = self
                        .client
                        .add_notification(&args.notification_url)
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "update_notifications" => {
                    let args: UpdateNotificationsArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result = self
                        .client
                        .update_notifications(args.notification_urls)
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "delete_notification" => {
                    let args: DeleteNotificationArgs =
                        serde_json::from_value(params.ok_or_else(|| {
                            Error::protocol(ErrorCode::InvalidParams, "Missing parameters")
                        })?)?;
                    let result = self
                        .client
                        .delete_notification(&args.notification_url)
                        .await
                        .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;
                    Ok(serde_json::to_value(result)?)
                }
                "tools/list" => {
                    let tools = vec![
                        Tool {
                            name: "get_system_info".to_string(),
                            description: "Retrieve ChangeDetection.io system status and version"
                                .to_string(),
                            input_schema: None,
                            annotations: None,
                        },
                        Tool {
                            name: "get_full_spec".to_string(),
                            description: "Retrieve the full OpenAPI specification of the ChangeDetection.io instance"
                                .to_string(),
                            input_schema: None,
                            annotations: None,
                        },
                        Tool {
                            name: "list_watches".to_string(),
                            description: "List all watches in ChangeDetection.io".to_string(),
                            input_schema: Some(get_schema::<ListWatchesArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "search_watches".to_string(),
                            description: "Search for watches by URL or title".to_string(),
                            input_schema: Some(get_schema::<SearchWatchesArgs>()),
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
                            name: "update_watch".to_string(),
                            description: "Update a specific watch".to_string(),
                            input_schema: Some(get_schema::<UpdateWatchArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "delete_watch".to_string(),
                            description: "Delete a specific watch".to_string(),
                            input_schema: Some(get_schema::<DeleteWatchArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "list_tags".to_string(),
                            description: "List all tags in ChangeDetection.io".to_string(),
                            input_schema: None,
                            annotations: None,
                        },
                        Tool {
                            name: "create_tag".to_string(),
                            description: "Create a new tag".to_string(),
                            input_schema: Some(get_schema::<CreateTagArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "get_tag_details".to_string(),
                            description: "Get details of a specific tag".to_string(),
                            input_schema: Some(get_schema::<GetTagDetailsArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "update_tag".to_string(),
                            description: "Update a specific tag".to_string(),
                            input_schema: Some(get_schema::<UpdateTagArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "delete_tag".to_string(),
                            description: "Delete a specific tag".to_string(),
                            input_schema: Some(get_schema::<DeleteTagArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "trigger_check".to_string(),
                            description: "Trigger a re-check for a specific watch".to_string(),
                            input_schema: Some(get_schema::<TriggerCheckArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "pause_watch".to_string(),
                            description: "Pause a watch (stop checking for changes)".to_string(),
                            input_schema: Some(get_schema::<WatchUuidArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "unpause_watch".to_string(),
                            description: "Resume checking for changes on a watch".to_string(),
                            input_schema: Some(get_schema::<WatchUuidArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "mute_notifications".to_string(),
                            description: "Stop sending notifications for a watch".to_string(),
                            input_schema: Some(get_schema::<WatchUuidArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "unmute_notifications".to_string(),
                            description: "Resume sending notifications for a watch".to_string(),
                            input_schema: Some(get_schema::<WatchUuidArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "get_watch_history".to_string(),
                            description: "Get the history of snapshots for a specific watch"
                                .to_string(),
                            input_schema: Some(get_schema::<GetWatchHistoryArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "get_watch_diff".to_string(),
                            description: "Get the difference between two snapshots of a watch"
                                .to_string(),
                            input_schema: Some(get_schema::<GetWatchDiffArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "get_snapshot_content".to_string(),
                            description: "Get the full content of a specific watch snapshot"
                                .to_string(),
                            input_schema: Some(get_schema::<GetSnapshotContentArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "import_watches".to_string(),
                            description: "Bulk import a list of URLs as new watches".to_string(),
                            input_schema: Some(get_schema::<ImportWatchesArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "list_notifications".to_string(),
                            description: "List all global notification endpoints".to_string(),
                            input_schema: None,
                            annotations: None,
                        },
                        Tool {
                            name: "add_notification".to_string(),
                            description: "Add a new global notification endpoint".to_string(),
                            input_schema: Some(get_schema::<AddNotificationArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "update_notifications".to_string(),
                            description: "Replace all global notification endpoints".to_string(),
                            input_schema: Some(get_schema::<UpdateNotificationsArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "delete_notification".to_string(),
                            description: "Delete a global notification endpoint".to_string(),
                            input_schema: Some(get_schema::<DeleteNotificationArgs>()),
                            annotations: None,
                        },
                    ];
                    Ok(serde_json::json!({ "tools": tools }))
                }
                _ => Err(Error::protocol(
                    ErrorCode::MethodNotFound,
                    format!("Method not found: {}", method),
                )),
            };

            let result_tokens = result.as_ref().map_or(0, |r| r.to_string().len());
            tracing::info!(
                "Token usage: (params: {}, result: {})",
                params_tokens,
                result_tokens
            );

            result
        }
        .instrument(span)
        .await
    }
}
