mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_pagination_optimization() {
    let app = MockApp::new().await;

    // Mock 5 watches
    let mut response_body = serde_json::Map::new();
    for i in 1..=5 {
        response_body.insert(
            format!("uuid-{}", i),
            json!({
                "url": format!("https://example.com/{}", i),
                "title": format!("Watch {}", i)
            }),
        );
    }

    app.mock_get(
        "/api/v1/watch",
        200,
        Some(serde_json::Value::Object(response_body)),
    )
    .await;

    // Request page 1, per_page 2
    let params = json!({
        "action": "List",
        "pagination": {
            "page": 1,
            "per_page": 2
        }
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    let watches = result.get("watches").unwrap().as_object().unwrap();
    assert_eq!(watches.len(), 2);
    assert_eq!(result["total"], 5);

    // Request page 3, per_page 2 (only 1 item left)
    let params = json!({
        "action": "List",
        "pagination": {
            "page": 3,
            "per_page": 2
        }
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    let watches = result.get("watches").unwrap().as_object().unwrap();
    assert_eq!(watches.len(), 1);
    assert_eq!(result["total"], 5);
}

#[tokio::test]
async fn test_mcp_field_selection_optimization() {
    let app = MockApp::new().await;

    let response_body = json!({
        "uuid-1": {
            "url": "https://example.com",
            "title": "Example",
            "paused": false,
            "last_error": ""
        }
    });

    app.mock_get("/api/v1/watch", 200, Some(response_body))
        .await;

    // Request only 'url' field
    let params = json!({
        "action": "List",
        "fields": ["url"]
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    let watches = result.get("watches").unwrap().as_object().unwrap();
    let watch = watches.get("uuid-1").unwrap().as_object().unwrap();

    assert!(watch.contains_key("url"));
    assert!(!watch.contains_key("title"));
    assert!(!watch.contains_key("paused"));
    assert!(!watch.contains_key("last_error"));
}

#[tokio::test]
async fn test_mcp_system_info_field_selection() {
    let app = MockApp::new().await;

    let response_body = json!({
        "watch_count": 10,
        "queue_size": 0,
        "overdue_watches": [],
        "uptime": 3600.0,
        "version": "0.45.2"
    });

    app.mock_get("/api/v1/systeminfo", 200, Some(response_body))
        .await;

    // Request only 'version' field
    let params = json!({
        "action": "GetInfo",
        "fields": ["version"]
    });
    let result = app
        .mcp
        .handle_method("system_ops", Some(params))
        .await
        .unwrap();

    assert!(result.as_object().unwrap().contains_key("version"));
    assert!(!result.as_object().unwrap().contains_key("uptime"));
    assert!(!result.as_object().unwrap().contains_key("watch_count"));
}
