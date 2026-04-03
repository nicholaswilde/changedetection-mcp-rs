mod common;

use common::MockApp;

#[tokio::test]
async fn test_get_watch_screenshot() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let expected_bytes = vec![0, 1, 2, 3, 4, 5];

    app.mock_get_binary(
        &format!("/api/v1/watch/{}/screenshot", uuid),
        200,
        expected_bytes.clone(),
    )
    .await;

    let result = app.client.get_watch_screenshot(uuid).await.unwrap();
    assert_eq!(result, expected_bytes);
}
