mod common;
use common::MockApp;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};
use serde_json::json;
use mcp_sdk_rs::server::ServerHandler;

#[tokio::test]
async fn test_mcp_set_watch_selectors() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    
    Mock::given(method("PUT"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "uuid": uuid,
        "css_filter": ".price"
    });
    let result = app.mcp.handle_method("set_watch_selectors", Some(params)).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_mcp_set_watch_fetcher() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    
    Mock::given(method("PUT"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "uuid": uuid,
        "fetcher": "playwright"
    });
    let result = app.mcp.handle_method("set_watch_fetcher", Some(params)).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_mcp_configure_watch_notifications() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    
    Mock::given(method("PUT"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "uuid": uuid,
        "notification_urls": ["tgram://bot_token/chat_id"]
    });
    let result = app.mcp.handle_method("configure_watch_notifications", Some(params)).await;

    assert!(result.is_ok());
}
