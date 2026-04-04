use crate::api::{BrowserStep, Client, Condition, Watch};
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
    /// List all current watches. Can be filtered by tag or state.
    List,
    /// Search for watches by URL or title using a query string.
    Search,
    /// Retrieve detailed information for a specific watch by its UUID.
    Get,
    /// Add a new URL as a watch. Optional tag and title can be provided.
    Create,
    /// Modify the configuration of an existing watch by its UUID.
    Update,
    /// Permanently remove a watch by its UUID.
    Delete,
    /// Manually trigger a check for changes on a specific watch.
    Trigger,
    /// Temporarily stop monitoring a specific watch.
    Pause,
    /// Resume monitoring a previously paused watch.
    Unpause,
    /// Disable notifications for a specific watch while still monitoring for changes.
    Mute,
    /// Re-enable notifications for a specific watch.
    Unmute,
    /// Add multiple URLs as new watches in a single operation.
    Import,
    /// Set CSS, XPath, or JSONPath selectors for a specific watch to target specific content.
    SetSelectors,
    /// Configure the fetching engine (e.g., 'playwright', 'basic') for a specific watch.
    SetFetcher,
    /// Configure per-watch notification endpoints and custom alert messages.
    ConfigureNotifications,
    /// List all watches that are currently in an error state.
    ListErrors,
    /// List watches that use a specific change detection processor (e.g., 'restock_diff').
    ListByProcessor,
    /// Configure browser automation steps (e.g., click, wait, input) for a specific watch.
    SetBrowserSteps,
    /// Configure change detection conditions (e.g., price thresholds, regex matches).
    SetConditions,
    /// Configure custom HTTP headers and POST body for a specific watch.
    SetRequestConfig,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct WatchOpsArgs {
    /// The specific operation to perform on watches.
    pub action: WatchAction,
    /// The unique identifier (UUID) of the watch. Required for most actions except List, Search, and Import.
    pub uuid: Option<String>,
    /// The full URL of the website to monitor. Required for Create and Update.
    pub url: Option<String>,
    /// A tag to apply to the watch or to use as a filter for List operations.
    pub tag: Option<String>,
    /// A custom, human-readable name for the watch.
    pub title: Option<String>,
    /// The search string to find watches by URL or title. Required for Search.
    pub query: Option<String>,
    /// Filter watches by their current state (e.g., 'paused', 'unpaused', 'error').
    pub state: Option<String>,
    /// The name of the change detection processor to filter by (e.g., 'text_json_diff'). Required for ListByProcessor.
    pub processor: Option<String>,
    /// A list of URLs to be added as new watches. Required for Import.
    pub urls: Option<Vec<String>>,
    /// CSS selector to filter the content of the page (e.g., '#content', 'article.main').
    pub css_filter: Option<String>,
    /// XPath expression to target specific elements on the page.
    pub xpath_filter: Option<String>,
    /// JSONPath expression for monitoring changes in JSON data.
    pub json_filter: Option<String>,
    /// The fetcher engine to use (e.g., 'html_webdriver', 'basic_http').
    pub fetcher: Option<String>,
    /// A list of Apprise-compatible notification service URLs (e.g., 'tgram://...', 'mailto://...').
    pub notification_urls: Option<Vec<String>>,
    /// Custom title for notifications sent from this watch.
    pub notification_title: Option<String>,
    /// Custom body text for notifications sent from this watch.
    pub notification_body: Option<String>,
    /// A list of browser automation steps to execute during fetching. Required for SetBrowserSteps.
    pub browser_steps: Option<Vec<BrowserStep>>,
    /// A list of condition rules for change detection logic. Required for SetConditions.
    pub conditions: Option<Vec<Condition>>,
    /// Logic operator for conditions: 'ALL' (match all) or 'ANY' (match any).
    pub conditions_match_logic: Option<String>,
    /// HTTP request body for the watch.
    pub body: Option<String>,
    /// HTTP headers to include in the request.
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(flatten)]
    pub common: CommonArgs,
}

#[derive(JsonSchema, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum TagAction {
    /// List all tags across all watches. Supports pagination and field selection.
    List,
    /// Create a new tag that can be used to categorize watches.
    Create,
    /// Retrieve detailed information for a specific tag.
    Get,
    /// Update the title or other properties of an existing tag.
    Update,
    /// Permanently remove a tag. This does not delete associated watches.
    Delete,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct TagOpsArgs {
    /// The specific operation to perform on tags.
    pub action: TagAction,
    /// The unique identifier (UUID) of the tag. Required for Get, Update, and Delete.
    pub uuid: Option<String>,
    /// The title of the tag. Required for Create and optionally for Update.
    pub title: Option<String>,
    #[serde(flatten)]
    pub common: CommonArgs,
}

