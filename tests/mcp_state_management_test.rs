mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_pause_watch() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let response_body = json!({"status": "success"});

    app.mock_get_with_query(
        &format!("/api/v1/watch/{}", uuid),
        "paused",
        "paused",
        200,
        Some(response_body.clone()),
    )
    .await;

    let params = json!({
        "action": "Pause",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_unpause_watch() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let response_body = json!({"status": "success"});

    app.mock_get_with_query(
        &format!("/api/v1/watch/{}", uuid),
        "paused",
        "unpaused",
        200,
        Some(response_body.clone()),
    )
    .await;

    let params = json!({
        "action": "Unpause",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_mute_notifications() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let response_body = json!({"status": "success"});

    app.mock_get_with_query(
        &format!("/api/v1/watch/{}", uuid),
        "muted",
        "muted",
        200,
        Some(response_body.clone()),
    )
    .await;

    let params = json!({
        "action": "Mute",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_unmute_notifications() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let response_body = json!({"status": "success"});

    app.mock_get_with_query(
        &format!("/api/v1/watch/{}", uuid),
        "muted",
        "unmuted",
        200,
        Some(response_body.clone()),
    )
    .await;

    let params = json!({
        "action": "Unmute",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_tools_list_state_management() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("tools/list", None).await.unwrap();

    let tools = result.get("tools").unwrap().as_array().unwrap();
    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();

    assert!(tool_names.contains(&"watch_ops"));
}
