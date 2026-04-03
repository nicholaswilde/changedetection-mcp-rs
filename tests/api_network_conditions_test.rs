mod common;

use changedetection_mcp_rs::api::ApiError;
use common::MockApp;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_api_timeout() {
    // Set a very short timeout (1s)
    let app = MockApp::new_with_timeout(Duration::from_secs(1)).await;
    let unique_uuid = uuid::Uuid::new_v4().to_string();

    // Simulate a 5-second delay
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}", unique_uuid)))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(5)))
        .mount(&app.server)
        .await;

    // We expect this to fail due to timeout
    let result = app.client.get_watch_details(&unique_uuid).await;

    match result {
        Err(ApiError::Http(e)) if e.is_timeout() => (),
        Err(ApiError::Middleware(e)) => {
            let msg = e.to_string().to_lowercase();
            if msg.contains("timeout")
                || msg.contains("timed out")
                || msg.contains("sending request")
            {
                // "sending request" is often the prefix for timeout in some middleware stacks
            } else {
                panic!("Expected timeout error, got Middleware error: {}", e);
            }
        }
        _ => panic!("Expected timeout error, got {:?}", result),
    }
}
