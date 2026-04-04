mod common;

use common::MockApp;
use serde_json::json;

#[tokio::test]
async fn test_trigger_backup() {
    let app = MockApp::new().await;

    let response_body = json!({
        "status": "success",
        "message": "Backup initiated"
    });

    app.mock_post("/api/v1/backup", 200, Some(response_body))
        .await;

    let result = app.client.trigger_backup().await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}

#[tokio::test]
async fn test_export_watches_to_json() {
    let app = MockApp::new().await;

    let watch_uuid = "watch-1";
    let watch_details = json!({
        "url": "https://example.com",
        "title": "Example",
        "paused": false,
        "notification_urls": ["mailto:test@example.com"]
    });

    // Mock list_watches
    let watches_list = json!({
        watch_uuid: {
            "url": "https://example.com",
            "title": "Example"
        }
    });
    app.mock_get("/api/v1/watch", 200, Some(watches_list)).await;

    // Mock get_watch_details for each watch
    app.mock_get(
        &format!("/api/v1/watch/{}", watch_uuid),
        200,
        Some(watch_details.clone()),
    )
    .await;

    let export = app.client.export_watches_to_json().await.unwrap();
    assert_eq!(export.len(), 1);
    assert_eq!(
        export.get(watch_uuid).unwrap().get("url").unwrap(),
        "https://example.com"
    );
}
