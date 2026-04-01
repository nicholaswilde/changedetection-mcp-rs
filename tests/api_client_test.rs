mod common;

use common::MockApp;
use serde_json::json;

#[tokio::test]
async fn test_list_watches() {
    let app = MockApp::new().await;

    let response_body = json!({
        "watch_id_1": {
            "url": "https://example.com",
            "title": "Example"
        }
    });

    app.mock_get("/api/v1/watch", 200, Some(response_body)).await;

    let watches = app.client.list_watches(None).await.unwrap();
    assert_eq!(watches.len(), 1);
}

#[tokio::test]
async fn test_get_watch_details() {
    let app = MockApp::new().await;

    let uuid = "watch_id_1";
    let response_body = json!({
        "url": "https://example.com",
        "title": "Example"
    });

    app.mock_get(&format!("/api/v1/watch/{}", uuid), 200, Some(response_body)).await;

    let watch = app.client.get_watch_details(uuid).await.unwrap();
    assert_eq!(watch.url, "https://example.com");
}

#[tokio::test]
async fn test_create_watch() {
    let app = MockApp::new().await;

    let response_body = json!({
        "status": "success",
        "uuid": "watch_id_1"
    });

    app.mock_post("/api/v1/watch", 201, Some(response_body)).await;

    let result = app.client
        .create_watch("https://example.com", None)
        .await
        .unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
    assert_eq!(result.get("uuid").unwrap(), "watch_id_1");
}

#[tokio::test]
async fn test_delete_watch() {
    let app = MockApp::new().await;

    let uuid = "watch_id_1";
    let response_body = json!({
        "status": "success"
    });

    app.mock_delete(&format!("/api/v1/watch/{}", uuid), 200, Some(response_body)).await;

    let result = app.client.delete_watch(uuid).await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}

#[tokio::test]
async fn test_trigger_check() {
    let app = MockApp::new().await;

    let uuid = "watch_id_1";
    let response_body = json!({
        "status": "success"
    });

    app.mock_get(&format!("/api/v1/watch/{}/recheck", uuid), 200, Some(response_body)).await;

    let result = app.client.trigger_check(uuid).await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}
