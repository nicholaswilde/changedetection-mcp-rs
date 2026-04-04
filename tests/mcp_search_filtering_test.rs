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

    let params = json!({
        "action": "ListErrors"
    });

    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    let watches = result.get("watches").unwrap().as_object().unwrap();
    assert_eq!(watches.len(), 1);
    assert!(watches.contains_key("uuid1"));
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
        "action": "ListByProcessor",
        "processor": "restock_diff"
    });

    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    let watches = result.get("watches").unwrap().as_object().unwrap();
    assert_eq!(watches.len(), 1);
    assert!(watches.contains_key("uuid1"));
}

#[tokio::test]
async fn test_mcp_tools_list_filtering() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("tools/list", None).await.unwrap();
    let tools = result.get("tools").unwrap().as_array().unwrap();

    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&"watch_ops"));
}