#[derive(JsonSchema, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum NotificationAction {
    /// List all global notification endpoints.
    List,
    /// Add a new Apprise-compatible URL to the global notification list.
    Add,
    /// Overwrite the entire list of global notification endpoints.
    Update,
    /// Remove a specific Apprise-compatible URL from the global list.
    Delete,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct NotificationOpsArgs {
    /// The specific operation to perform on global notification settings.
    pub action: NotificationAction,
    /// A single Apprise-compatible notification URL (e.g., 'pntry://...'). Required for Add and Delete.
    pub notification_url: Option<String>,
    /// A list of Apprise-compatible URLs. Required for Update.
    pub notification_urls: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonArgs,
}

#[derive(JsonSchema, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum HistoryAction {
    /// Retrieve a list of all snapshot timestamps for a specific watch.
    GetHistory,
    /// Compare two snapshots and return the differences.
    GetDiff,
    /// Retrieve the full content of a specific snapshot by its timestamp.
    GetContent,
    /// Capture or retrieve a visual screenshot of the current watch state.
    GetScreenshot,
    /// List all snapshots across all watches, optionally filtered by tag.
    ListAll,
    /// Set a limit on the number of snapshots to retain for a specific watch.
    SetLimit,
    /// Retrieve technical metadata for a specific snapshot (e.g., content-length).
    GetInfo,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct HistoryOpsArgs {
    /// The specific operation to perform on watch history.
    pub action: HistoryAction,
    /// The UUID of the watch. Required for most actions except ListAll.
    pub uuid: Option<String>,
    /// The timestamp of the snapshot to retrieve or inspect.
    pub timestamp: Option<String>,
    /// The timestamp of the first snapshot to compare. Required for GetDiff.
    pub from_timestamp: Option<String>,
    /// The timestamp of the second snapshot to compare. Required for GetDiff.
    pub to_timestamp: Option<String>,
    /// The output format for the diff (e.g., 'text', 'markdown', 'html').
    pub format: Option<String>,
    /// The maximum number of snapshots to keep. Required for SetLimit.
    pub limit: Option<i32>,
    /// A tag to filter history across all watches. Used with ListAll.
    pub tag: Option<String>,
    #[serde(flatten)]
    pub common: CommonArgs,
}

#[derive(JsonSchema, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum SystemAction {
    /// Retrieve ChangeDetection.io server version and instance statistics.
    GetInfo,
    /// Retrieve the full OpenAPI specification for the ChangeDetection.io API.
    GetSpec,
    /// List all available fetching engines (e.g., 'playwright', 'basic_http').
    ListFetchers,
    /// List all configured system-level proxies.
    ListProxies,
    /// Retrieve the full set of global instance-level settings.
    GetSettings,
    /// List all available change detection processors (e.g., 'restock_diff').
    ListProcessors,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct SystemOpsArgs {
    /// The specific operation to perform for system discovery and settings.
    pub action: SystemAction,
    #[serde(flatten)]
    pub common: CommonArgs,
}

