mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_import_watches() {
    let app = MockApp::new().await;
    let response_body = json!(["uuid-1", "uuid-2"]);

    app.mock_post_with_query(
        "/api/v1/import",
        "tag",
        "imported",
        200,
        Some(response_body.clone()),
    )
    .await;

    let params = json!({
        "urls": ["https://example.com/1", "https://example.com/2"],
        "tag": "imported"
    });
    let result = app
        .mcp
        .handle_method("import_watches", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_tools_list_import() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("tools/list", None).await.unwrap();

    let tools = result.get("tools").unwrap().as_array().unwrap();
    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();

    assert!(tool_names.contains(&"import_watches"));
}
