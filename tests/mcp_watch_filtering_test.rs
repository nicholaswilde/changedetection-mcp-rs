mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_list_watches_filter_paused() {
    let app = MockApp::new().await;

    // Mock list_watches to return two watches
    let list_response = json!({
        "uuid-1": {
            "url": "https://example.com/1",
            "title": "Watch 1"
        },
        "uuid-2": {
            "url": "https://example.com/2",
            "title": "Watch 2"
        }
    });
    app.mock_get("/api/v1/watch", 200, Some(list_response))
        .await;

    // Mock get_watch_details for each
    let details_1 = json!({
        "url": "https://example.com/1",
        "title": "Watch 1",
        "paused": true
    });
    let details_2 = json!({
        "url": "https://example.com/2",
        "title": "Watch 2",
        "paused": false
    });
    app.mock_get("/api/v1/watch/uuid-1", 200, Some(details_1))
        .await;
    app.mock_get("/api/v1/watch/uuid-2", 200, Some(details_2))
        .await;

    let params = json!({
        "action": "List",
        "state": "paused"
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    let result_obj = result.get("watches").unwrap().as_object().unwrap();
    assert_eq!(result_obj.len(), 1);
    assert!(result_obj.contains_key("uuid-1"));
    assert!(!result_obj.contains_key("uuid-2"));
}

#[tokio::test]
async fn test_mcp_list_watches_filter_unpaused() {
    let app = MockApp::new().await;

    let list_response = json!({
        "uuid-1": { "url": "https://example.com/1", "title": "Watch 1" },
        "uuid-2": { "url": "https://example.com/2", "title": "Watch 2" }
    });
    app.mock_get("/api/v1/watch", 200, Some(list_response))
        .await;

    app.mock_get(
        "/api/v1/watch/uuid-1",
        200,
        Some(json!({"url": "https://example.com/1", "paused": true})),
    )
    .await;
    app.mock_get(
        "/api/v1/watch/uuid-2",
        200,
        Some(json!({"url": "https://example.com/2", "paused": false})),
    )
    .await;

    let params = json!({
        "action": "List",
        "state": "unpaused"
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    let result_obj = result.get("watches").unwrap().as_object().unwrap();
    assert_eq!(result_obj.len(), 1);
    assert!(result_obj.contains_key("uuid-2"));
}

#[tokio::test]
async fn test_mcp_list_watches_filter_error() {
    let app = MockApp::new().await;

    let list_response = json!({
        "uuid-1": { "url": "https://example.com/1", "title": "Watch 1" },
        "uuid-2": { "url": "https://example.com/2", "title": "Watch 2" }
    });
    app.mock_get("/api/v1/watch", 200, Some(list_response))
        .await;

    app.mock_get(
        "/api/v1/watch/uuid-1",
        200,
        Some(json!({"url": "https://example.com/1", "last_error": "Some error"})),
    )
    .await;
    app.mock_get(
        "/api/v1/watch/uuid-2",
        200,
        Some(json!({"url": "https://example.com/2", "last_error": false})),
    )
    .await;

    let params = json!({
        "action": "List",
        "state": "error"
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    let result_obj = result.get("watches").unwrap().as_object().unwrap();
    assert_eq!(result_obj.len(), 1);
    assert!(result_obj.contains_key("uuid-1"));
}
