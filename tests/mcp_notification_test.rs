mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_notification_ops_list() {
    let app = MockApp::new().await;

    let response_body = json!({
        "notification_urls": ["mailto://test@example.com"]
    });

    app.mock_get("/api/v1/notifications", 200, Some(response_body.clone()))
        .await;

    let params = json!({ "action": "List" });
    let result = app
        .mcp
        .handle_method("notification_ops", Some(params))
        .await
        .unwrap();

    let notifications = result.get("notifications").unwrap().as_array().unwrap();
    assert_eq!(notifications[0], "mailto://test@example.com");
    assert_eq!(result.get("total").unwrap(), 1);
}

#[tokio::test]
async fn test_mcp_notification_ops_add() {
    let app = MockApp::new().await;

    let response_body = json!({
        "status": "success",
        "notification_urls": ["mailto://test@example.com"]
    });

    app.mock_post("/api/v1/notifications", 201, Some(response_body.clone()))
        .await;

    let params = json!({ "action": "Add", "notification_url": "mailto://test@example.com" });
    let result = app
        .mcp
        .handle_method("notification_ops", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_notification_ops_update() {
    let app = MockApp::new().await;

    let params = json!({
        "action": "Update",
        "notification_urls": ["mailto://new@example.com"]
    });
    let response_body = json!({
        "status": "success"
    });

    app.mock_put("/api/v1/notifications", 200, Some(response_body.clone()))
        .await;

    let result = app
        .mcp
        .handle_method("notification_ops", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_notification_ops_delete() {
    let app = MockApp::new().await;

    let url = "mailto://test@example.com";
    let response_body = json!({
        "status": "success"
    });

    app.mock_delete("/api/v1/notifications", 200, Some(response_body.clone()))
        .await;

    let params = json!({ "action": "Delete", "notification_url": url });
    let result = app
        .mcp
        .handle_method("notification_ops", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}
