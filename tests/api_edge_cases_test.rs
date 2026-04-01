mod common;

use common::MockApp;
use changedetection_mcp_rs::api::ApiError;

#[tokio::test]
async fn test_api_401_unauthorized() {
    let app = MockApp::new().await;
    app.mock_get("/api/v1/watch", 401, None).await;

    let result = app.client.list_watches(None).await;
    match result {
        Err(ApiError::Http(e)) => assert_eq!(e.status(), Some(reqwest::StatusCode::UNAUTHORIZED)),
        _ => panic!("Expected HTTP 401 Unauthorized error, got {:?}", result),
    }
}

#[tokio::test]
async fn test_api_403_forbidden() {
    let app = MockApp::new().await;
    app.mock_get("/api/v1/watch", 403, None).await;

    let result = app.client.list_watches(None).await;
    match result {
        Err(ApiError::Http(e)) => assert_eq!(e.status(), Some(reqwest::StatusCode::FORBIDDEN)),
        _ => panic!("Expected HTTP 403 Forbidden error, got {:?}", result),
    }
}

#[tokio::test]
async fn test_api_404_not_found() {
    let app = MockApp::new().await;
    app.mock_get("/api/v1/watch", 404, None).await;

    let result = app.client.list_watches(None).await;
    match result {
        Err(ApiError::Http(e)) => assert_eq!(e.status(), Some(reqwest::StatusCode::NOT_FOUND)),
        _ => panic!("Expected HTTP 404 Not Found error, got {:?}", result),
    }
}

#[tokio::test]
async fn test_api_500_internal_error() {
    let app = MockApp::new().await;
    app.mock_get("/api/v1/watch", 500, None).await;

    let result = app.client.list_watches(None).await;
    match result {
        Err(ApiError::Http(e)) => assert_eq!(e.status(), Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR)),
        _ => panic!("Expected HTTP 500 Internal Error, got {:?}", result),
    }
}
