mod common;

use common::MockApp;
use serde_json::json;

#[tokio::test]
async fn test_pause_watch() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let response_body = json!({"status": "success"});

    app.mock_get_with_query(
        &format!("/api/v1/watch/{}", uuid),
        "paused",
        "paused",
        200,
        Some(response_body),
    )
    .await;

    let result = app
        .client
        .set_watch_state(uuid, "paused", "paused")
        .await
        .unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}

#[tokio::test]
async fn test_unpause_watch() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let response_body = json!({"status": "success"});

    app.mock_get_with_query(
        &format!("/api/v1/watch/{}", uuid),
        "paused",
        "unpaused",
        200,
        Some(response_body),
    )
    .await;

    let result = app
        .client
        .set_watch_state(uuid, "paused", "unpaused")
        .await
        .unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}

#[tokio::test]
async fn test_mute_notifications() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let response_body = json!({"status": "success"});

    app.mock_get_with_query(
        &format!("/api/v1/watch/{}", uuid),
        "muted",
        "muted",
        200,
        Some(response_body),
    )
    .await;

    let result = app
        .client
        .set_watch_state(uuid, "muted", "muted")
        .await
        .unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}

#[tokio::test]
async fn test_unmute_notifications() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let response_body = json!({"status": "success"});

    app.mock_get_with_query(
        &format!("/api/v1/watch/{}", uuid),
        "muted",
        "unmuted",
        200,
        Some(response_body),
    )
    .await;

    let result = app
        .client
        .set_watch_state(uuid, "muted", "unmuted")
        .await
        .unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}
