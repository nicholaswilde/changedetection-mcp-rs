mod common;
use common::MockApp;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};
use serde_json::json;
use mcp_sdk_rs::server::ServerHandler;

#[tokio::test]
async fn test_mcp_list_all_history() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    
    // 1. Mock list_watches
    Mock::given(method("GET"))
        .and(path("/api/v1/watch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            uuid: {"url": "https://example.com"}
        })))
        .mount(&app.server)
        .await;

    // 2. Mock get_watch_history
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}/history", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "1234567890": "/path/to/snap.txt"
        })))
        .mount(&app.server)
        .await;

    let result = app.mcp.handle_method("list_all_history", None).await.unwrap();
    assert!(result.is_object());
    assert!(result.as_object().unwrap().contains_key(uuid));
}

#[tokio::test]
async fn test_mcp_set_history_limit() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    
    Mock::given(method("PUT"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "uuid": uuid,
        "limit": 50
    });
    let result = app.mcp.handle_method("set_history_limit", Some(params)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_mcp_get_snapshot_info() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let timestamp = "1234567890";
    
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}/history/{}", uuid, timestamp)))
        .respond_with(ResponseTemplate::new(200)
            .append_header("content-type", "text/plain")
            .set_body_string("some content"))
        .mount(&app.server)
        .await;

    let params = json!({
        "uuid": uuid,
        "timestamp": timestamp
    });
    let result = app.mcp.handle_method("get_snapshot_info", Some(params)).await.unwrap();
    assert_eq!(result["uuid"], uuid);
    assert_eq!(result["timestamp"], timestamp);
    assert_eq!(result["content_type"], "text/plain");
}
