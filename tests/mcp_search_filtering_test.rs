mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_find_watches_by_error() {
    let app = MockApp::new().await;

    let response_body = json!({
        "uuid1": {
            "url": "https://example.com/error",
            "title": "Error Watch",
            "paused": false,
            "last_error": "Connection Timeout"
        },
        "uuid2": {
            "url": "https://example.com/ok",
            "title": "OK Watch",
            "paused": false,
            "last_error": false
        }
    });

    app.mock_get("/api/v1/watch", 200, Some(response_body))
        .await;

    let result = app
        .mcp
        .handle_method("find_watches_by_error", None)
        .await
        .unwrap();
    let error_watches: std::collections::HashMap<String, serde_json::Value> =
        serde_json::from_value(result).unwrap();

    assert_eq!(error_watches.len(), 1);
    assert!(error_watches.contains_key("uuid1"));
}

#[tokio::test]
async fn test_mcp_list_watches_by_processor() {
    let app = MockApp::new().await;

    let response_body = json!({
        "uuid1": {
            "url": "https://example.com/restock",
            "title": "Restock Watch",
            "paused": false,
            "processor": "restock_diff"
        },
        "uuid2": {
            "url": "https://example.com/text",
            "title": "Text Watch",
            "paused": false,
            "processor": "text_json_diff"
        }
    });

    app.mock_get("/api/v1/watch", 200, Some(response_body))
        .await;

    let params = json!({
        "processor": "restock_diff"
    });

    let result = app
        .mcp
        .handle_method("list_watches_by_processor", Some(params))
        .await
        .unwrap();
    let filtered_watches: std::collections::HashMap<String, serde_json::Value> =
        serde_json::from_value(result).unwrap();

    assert_eq!(filtered_watches.len(), 1);
    assert!(filtered_watches.contains_key("uuid1"));
}

#[tokio::test]
async fn test_mcp_tools_list_filtering() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("tools/list", None).await.unwrap();
    let tools = result.get("tools").unwrap().as_array().unwrap();

    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&"find_watches_by_error"));
    assert!(tool_names.contains(&"list_watches_by_processor"));
}
