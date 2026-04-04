mod common;
use common::MockApp;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_set_watch_selectors() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";

    Mock::given(method("PUT"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let result = app
        .client
        .set_watch_selectors(
            uuid,
            Some(".price"),
            Some("//div[@id='price']"),
            Some("$.store.book[*].author"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_set_watch_fetcher() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";

    Mock::given(method("PUT"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let result = app.client.set_watch_fetcher(uuid, "playwright").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_configure_watch_notifications() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";

    Mock::given(method("PUT"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let result = app
        .client
        .configure_watch_notifications(
            uuid,
            vec!["tgram://bot_token/chat_id".to_string()],
            Some("Price Alert"),
            Some("Price changed!"),
        )
        .await;

    assert!(result.is_ok());
}
