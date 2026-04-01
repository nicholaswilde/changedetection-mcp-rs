use changedetection_mcp_rs::api::Client;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate, Respond, Request};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct RetryResponder {
    count: Arc<AtomicUsize>,
}

impl Respond for RetryResponder {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        if self.count.fetch_add(1, Ordering::SeqCst) == 0 {
            ResponseTemplate::new(500)
        } else {
            ResponseTemplate::new(200).set_body_json(json!({
                "watch_id_1": {
                    "url": "https://example.com",
                    "title": "Example"
                }
            }))
        }
    }
}

#[tokio::test]
async fn test_api_client_retries_on_failure() {
    let mock_server = MockServer::start().await;
    let client = Client::new(mock_server.uri(), "test_api_key".to_string());

    let count = Arc::new(AtomicUsize::new(0));
    Mock::given(method("GET"))
        .and(path("/api/v1/watch"))
        .respond_with(RetryResponder { count: count.clone() })
        .expect(2)
        .mount(&mock_server)
        .await;

    let result = client.list_watches(None).await;
    assert!(result.is_ok(), "Client should have retried and succeeded: {:?}", result.err());
    let watches = result.unwrap();
    assert_eq!(watches.len(), 1);
    assert_eq!(count.load(Ordering::SeqCst), 2, "Should have made 2 requests");
}

#[tokio::test]
async fn test_api_client_caching() {
    let mock_server = MockServer::start().await;
    let client = Client::new(mock_server.uri(), "test_api_key".to_string());

    let response_body = json!({
        "watch_id_1": {
            "url": "https://example.com",
            "title": "Example"
        }
    });

    // Expect only 1 call despite 2 requests
    Mock::given(method("GET"))
        .and(path("/api/v1/watch"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(response_body)
            .insert_header("cache-control", "max-age=60"))
        .expect(1) 
        .mount(&mock_server)
        .await;

    client.list_watches(None).await.unwrap();
    client.list_watches(None).await.unwrap();
}
