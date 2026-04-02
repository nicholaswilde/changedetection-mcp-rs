mod common;

use common::MockApp;
use serde_json::json;

#[tokio::test]
async fn test_list_notifications() {
    let app = MockApp::new().await;

    let response_body = json!({
        "notification_id_1": "mailto://test@example.com",
        "notification_id_2": "tgram://bot_token/chat_id"
    });

    app.mock_get("/api/v1/notifications", 200, Some(response_body))
        .await;

    let notifications = app.client.list_notifications().await.unwrap();
    assert_eq!(notifications.len(), 2);
    assert_eq!(notifications.get("notification_id_1").unwrap(), "mailto://test@example.com");
}

#[tokio::test]
async fn test_add_notification() {
    let app = MockApp::new().await;

    let response_body = json!({
        "status": "success",
        "uuid": "notification_id_1"
    });

    app.mock_post("/api/v1/notifications", 201, Some(response_body))
        .await;

    let result = app
        .client
        .add_notification("mailto://test@example.com")
        .await
        .unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
    assert_eq!(result.get("uuid").unwrap(), "notification_id_1");
}

#[tokio::test]
async fn test_update_notifications() {
    let app = MockApp::new().await;

    let payload = json!({
        "notification_id_1": "mailto://new@example.com"
    });
    let response_body = json!({
        "status": "success"
    });

    app.mock_put("/api/v1/notifications", 200, Some(response_body))
        .await;

    let result = app.client.update_notifications(payload).await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}

#[tokio::test]
async fn test_delete_notification() {
    let app = MockApp::new().await;

    let uuid = "notification_id_1";
    let response_body = json!({
        "status": "success"
    });

    app.mock_delete(&format!("/api/v1/notifications/{}", uuid), 200, Some(response_body))
        .await;

    let result = app.client.delete_notification(uuid).await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}
