mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_list_notifications() {
    let app = MockApp::new().await;

    let response_body = json!({
        "notification_urls": ["mailto://test@example.com"]
    });

    app.mock_get("/api/v1/notifications", 200, Some(response_body.clone()))
        .await;

    let result = app.mcp.handle_method("list_notifications", None).await.unwrap();

    // The API client now returns Vec<String>, which MCP should serialize as an array
    assert_eq!(result, json!(["mailto://test@example.com"]));
}

#[tokio::test]
async fn test_mcp_add_notification() {
    let app = MockApp::new().await;

    let response_body = json!({
        "status": "success",
        "notification_urls": ["mailto://test@example.com"]
    });

    app.mock_post("/api/v1/notifications", 201, Some(response_body.clone()))
        .await;

    let params = json!({ "notification_url": "mailto://test@example.com" });
    let result = app
        .mcp
        .handle_method("add_notification", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_update_notifications() {
    let app = MockApp::new().await;

    let params = json!({
        "notification_urls": ["mailto://new@example.com"]
    });
    let response_body = json!({
        "status": "success"
    });

    app.mock_put("/api/v1/notifications", 200, Some(response_body.clone()))
        .await;

    let result = app
        .mcp
        .handle_method("update_notifications", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_delete_notification() {
    let app = MockApp::new().await;

    let url = "mailto://test@example.com";
    let response_body = json!({
        "status": "success"
    });

    app.mock_delete("/api/v1/notifications", 200, Some(response_body.clone()))
        .await;

    let params = json!({ "notification_url": url });
    let result = app
        .mcp
        .handle_method("delete_notification", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_tools_list_notifications() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("tools/list", None).await.unwrap();

    let tools = result.get("tools").unwrap().as_array().unwrap();
    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    
    assert!(tool_names.contains(&"list_notifications"));
    assert!(tool_names.contains(&"add_notification"));
    assert!(tool_names.contains(&"update_notifications"));
    assert!(tool_names.contains(&"delete_notification"));
}
