use crate::api::{Client, Watch};
use crate::cli::Transport;
use async_trait::async_trait;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use base64::{engine::general_purpose, Engine as _};
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

pub mod helpers;

// --- Consolidated Category Tools ---

#[derive(JsonSchema, Deserialize, Debug)]
pub struct PaginationArgs {
    /// Page number (1-indexed)
    pub page: Option<usize>,
    /// Number of items per page
    pub per_page: Option<usize>,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct CommonArgs {
    /// Optional pagination parameters
    pub pagination: Option<PaginationArgs>,
    /// Optional list of fields to include in the response
    pub fields: Option<Vec<String>>,
}

#[derive(JsonSchema, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum WatchAction {
    List,
    Search,
    Get,
    Create,
    Update,
    Delete,
    Trigger,
    Pause,
    Unpause,
    Mute,
    Unmute,
    Import,
    SetSelectors,
    SetFetcher,
    ConfigureNotifications,
    ListErrors,
    ListByProcessor,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct WatchOpsArgs {
    /// The action to perform
    pub action: WatchAction,
    /// The UUID of the watch (required for most actions)
    pub uuid: Option<String>,
    /// The URL to watch (for Create/Update)
    pub url: Option<String>,
    /// Optional tag or tag filter
    pub tag: Option<String>,
    /// Optional title for the watch
    pub title: Option<String>,
    /// The search query (for Search)
    pub query: Option<String>,
    /// Optional state filter (for List)
    pub state: Option<String>,
    /// The processor name (for ListByProcessor)
    pub processor: Option<String>,
    /// List of URLs to import (for Import)
    pub urls: Option<Vec<String>>,
    /// Optional CSS filter
    pub css_filter: Option<String>,
    /// Optional XPath filter
    pub xpath_filter: Option<String>,
    /// Optional JSON filter
    pub json_filter: Option<String>,
    /// The fetcher engine to use
    pub fetcher: Option<String>,
    /// List of Apprise-compatible notification URLs
    pub notification_urls: Option<Vec<String>>,
    /// Optional notification title override
    pub notification_title: Option<String>,
    /// Optional notification body override
    pub notification_body: Option<String>,
    #[serde(flatten)]
    pub common: CommonArgs,
}

#[derive(JsonSchema, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum TagAction {
    List,
    Create,
    Get,
    Update,
    Delete,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct TagOpsArgs {
    /// The action to perform
    pub action: TagAction,
    /// The UUID of the tag
    pub uuid: Option<String>,
    /// The title of the tag
    pub title: Option<String>,
    #[serde(flatten)]
    pub common: CommonArgs,
}

#[derive(JsonSchema, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum NotificationAction {
    List,
    Add,
    Update,
    Delete,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct NotificationOpsArgs {
    /// The action to perform
    pub action: NotificationAction,
    /// The Apprise-compatible URL to add or delete
    pub notification_url: Option<String>,
    /// The list of Apprise-compatible URLs for bulk update
    pub notification_urls: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonArgs,
}

#[derive(JsonSchema, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum HistoryAction {
    GetHistory,
    GetDiff,
    GetContent,
    GetScreenshot,
    ListAll,
    SetLimit,
    GetInfo,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct HistoryOpsArgs {
    /// The action to perform
    pub action: HistoryAction,
    /// The UUID of the watch
    pub uuid: Option<String>,
    /// The timestamp of the snapshot
    pub timestamp: Option<String>,
    /// The source snapshot timestamp (for GetDiff)
    pub from_timestamp: Option<String>,
    /// The target snapshot timestamp (for GetDiff)
    pub to_timestamp: Option<String>,
    /// The format of the output (e.g. "text", "markdown")
    pub format: Option<String>,
    /// The maximum number of snapshots to keep
    pub limit: Option<i32>,
    /// Optional tag filter
    pub tag: Option<String>,
    #[serde(flatten)]
    pub common: CommonArgs,
}

#[derive(JsonSchema, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum SystemAction {
    GetInfo,
    GetSpec,
    ListFetchers,
    ListProxies,
    GetSettings,
    ListProcessors,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct SystemOpsArgs {
    /// The action to perform
    pub action: SystemAction,
    #[serde(flatten)]
    pub common: CommonArgs,
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
                "watch_ops" => {
                    let args: WatchOpsArgs = serde_json::from_value(params.as_ref().cloned().unwrap_or(serde_json::json!({})))?;
                    match args.action {
                        WatchAction::List => {
                            let mut watches = self
                                .client
                                .list_watches(args.tag.as_deref())
                                .await
                                .map_err(|e| Error::protocol(ErrorCode::InternalError, e.to_string()))?;

                            if let Some(target_state) = args.state {
                                let mut filtered = std::collections::HashMap::new();
                                for (uuid, mut watch) in watches {
                                    if let Ok(details) = self.client.get_watch_details(&uuid).await {
                                        watch.paused = details.paused;
                                        watch.last_error = details.last_error;

                                        let matches = match target_state.to_lowercase().as_str() {
                                            "paused" => watch.paused.unwrap_or(false),
                                            "unpaused" => !watch.paused.unwrap_or(false),
                                            "error" => {
                                                if let Some(err) = &watch.last_error {
                                                    match err {
                                                        serde_json::Value::Bool(b) => *b,
                                                        serde_json::Value::String(s) => !s.is_empty(),
                                                        _ => true,
                                                    }
                                                } else {
                                                    false
                                                }
                                            }
                                            _ => true,
                                        };

                                        if matches {
                                            filtered.insert(uuid, watch);
                                        }
                                    }
                                }
                                watches = filtered;
                            }

                            // Pagination
                            let (paged, total) = if let Some(p) = args.common.pagination {
                                let (p, t) = helpers::paginate_map(&watches, p.page.unwrap_or(1), p.per_page.unwrap_or(50));
                                (p.into_iter().map(|(k, v)| (k.clone(), (*v).clone())).collect::<std::collections::HashMap<String, Watch>>(), t)
                            } else {
                                let t = watches.len();
                                (watches, t)
                            };

                            // Field selection
                            let mut final_result = serde_json::json!({});
                            if let Some(fields) = args.common.fields {
                                let mut filtered_map = serde_json::Map::new();
                                for (uuid, watch) in paged {
                                    let val = serde_json::to_value(watch)?;
                                    filtered_map.insert(uuid, helpers::filter_fields(&val, &fields));
                                }
                                final_result = serde_json::Value::Object(filtered_map);
                            } else {
                                final_result = serde_json::to_value(paged)?;
                            }

                            Ok(serde_json::json!({
                                "watches": final_result,
                                "total": total
                            }))
                        }
                        WatchAction::Search => {
                            let query = args.query.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing query")
                            })?;
                            let watches = self.client.search_watches(&query).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(watches)?)
                        }
                        WatchAction::Get => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let watch = self.client.get_watch_details(&uuid).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(watch)?)
                        }
                        WatchAction::Create => {
                            let url = args.url.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing url")
                            })?;
                            let result = self.client.create_watch(&url, args.tag.as_deref()).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::Update => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let payload = params.unwrap_or(serde_json::json!({}));
                            let result = self.client.update_watch(&uuid, payload).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::Delete => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let result = self.client.delete_watch(&uuid).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::Trigger => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let result = self.client.trigger_check(&uuid).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::Pause => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let result = self.client.set_watch_state(&uuid, "paused", "paused").await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::Unpause => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let result = self.client.set_watch_state(&uuid, "paused", "unpaused").await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::Mute => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let result = self.client.set_watch_state(&uuid, "muted", "muted").await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::Unmute => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let result = self.client.set_watch_state(&uuid, "muted", "unmuted").await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::Import => {
                            let urls = args.urls.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing urls")
                            })?;
                            let result = self.client.import_watches(urls, args.tag.as_deref()).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::SetSelectors => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let result = self.client.set_watch_selectors(
                                &uuid,
                                args.css_filter.as_deref(),
                                args.xpath_filter.as_deref(),
                                args.json_filter.as_deref(),
                            ).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::SetFetcher => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let fetcher = args.fetcher.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing fetcher")
                            })?;
                            let result = self.client.set_watch_fetcher(&uuid, &fetcher).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::ConfigureNotifications => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let urls = args.notification_urls.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing notification_urls")
                            })?;
                            let result = self.client.configure_watch_notifications(
                                &uuid,
                                urls,
                                args.notification_title.as_deref(),
                                args.notification_body.as_deref(),
                            ).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::ListErrors => {
                            let result = self.client.find_watches_by_error().await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::ListByProcessor => {
                            let processor = args.processor.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing processor")
                            })?;
                            let result = self.client.list_watches_by_processor(&processor).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                    }
                }
                "tag_ops" => {
                    let args: TagOpsArgs = serde_json::from_value(params.as_ref().cloned().unwrap_or(serde_json::json!({})))?;
                    match args.action {
                        TagAction::List => {
                            let tags_val = self.client.list_tags().await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;

                            let tags = tags_val.as_object().ok_or_else(|| {
                                Error::protocol(ErrorCode::InternalError, "Expected object for tags")
                            })?;

                            // Pagination
                            let (paged, total) = if let Some(p) = args.common.pagination {
                                helpers::paginate_map(tags, p.page.unwrap_or(1), p.per_page.unwrap_or(50))
                            } else {
                                let t = tags.len();
                                (tags.iter().collect(), t)
                            };

                            // Field selection
                            let mut filtered_map = serde_json::Map::new();
                            let fields = args.common.fields.as_deref().unwrap_or(&[]);
                            for (uuid, val) in paged {
                                filtered_map.insert(uuid.clone(), helpers::filter_fields(val, fields));
                            }

                            Ok(serde_json::json!({
                                "tags": serde_json::Value::Object(filtered_map),
                                "total": total
                            }))
                        }
                        TagAction::Create => {
                            let title = args.title.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing title")
                            })?;
                            let result = self.client.create_tag(&title).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        TagAction::Get => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let result = self.client.get_tag_details(&uuid).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        TagAction::Update => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let payload = params.as_ref().cloned().unwrap_or(serde_json::json!({}));
                            let result = self.client.update_tag(&uuid, payload).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        TagAction::Delete => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let result = self.client.delete_tag(&uuid).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                    }
                }
                "notification_ops" => {
                    let args: NotificationOpsArgs = serde_json::from_value(params.as_ref().cloned().unwrap_or(serde_json::json!({})))?;
                    match args.action {
                        NotificationAction::List => {
                            let notifications = self.client.list_notifications().await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;

                            // Pagination
                            let (paged, total) = if let Some(p) = args.common.pagination {
                                helpers::paginate_vec(&notifications, p.page.unwrap_or(1), p.per_page.unwrap_or(50))
                            } else {
                                let t = notifications.len();
                                (notifications.iter().collect(), t)
                            };

                            // Field selection (not very useful for Vec<String>, but for consistency)
                            let final_result = if let Some(fields) = args.common.fields {
                                paged.into_iter().map(|s| helpers::filter_fields(&serde_json::Value::String(s.clone()), &fields)).collect::<Vec<_>>()
                            } else {
                                paged.into_iter().map(|s| serde_json::Value::String(s.clone())).collect::<Vec<_>>()
                            };

                            Ok(serde_json::json!({
                                "notifications": final_result,
                                "total": total
                            }))
                        }
                        NotificationAction::Add => {
                            let url = args.notification_url.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing notification_url")
                            })?;
                            let result = self.client.add_notification(&url).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        NotificationAction::Update => {
                            let urls = args.notification_urls.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing notification_urls")
                            })?;
                            let result = self.client.update_notifications(urls).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        NotificationAction::Delete => {
                            let url = args.notification_url.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing notification_url")
                            })?;
                            let result = self.client.delete_notification(&url).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                    }
                }
                "history_ops" => {
                    let args: HistoryOpsArgs = serde_json::from_value(params.as_ref().cloned().unwrap_or(serde_json::json!({})))?;
                    match args.action {
                        HistoryAction::ListAll => {
                            let history_map = self.client.list_all_history(args.tag.as_deref()).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;

                            // Flatten the history map into a Vec
                            let mut history_vec = Vec::new();
                            for (uuid, snapshots) in history_map {
                                for (timestamp, url) in snapshots {
                                    history_vec.push(serde_json::json!({
                                        "watch_uuid": uuid,
                                        "timestamp": timestamp,
                                        "url": url
                                    }));
                                }
                            }

                            // Pagination
                            let (paged, total) = if let Some(p) = args.common.pagination {
                                helpers::paginate_vec(&history_vec, p.page.unwrap_or(1), p.per_page.unwrap_or(50))
                            } else {
                                let t = history_vec.len();
                                (history_vec.iter().collect(), t)
                            };

                            // Field selection
                            let fields = args.common.fields.as_deref().unwrap_or(&[]);
                            let final_result: Vec<serde_json::Value> = paged.into_iter().map(|v| helpers::filter_fields(v, fields)).collect();

                            Ok(serde_json::json!({
                                "history": final_result,
                                "total": total
                            }))
                        }
                        HistoryAction::GetHistory => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let result = self.client.get_watch_history(&uuid).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        HistoryAction::GetDiff => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let from = args.from_timestamp.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing from_timestamp")
                            })?;
                            let to = args.to_timestamp.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing to_timestamp")
                            })?;
                            let result = self.client.get_watch_diff(&uuid, &from, &to, args.format.as_deref()).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        HistoryAction::GetContent => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let ts = args.timestamp.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing timestamp")
                            })?;
                            let result = self.client.get_snapshot_content(&uuid, &ts).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        HistoryAction::GetScreenshot => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let result = self.client.get_watch_screenshot(&uuid).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            let b64 = general_purpose::STANDARD.encode(result);
                            Ok(serde_json::to_value(b64)?)
                        }
                        HistoryAction::SetLimit => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let limit = args.limit.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing limit")
                            })?;
                            let result = self.client.set_history_limit(&uuid, limit).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        HistoryAction::GetInfo => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let ts = args.timestamp.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing timestamp")
                            })?;
                            let result = self.client.get_snapshot_info(&uuid, &ts).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                    }
                }
                "system_ops" => {
                    let args: SystemOpsArgs = serde_json::from_value(params.as_ref().cloned().unwrap_or(serde_json::json!({})))?;
                    match args.action {
                        SystemAction::GetInfo => {
                            let info = self.client.get_system_info().await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;

                            let info_val = serde_json::to_value(info)?;
                            let fields = args.common.fields.as_deref().unwrap_or(&[]);
                            Ok(helpers::filter_fields(&info_val, fields))
                        }
                        SystemAction::GetSpec => {
                            let spec = self.client.get_full_spec().await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(spec)?)
                        }
                        SystemAction::ListFetchers => {
                            let fetchers = self.client.list_fetchers().await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(fetchers)?)
                        }
                        SystemAction::ListProxies => {
                            let proxies = self.client.list_proxies().await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(proxies)?)
                        }
                        SystemAction::GetSettings => {
                            let settings = self.client.get_global_settings().await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(settings)?)
                        }
                        SystemAction::ListProcessors => {
                            let processors = self.client.list_processors().await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(processors)?)
                        }
                    }
                }
                "tools/list" => {
                    let tools = vec![
                        Tool {
                            name: "watch_ops".to_string(),
                            description: "Consolidated operations for watches (list, search, get, create, update, delete, trigger, pause, mute, import, etc)".to_string(),
                            input_schema: Some(get_schema::<WatchOpsArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "tag_ops".to_string(),
                            description: "Consolidated operations for tags (list, create, get, update, delete)".to_string(),
                            input_schema: Some(get_schema::<TagOpsArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "notification_ops".to_string(),
                            description: "Consolidated operations for global notification endpoints (list, add, update, delete)".to_string(),
                            input_schema: Some(get_schema::<NotificationOpsArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "history_ops".to_string(),
                            description: "Consolidated operations for watch history (snapshots, diffs, screenshots, retention)".to_string(),
                            input_schema: Some(get_schema::<HistoryOpsArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "system_ops".to_string(),
                            description: "Consolidated operations for system discovery and settings (info, spec, fetchers, proxies, processors)".to_string(),
                            input_schema: Some(get_schema::<SystemOpsArgs>()),
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
