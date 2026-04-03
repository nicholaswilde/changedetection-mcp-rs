mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_get_snapshot_content() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let timestamp = "1234567890";
    let expected_content = "<html><body>Snapshot content</body></html>";

    app.mock_get_text(
        &format!("/api/v1/watch/{}/history/{}", uuid, timestamp),
        200,
        expected_content,
    )
    .await;

    let params = json!({
        "uuid": uuid,
        "timestamp": timestamp
    });
    let result = app
        .mcp
        .handle_method("get_snapshot_content", Some(params))
        .await
        .unwrap();

    assert_eq!(result, json!(expected_content));
}

#[tokio::test]
async fn test_mcp_tools_list_snapshot_content() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("tools/list", None).await.unwrap();

    let tools = result.get("tools").unwrap().as_array().unwrap();
    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();

    assert!(tool_names.contains(&"get_snapshot_content"));
}