#[derive(JsonSchema, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum MaintenanceAction {
    /// Manually trigger a system-wide backup of all watch configurations and data.
    Backup,
    /// Export the full list of watches and their configurations as a single JSON file.
    Export,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct MaintenanceOpsArgs {
    /// The specific maintenance task to perform.
    pub action: MaintenanceAction,
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
            resources: Some(serde_json::json!({
                "subscribe": false,
                "listChanged": false,
            })),
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
                "resources/list" => {
                    let resources = vec![
                        serde_json::json!({
                            "uri": "system://openapi-spec",
                            "name": "OpenAPI Specification",
                            "mimeType": "application/x-yaml",
                            "description": "The full OpenAPI specification for the ChangeDetection.io API"
                        })
                    ];
                    Ok(serde_json::json!({ "resources": resources }))
                }
                "resources/read" => {
                    let uri = params.as_ref()
                        .and_then(|p| p.get("uri"))
                        .and_then(|u| u.as_str())
                        .ok_or_else(|| Error::protocol(ErrorCode::InvalidParams, "Missing uri"))?;

                    if uri == "system://openapi-spec" {
                        let spec = self.client.get_full_spec().await.map_err(|e| {
                            Error::protocol(ErrorCode::InternalError, e.to_string())
                        })?;
                        Ok(serde_json::json!({
                            "contents": [{
                                "uri": uri,
                                "mimeType": "application/x-yaml",
                                "text": spec
                            }]
                        }))
                    } else if uri.starts_with("watches://") && uri.ends_with("/latest") {
                        let uuid = uri.strip_prefix("watches://")
                            .and_then(|s| s.strip_suffix("/latest"))
                            .ok_or_else(|| Error::protocol(ErrorCode::InvalidParams, "Invalid watches URI format"))?;

                        let content = self.client.get_snapshot_content(uuid, "latest").await.map_err(|e| {
                            Error::protocol(ErrorCode::InternalError, e.to_string())
                        })?;

                        Ok(serde_json::json!({
                            "contents": [{
                                "uri": uri,
                                "mimeType": "text/plain",
                                "text": content
                            }]
                        }))
                    } else {
                        Err(Error::protocol(ErrorCode::InvalidParams, format!("Unknown resource URI: {}", uri)))
                    }
                }
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
                            let mut payload = params.unwrap_or(serde_json::json!({}));
                            if let Some(obj) = payload.as_object_mut() {
                                obj.remove("action");
                            }
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
                            let count = result.len();
                            Ok(serde_json::json!({
                                "watches": result,
                                "total": count
                            }))
                        }
                        WatchAction::ListByProcessor => {
                            let processor = args.processor.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing processor")
                            })?;
                            let result = self.client.list_watches_by_processor(&processor).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            let count = result.len();
                            Ok(serde_json::json!({
                                "watches": result,
                                "total": count
                            }))
                        }
                        WatchAction::SetBrowserSteps => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let steps = args.browser_steps.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing browser_steps")
                            })?;
                            let result = self.client.set_browser_steps(&uuid, steps).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::SetConditions => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let conditions = args.conditions.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing conditions")
                            })?;
                            let result = self.client.set_conditions(
                                &uuid,
                                conditions,
                                args.conditions_match_logic.as_deref(),
                            ).await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::to_value(result)?)
                        }
                        WatchAction::SetRequestConfig => {
                            let uuid = args.uuid.ok_or_else(|| {
                                Error::protocol(ErrorCode::InvalidParams, "Missing uuid")
                            })?;
                            let result = self.client.set_request_config(
                                &uuid,
                                args.headers,
                                args.body.as_deref(),
                            ).await.map_err(|e| {
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
                            let mut payload = params.as_ref().cloned().unwrap_or(serde_json::json!({}));
                            if let Some(obj) = payload.as_object_mut() {
                                obj.remove("action");
                            }
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
                            let count = fetchers.len();
                            Ok(serde_json::json!({
                                "fetchers": fetchers,
                                "total": count
                            }))
                        }
                        SystemAction::ListProxies => {
                            let proxies = self.client.list_proxies().await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            let count = proxies.len();
                            Ok(serde_json::json!({
                                "proxies": proxies,
                                "total": count
                            }))
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
                "maintenance_ops" => {
                    let args: MaintenanceOpsArgs = serde_json::from_value(params.as_ref().cloned().unwrap_or(serde_json::json!({})))?;
                    match args.action {
                        MaintenanceAction::Backup => {
                            let result = self.client.trigger_backup().await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(result)
                        }
                        MaintenanceAction::Export => {
                            let result = self.client.export_watches_to_json().await.map_err(|e| {
                                Error::protocol(ErrorCode::InternalError, e.to_string())
                            })?;
                            Ok(serde_json::json!({
                                "watches": result,
                                "total": result.len()
                            }))
                        }
                    }
                }
                "tools/list" => {
                    let tools = vec![
                        Tool {
                            name: "watch_ops".to_string(),
                            description: "Comprehensive operations for managing and interacting with individual or multiple watches. Actions include listing, searching, detailed retrieval, creation, updates, deletion, manual triggering, pausing/unpausing, muting/unmuting notifications, and configuring advanced monitoring settings (selectors, fetchers, notifications, browser steps, conditions, custom request config).".to_string(),
                            input_schema: Some(get_schema::<WatchOpsArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "tag_ops".to_string(),
                            description: "Operations for managing watch categories (tags). Actions include listing, creating, retrieving, updating, and deleting tags used for watch organization.".to_string(),
                            input_schema: Some(get_schema::<TagOpsArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "notification_ops".to_string(),
                            description: "Operations for managing global, system-wide notification endpoints. Actions include listing, adding, updating, and deleting Apprise-compatible alert URLs.".to_string(),
                            input_schema: Some(get_schema::<NotificationOpsArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "history_ops".to_string(),
                            description: "Operations for managing and analyzing the historical data of watches. Actions include retrieving snapshot history, comparing snapshots (diffs), fetching specific snapshot content, capturing screenshots, and managing data retention limits.".to_string(),
                            input_schema: Some(get_schema::<HistoryOpsArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "system_ops".to_string(),
                            description: "Operations for discovering server-level configurations and capabilities. Actions include retrieving system info, API specifications, available fetching engines, configured proxies, global settings, and change detection processors.".to_string(),
                            input_schema: Some(get_schema::<SystemOpsArgs>()),
                            annotations: None,
                        },
                        Tool {
                            name: "maintenance_ops".to_string(),
                            description: "Operations for critical system-wide maintenance tasks. Actions include triggering full system backups and performing complete watch configuration exports.".to_string(),
                            input_schema: Some(get_schema::<MaintenanceOpsArgs>()),
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
