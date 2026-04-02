mod common;

use changedetection_mcp_rs::api::ApiError;
use common::MockApp;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_api_invalid_json() {
    let app = MockApp::new().await;

    // Return something that is NOT JSON
    Mock::given(method("GET"))
        .and(path("/api/v1/watch"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Not JSON at all"))
        .mount(&app.server)
        .await;

    let result = app.client.list_watches(None).await;

    match result {
        Err(ApiError::Http(e)) if e.is_decode() => (),
        _ => panic!("Expected HTTP decode error, got {:?}", result),
    }
}

#[tokio::test]
async fn test_api_unexpected_json_schema() {
    let app = MockApp::new().await;

    // Return valid JSON but not what we expect (e.g. an array instead of a map)
    Mock::given(method("GET"))
        .and(path("/api/v1/watch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([1, 2, 3])))
        .mount(&app.server)
        .await;

    let result = app.client.list_watches(None).await;

    match result {
        Err(ApiError::Http(e)) if e.is_decode() => (),
        _ => panic!("Expected HTTP decode error, got {:?}", result),
    }
}
