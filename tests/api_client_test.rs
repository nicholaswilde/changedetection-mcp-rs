use changedetection_mcp_rs::api::Client;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_watches() {
    let mock_server = MockServer::start().await;
    let client = Client::new(mock_server.uri(), "test_api_key".to_string());

    let response_body = json!({
        "watch_id_1": {
            "url": "https://example.com",
            "title": "Example"
        }
    });

    Mock::given(method("GET"))
        .and(path("/api/v1/watch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let watches = client.list_watches().await.unwrap();
    assert_eq!(watches.len(), 1);
}

#[tokio::test]
async fn test_get_watch_details() {
    let mock_server = MockServer::start().await;
    let client = Client::new(mock_server.uri(), "test_api_key".to_string());

    let uuid = "watch_id_1";
    let response_body = json!({
        "url": "https://example.com",
        "title": "Example"
    });

    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let watch = client.get_watch_details(uuid).await.unwrap();
    assert_eq!(watch.url, "https://example.com");
}

#[tokio::test]
async fn test_create_watch() {
    let mock_server = MockServer::start().await;
    let client = Client::new(mock_server.uri(), "test_api_key".to_string());

    let response_body = json!({
        "status": "success",
        "uuid": "watch_id_1"
    });

    Mock::given(method("POST"))
        .and(path("/api/v1/watch"))
        .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let result = client
        .create_watch("https://example.com", None)
        .await
        .unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
    assert_eq!(result.get("uuid").unwrap(), "watch_id_1");
}

#[tokio::test]
async fn test_delete_watch() {
    let mock_server = MockServer::start().await;
    let client = Client::new(mock_server.uri(), "test_api_key".to_string());

    let uuid = "watch_id_1";
    let response_body = json!({
        "status": "success"
    });

    Mock::given(method("DELETE"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let result = client.delete_watch(uuid).await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}

#[tokio::test]
async fn test_trigger_check() {
    let mock_server = MockServer::start().await;
    let client = Client::new(mock_server.uri(), "test_api_key".to_string());

    let uuid = "watch_id_1";
    let response_body = json!({
        "status": "success"
    });

    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}/recheck", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let result = client.trigger_check(uuid).await.unwrap();
    assert_eq!(result.get("status").unwrap(), "success");
}
