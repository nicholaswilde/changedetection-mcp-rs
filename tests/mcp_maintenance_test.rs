mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_maintenance_ops_backup() {
    let app = MockApp::new().await;

    let response_body = json!({
        "status": "success",
        "message": "Backup initiated"
    });

    app.mock_post("/api/v1/backup", 200, Some(response_body.clone()))
        .await;

    let params = json!({ "action": "Backup" });
    let result = app
        .mcp
        .handle_method("maintenance_ops", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_maintenance_ops_export() {
    let app = MockApp::new().await;

    let watch_uuid = "watch-1";
    let watch_details = json!({
        "url": "https://example.com",
        "title": "Example",
        "paused": false
    });

    // Mock list_watches
    let watches_list = json!({
        watch_uuid: {
            "url": "https://example.com",
            "title": "Example"
        }
    });
    app.mock_get("/api/v1/watch", 200, Some(watches_list)).await;

    // Mock get_watch_details for each watch
    app.mock_get(&format!("/api/v1/watch/{}", watch_uuid), 200, Some(watch_details.clone()))
        .await;

    let params = json!({ "action": "Export" });
    let result = app
        .mcp
        .handle_method("maintenance_ops", Some(params))
        .await
        .unwrap();

    let export = result.get("watches").unwrap().as_object().unwrap();
    assert_eq!(export.len(), 1);
    assert_eq!(export.get(watch_uuid).unwrap().get("url").unwrap(), "https://example.com");
}

#[tokio::test]
async fn test_mcp_maintenance_ops_list_in_tools() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("tools/list", None).await.unwrap();
    let tools = result.get("tools").unwrap().as_array().unwrap();

    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&"maintenance_ops"));
}
