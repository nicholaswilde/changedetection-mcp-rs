use changedetection_mcp_rs::api::Client;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_full_spec_success() {
    let mock_server = MockServer::start().await;
    let api_key = "test-api-key";
    let client = Client::new(mock_server.uri(), api_key.to_string());

    let yaml_spec = r#"
openapi: 3.0.0
info:
  title: ChangeDetection.io API
  version: v1
paths:
  /api/v1/watch:
    get:
      summary: List watches
"#;

    Mock::given(method("GET"))
        .and(path("/api/v1/full-spec"))
        .and(header("x-api-key", api_key))
        .respond_with(ResponseTemplate::new(200).set_body_string(yaml_spec))
        .mount(&mock_server)
        .await;

    let result = client.get_full_spec().await.unwrap();
    assert_eq!(result, yaml_spec);
}

#[tokio::test]
async fn test_get_full_spec_error() {
    let mock_server = MockServer::start().await;
    let api_key = "test-api-key";
    let client = Client::new(mock_server.uri(), api_key.to_string());

    Mock::given(method("GET"))
        .and(path("/api/v1/full-spec"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let result = client.get_full_spec().await;
    assert!(result.is_err());
}
