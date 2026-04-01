mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_get_watch_history() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let response_body = json!({
        "1234567890": "Snapshot 1",
        "1234567891": "Snapshot 2"
    });

    app.mock_get(&format!("/api/v1/watch/{}/history", uuid), 200, Some(response_body.clone())).await;

    let params = json!({ "uuid": uuid });
    let result = app.mcp.handle_method("get_watch_history", Some(params)).await.unwrap();
    
    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_get_watch_diff() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let from = "1234567890";
    let to = "1234567891";
    let response_body = "Difference between snapshots";

    app.mock_get_text(&format!("/api/v1/watch/{}/difference/{}/{}", uuid, from, to), 200, response_body).await;

    let params = json!({ 
        "uuid": uuid,
        "from_timestamp": from,
        "to_timestamp": to
    });
    let result = app.mcp.handle_method("get_watch_diff", Some(params)).await.unwrap();
    
    assert_eq!(result, json!(response_body));
}
