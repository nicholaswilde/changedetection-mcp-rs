mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_resources_list() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("resources/list", None).await.unwrap();

    let resources = result.get("resources").unwrap().as_array().unwrap();
    let resource_uris: Vec<&str> = resources
        .iter()
        .map(|r| r["uri"].as_str().unwrap())
        .collect();

    assert!(resource_uris.contains(&"system://openapi-spec"));
}

#[tokio::test]
async fn test_mcp_resources_read_system_spec() {
    let app = MockApp::new().await;

    let yaml_spec = "openapi: 3.0.0";
    app.mock_get_text("/api/v1/full-spec", 200, yaml_spec).await;

    let params = json!({ "uri": "system://openapi-spec" });
    let result = app
        .mcp
        .handle_method("resources/read", Some(params))
        .await
        .unwrap();

    let contents = result.get("contents").unwrap().as_array().unwrap();
    assert_eq!(contents[0]["text"], yaml_spec);
    assert_eq!(contents[0]["uri"], "system://openapi-spec");
}

#[tokio::test]
async fn test_mcp_resources_read_watch_snapshot() {
    let app = MockApp::new().await;
    let uuid = "watch-1";
    let content = "Snapshot content";

    app.mock_get_text(
        &format!("/api/v1/watch/{}/history/latest", uuid),
        200,
        content,
    )
    .await;

    let params = json!({ "uri": format!("watches://{}/latest", uuid) });
    let result = app
        .mcp
        .handle_method("resources/read", Some(params))
        .await
        .unwrap();

    let contents = result.get("contents").unwrap().as_array().unwrap();
    assert_eq!(contents[0]["text"], content);
    assert_eq!(contents[0]["uri"], format!("watches://{}/latest", uuid));
}
