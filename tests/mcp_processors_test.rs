mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_list_processors() {
    let app = MockApp::new().await;
    let mock_spec = r#"
components:
  schemas:
    Watch:
      properties:
        processor:
          enum:
          - restock_diff
          - text_json_diff
"#;

    app.mock_get_text("/api/v1/full-spec", 200, mock_spec).await;

    let result = app.mcp.handle_method("list_processors", None).await.unwrap();

    assert_eq!(result, json!(["restock_diff", "text_json_diff"]));
}

#[tokio::test]
async fn test_mcp_tools_list_processors() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("tools/list", None).await.unwrap();

    let tools = result.get("tools").unwrap().as_array().unwrap();
    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    
    assert!(tool_names.contains(&"list_processors"));
}
