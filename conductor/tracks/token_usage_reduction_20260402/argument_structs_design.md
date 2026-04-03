# Consolidated Argument Structs Design

This document defines the new consolidated MCP tool argument structures.

## Common Components

```rust
#[derive(JsonSchema, Deserialize, Debug)]
pub struct PaginationArgs {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct CommonArgs {
    pub pagination: Option<PaginationArgs>,
    pub fields: Option<Vec<String>>,
}
```

## 1. `watch_ops`

```rust
#[derive(JsonSchema, Deserialize, Debug)]
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
    pub action: WatchAction,
    pub uuid: Option<String>,
    pub url: Option<String>,
    pub tag: Option<String>,
    pub title: Option<String>,
    pub query: Option<String>, // For Search
    pub state: Option<String>, // For List filtering
    pub processor: Option<String>, // For ListByProcessor
    pub urls: Option<Vec<String>>, // For Import
    pub css_filter: Option<String>,
    pub xpath_filter: Option<String>,
    pub json_filter: Option<String>,
    pub fetcher: Option<String>,
    pub notification_urls: Option<Vec<String>>,
    pub notification_title: Option<String>,
    pub notification_body: Option<String>,
    #[serde(flatten)]
    pub common: CommonArgs,
}
```

## 2. `tag_ops`

```rust
#[derive(JsonSchema, Deserialize, Debug)]
pub enum TagAction {
    List,
    Create,
    Get,
    Update,
    Delete,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct TagOpsArgs {
    pub action: TagAction,
    pub uuid: Option<String>,
    pub title: Option<String>,
    #[serde(flatten)]
    pub common: CommonArgs,
}
```

## 3. `notification_ops`

```rust
#[derive(JsonSchema, Deserialize, Debug)]
pub enum NotificationAction {
    List,
    Add,
    Update,
    Delete,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct NotificationOpsArgs {
    pub action: NotificationAction,
    pub notification_url: Option<String>,
    pub notification_urls: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonArgs,
}
```

## 4. `history_ops`

```rust
#[derive(JsonSchema, Deserialize, Debug)]
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
    pub action: HistoryAction,
    pub uuid: Option<String>,
    pub timestamp: Option<String>,
    pub from_timestamp: Option<String>,
    pub to_timestamp: Option<String>,
    pub format: Option<String>,
    pub limit: Option<i32>,
    pub tag: Option<String>,
    #[serde(flatten)]
    pub common: CommonArgs,
}
```

## 5. `system_ops`

```rust
#[derive(JsonSchema, Deserialize, Debug)]
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
    pub action: SystemAction,
    #[serde(flatten)]
    pub common: CommonArgs,
}
```
