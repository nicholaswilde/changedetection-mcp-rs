mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_watch_ops_list() {
    let app = MockApp::new().await;

    let response_body = json!({
        "watch_id_1": {
            "url": "https://example.com",
            "title": "Example"
        }
    });

    app.mock_get("/api/v1/watch", 200, Some(response_body))
        .await;

    let params = json!({
        "action": "List"
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    let watches = result.get("watches").unwrap().as_object().unwrap();
    assert!(watches.get("watch_id_1").is_some());
    assert_eq!(result.get("total").unwrap(), 1);
}

#[tokio::test]
async fn test_mcp_tag_ops_list() {
    let app = MockApp::new().await;

    let response_body = json!({
        "tag_id_1": {
            "uuid": "tag_id_1",
            "title": "Tag 1"
        }
    });

    app.mock_get("/api/v1/tags", 200, Some(response_body)).await;

    let params = json!({
        "action": "List"
    });
    let result = app
        .mcp
        .handle_method("tag_ops", Some(params))
        .await
        .unwrap();

    let tags = result.get("tags").unwrap().as_object().unwrap();
    assert_eq!(tags.get("tag_id_1").unwrap()["title"], "Tag 1");
    assert_eq!(result.get("total").unwrap(), 1);
}

#[tokio::test]
async fn test_mcp_notification_ops_list() {
    let app = MockApp::new().await;

    let response_body = json!({
        "notification_urls": ["mailto://test@example.com"]
    });

    app.mock_get("/api/v1/notifications", 200, Some(response_body))
        .await;

    let params = json!({
        "action": "List"
    });
    let result = app
        .mcp
        .handle_method("notification_ops", Some(params))
        .await
        .unwrap();
    println!("Notification ops result: {:?}", result);

    let notifications = result.get("notifications").unwrap().as_array().unwrap();
    assert_eq!(notifications[0], "mailto://test@example.com");
    assert_eq!(result.get("total").unwrap(), 1);
}

#[tokio::test]
async fn test_mcp_history_ops_list_all() {
    let app = MockApp::new().await;

    // list_all_history calls list_watches then get_watch_history for each watch
    app.mock_get(
        "/api/v1/watch",
        200,
        Some(json!({"uuid1": {"url": "https://example.com"}})),
    )
    .await;
    app.mock_get(
        "/api/v1/watch/uuid1/history",
        200,
        Some(json!({"12345": "https://example.com"})),
    )
    .await;

    let params = json!({
        "action": "ListAll"
    });
    let result = app
        .mcp
        .handle_method("history_ops", Some(params))
        .await
        .unwrap();

    let history = result.get("history").unwrap().as_array().unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0]["watch_uuid"], "uuid1");
    assert_eq!(result.get("total").unwrap(), 1);
}

#[tokio::test]
async fn test_mcp_system_ops_get_info() {
    let app = MockApp::new().await;

    let response_body = json!({
        "watch_count": 10,
        "queue_size": 0,
        "overdue_watches": [],
        "uptime": 3600.0,
        "version": "0.45.2"
    });

    app.mock_get("/api/v1/systeminfo", 200, Some(response_body.clone()))
        .await;

    let params = json!({
        "action": "GetInfo"
    });
    let result = app
        .mcp
        .handle_method("system_ops", Some(params))
        .await
        .unwrap();

    assert_eq!(result["version"], "0.45.2");
}

#[tokio::test]
async fn test_mcp_tools_list_consolidated() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("tools/list", None).await.unwrap();

    let tools = result.get("tools").unwrap().as_array().unwrap();
    // We expect 5 consolidated tools (plus the 39 existing ones until we remove them)
    // Actually, the plan says we will replace them.
    // For now let's just check if the new ones are there.

    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&"watch_ops"));
    assert!(tool_names.contains(&"tag_ops"));
    assert!(tool_names.contains(&"notification_ops"));
    assert!(tool_names.contains(&"history_ops"));
    assert!(tool_names.contains(&"system_ops"));
}
