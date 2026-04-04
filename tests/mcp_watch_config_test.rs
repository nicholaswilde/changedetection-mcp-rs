mod common;
use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

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
        "action": "SetSelectors",
        "uuid": uuid,
        "css_filter": ".price"
    });
    let result = app.mcp.handle_method("watch_ops", Some(params)).await;

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
        "action": "SetFetcher",
        "uuid": uuid,
        "fetcher": "playwright"
    });
    let result = app.mcp.handle_method("watch_ops", Some(params)).await;

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
        "action": "ConfigureNotifications",
        "uuid": uuid,
        "notification_urls": ["tgram://bot_token/chat_id"]
    });
    let result = app.mcp.handle_method("watch_ops", Some(params)).await;

    assert!(result.is_ok());
}
