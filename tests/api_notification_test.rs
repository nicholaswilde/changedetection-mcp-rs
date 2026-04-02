mod common;

use common::MockApp;
use serde_json::json;

#[tokio::test]
async fn test_list_notifications() {
    let app = MockApp::new().await;

    let response_body = json!({
        "notification_urls": [
            "mailto://test@example.com",
            "tgram://bot_token/chat_id"
        ]
    });

    app.mock_get("/api/v1/notifications", 200, Some(response_body))
        .await;

    let notifications = app.client.list_notifications().await.unwrap();
    assert_eq!(notifications.len(), 2);
    assert_eq!(notifications[0], "mailto://test@example.com");
}

#[tokio::test]
async fn test_add_notification() {
    let app = MockApp::new().await;

    let response_body = json!({
        "status": "success",
        "notification_urls": ["mailto://test@example.com"]
    });

    app.mock_post("/api/v1/notifications", 201, Some(response_body))
        .await;

    let result = app
        .client
        .add_notification("mailto://test@example.com")
        .await
        .unwrap();
    assert_eq!(result["status"], "success");
}

#[tokio::test]
async fn test_update_notifications() {
    let app = MockApp::new().await;

    let notification_urls = vec!["mailto://new@example.com".to_string()];
    let response_body = json!({
        "status": "success"
    });

    app.mock_put("/api/v1/notifications", 200, Some(response_body))
        .await;

    let result = app.client.update_notifications(notification_urls).await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}

#[tokio::test]
async fn test_delete_notification() {
    let app = MockApp::new().await;

    let url = "mailto://test@example.com";
    let response_body = json!({
        "status": "success"
    });

    app.mock_delete("/api/v1/notifications", 200, Some(response_body))
        .await;

    let result = app.client.delete_notification(url).await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}
