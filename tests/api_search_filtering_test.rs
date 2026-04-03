mod common;

use common::MockApp;
use serde_json::json;

#[tokio::test]
async fn test_find_watches_by_error() {
    let app = MockApp::new().await;

    let response_body = json!({
        "uuid1": {
            "url": "https://example.com/error",
            "title": "Error Watch",
            "paused": false,
            "last_error": "404 Not Found"
        },
        "uuid2": {
            "url": "https://example.com/ok",
            "title": "OK Watch",
            "paused": false,
            "last_error": false
        }
    });

    app.mock_get("/api/v1/watch", 200, Some(response_body))
        .await;

    let error_watches = app.client.find_watches_by_error().await.unwrap();
    assert_eq!(error_watches.len(), 1);
    assert!(error_watches.contains_key("uuid1"));
    assert_eq!(error_watches["uuid1"].url, "https://example.com/error");
}

#[tokio::test]
async fn test_list_watches_by_processor() {
    let app = MockApp::new().await;

    let response_body = json!({
        "uuid1": {
            "url": "https://example.com/restock",
            "title": "Restock Watch",
            "paused": false,
            "processor": "restock_diff"
        },
        "uuid2": {
            "url": "https://example.com/text",
            "title": "Text Watch",
            "paused": false,
            "processor": "text_json_diff"
        }
    });

    app.mock_get("/api/v1/watch", 200, Some(response_body))
        .await;

    let restock_watches = app.client.list_watches_by_processor("restock_diff").await.unwrap();
    assert_eq!(restock_watches.len(), 1);
    assert!(restock_watches.contains_key("uuid1"));
}
