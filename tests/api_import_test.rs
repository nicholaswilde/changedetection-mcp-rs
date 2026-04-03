mod common;

use common::MockApp;
use serde_json::json;

#[tokio::test]
async fn test_import_watches() {
    let app = MockApp::new().await;
    let urls = vec![
        "https://example.com/1".to_string(),
        "https://example.com/2".to_string(),
    ];
    let tag = "imported";
    let response_body = json!(["uuid-1", "uuid-2"]);

    // The endpoint expects line-separated URLs in the body
    // and config in query params.
    app.mock_post_with_query(
        "/api/v1/import",
        "tag",
        "imported",
        200,
        Some(response_body),
    )
    .await;

    let result = app.client.import_watches(urls, Some(tag)).await.unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "uuid-1");
    assert_eq!(result[1], "uuid-2");
}
