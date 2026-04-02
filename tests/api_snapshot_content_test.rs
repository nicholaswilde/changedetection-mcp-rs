mod common;

use common::MockApp;

#[tokio::test]
async fn test_get_snapshot_content() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let timestamp = "1234567890";
    let expected_content = "<html><body>Snapshot content</body></html>";

    app.mock_get_text(
        &format!("/api/v1/watch/{}/history/{}", uuid, timestamp),
        200,
        expected_content,
    )
    .await;

    let content = app
        .client
        .get_snapshot_content(uuid, timestamp)
        .await
        .unwrap();
    assert_eq!(content, expected_content);
}
