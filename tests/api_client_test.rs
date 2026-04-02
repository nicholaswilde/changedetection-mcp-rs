mod common;

use common::MockApp;
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_list_watches() {
    let app = MockApp::new().await;

    let response_body = json!({
        "watch_id_1": {
            "url": "https://example.com",
            "title": "Example"
        }
    });

    app.mock_get("/api/v1/watch", 200, Some(response_body))
        .await;

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

    app.mock_get(&format!("/api/v1/watch/{}", uuid), 200, Some(response_body))
        .await;

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

    app.mock_post("/api/v1/watch", 201, Some(response_body))
        .await;

    let result = app
        .client
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

    app.mock_delete(&format!("/api/v1/watch/{}", uuid), 200, Some(response_body))
        .await;

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

    app.mock_get_with_query(
        &format!("/api/v1/watch/{}", uuid),
        "recheck",
        "1",
        200,
        Some(response_body),
    )
    .await;

    let result = app.client.trigger_check(uuid).await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}

#[tokio::test]
async fn test_get_watch_history() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let response_body = json!({
        "1234567890": "Snapshot 1",
        "1234567891": "Snapshot 2"
    });

    app.mock_get(
        &format!("/api/v1/watch/{}/history", uuid),
        200,
        Some(response_body),
    )
    .await;

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

    app.mock_get_text(
        &format!("/api/v1/watch/{}/difference/{}/{}", uuid, from, to),
        200,
        response_body,
    )
    .await;

    let diff = app.client.get_watch_diff(uuid, from, to, None).await.unwrap();
    assert_eq!(diff, response_body);
}

#[tokio::test]
async fn test_update_watch() {
    let app = MockApp::new().await;

    let uuid = "watch_id_1";
    let payload = json!({
        "url": "https://new-example.com",
        "title": "New Example"
    });
    let response_body = json!({
        "status": "success"
    });

    app.mock_put(&format!("/api/v1/watch/{}", uuid), 200, Some(response_body))
        .await;

    let result = app.client.update_watch(uuid, payload).await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}

#[tokio::test]
async fn test_search_watches() {
    let app = MockApp::new().await;

    let query = "example";
    let response_body = json!({
        "watch_id_1": {
            "url": "https://example.com",
            "title": "Example"
        }
    });

    Mock::given(method("GET"))
        .and(path("/api/v1/search"))
        .and(query_param("q", query))
        .and(query_param("partial", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&app.server)
        .await;

    let watches = app.client.search_watches(query).await.unwrap();
    assert_eq!(watches.len(), 1);
    assert!(watches.contains_key("watch_id_1"));
}

#[tokio::test]
async fn test_list_tags() {
    let app = MockApp::new().await;

    let response_body = json!({
        "tag_id_1": {
            "uuid": "tag_id_1",
            "title": "Tag 1"
        }
    });

    app.mock_get("/api/v1/tags", 200, Some(response_body)).await;

    let tags = app.client.list_tags().await.unwrap();
    assert_eq!(tags.as_object().unwrap().len(), 1);
    assert_eq!(tags["tag_id_1"]["title"], "Tag 1");
}

#[tokio::test]
async fn test_create_tag() {
    let app = MockApp::new().await;

    let response_body = json!("tag_id_1");

    app.mock_post("/api/v1/tag", 201, Some(response_body)).await;

    let result = app.client.create_tag("New Tag").await.unwrap();
    assert_eq!(result, "tag_id_1");
}

#[tokio::test]
async fn test_get_tag_details() {
    let app = MockApp::new().await;

    let uuid = "tag_id_1";
    let response_body = json!({
        "uuid": uuid,
        "title": "Tag 1"
    });

    app.mock_get(&format!("/api/v1/tag/{}", uuid), 200, Some(response_body))
        .await;

    let tag = app.client.get_tag_details(uuid).await.unwrap();
    assert_eq!(tag["title"], "Tag 1");
}

#[tokio::test]
async fn test_update_tag() {
    let app = MockApp::new().await;

    let uuid = "tag_id_1";
    let payload = json!({
        "title": "Updated Tag"
    });
    let response_body = json!({
        "status": "success"
    });

    app.mock_put(&format!("/api/v1/tag/{}", uuid), 200, Some(response_body))
        .await;

    let result = app.client.update_tag(uuid, payload).await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}

#[tokio::test]
async fn test_delete_tag() {
    let app = MockApp::new().await;

    let uuid = "tag_id_1";
    let response_body = json!({
        "status": "success"
    });

    app.mock_delete(&format!("/api/v1/tag/{}", uuid), 200, Some(response_body))
        .await;

    let result = app.client.delete_tag(uuid).await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}

#[tokio::test]
async fn test_get_system_info() {
    let app = MockApp::new().await;

    let response_body = json!({
        "watch_count": 10,
        "queue_size": 2,
        "overdue_watches": ["watch-1"],
        "uptime": 3600.0,
        "version": "0.45.2"
    });

    app.mock_get("/api/v1/systeminfo", 200, Some(response_body))
        .await;

    let info = app.client.get_system_info().await.unwrap();
    assert_eq!(info.watch_count, 10);
    assert_eq!(info.version, "0.45.2");
    assert_eq!(info.overdue_watches.len(), 1);
}
