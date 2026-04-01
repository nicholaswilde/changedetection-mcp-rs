mod common;

use common::MockApp;
use serde_json::json;

#[tokio::test]
async fn test_get_watch_history() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let response_body = json!({
        "1234567890": "Snapshot 1",
        "1234567891": "Snapshot 2"
    });

    app.mock_get(&format!("/api/v1/watch/{}/history", uuid), 200, Some(response_body)).await;

    let history = app.client.get_watch_history(uuid).await.unwrap();
    assert_eq!(history.len(), 2);
    assert!(history.contains_key("1234567890"));
}

#[tokio::test]
async fn test_get_watch_diff() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let from = "1234567890";
    let to = "1234567891";
    let response_body = "Difference between snapshots";

    app.mock_get_text(&format!("/api/v1/watch/{}/difference/{}/{}", uuid, from, to), 200, response_body).await;

    let diff = app.client.get_watch_diff(uuid, from, to).await.unwrap();
    assert_eq!(diff, response_body);
}
