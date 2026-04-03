mod common;

use base64::{engine::general_purpose, Engine as _};
use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_get_watch_screenshot() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let binary_data = vec![0, 1, 2, 3, 4, 5];
    let expected_base64 = general_purpose::STANDARD.encode(&binary_data);

    app.mock_get_binary(
        &format!("/api/v1/watch/{}/screenshot", uuid),
        200,
        binary_data,
    )
    .await;

    let params = json!({ "uuid": uuid });
    let result = app
        .mcp
        .handle_method("get_watch_screenshot", Some(params))
        .await
        .unwrap();

    assert_eq!(result, json!(expected_base64));
}

#[tokio::test]
async fn test_mcp_tools_list_screenshot() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("tools/list", None).await.unwrap();

    let tools = result.get("tools").unwrap().as_array().unwrap();
    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();

    assert!(tool_names.contains(&"get_watch_screenshot"));
}
