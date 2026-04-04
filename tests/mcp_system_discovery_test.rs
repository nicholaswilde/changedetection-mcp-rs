mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;

#[tokio::test]
async fn test_mcp_list_fetchers() {
    let app = MockApp::new().await;

    let response_body = json!(["html_requests", "html_webdriver"]);

    app.mock_get("/api/v1/fetchers", 200, Some(response_body))
        .await;

    let params = json!({
        "action": "ListFetchers"
    });

    let result = app
        .mcp
        .handle_method("system_ops", Some(params))
        .await
        .unwrap();
    let fetchers: Vec<String> = serde_json::from_value(result).unwrap();
    assert_eq!(fetchers.len(), 2);
    assert!(fetchers.contains(&"html_requests".to_string()));
}

#[tokio::test]
async fn test_mcp_list_proxies() {
    let app = MockApp::new().await;

    let response_body = json!({
        "p1": "http://proxy1"
    });

    app.mock_get("/api/v1/proxies", 200, Some(response_body))
        .await;

    let params = json!({
        "action": "ListProxies"
    });

    let result = app
        .mcp
        .handle_method("system_ops", Some(params))
        .await
        .unwrap();
    let proxies: std::collections::HashMap<String, String> =
        serde_json::from_value(result.get("proxies").unwrap().clone()).unwrap();
    assert_eq!(proxies.get("p1").unwrap(), "http://proxy1");
}

#[tokio::test]
async fn test_mcp_get_global_settings() {
    let app = MockApp::new().await;

    let response_body = json!({
        "setting1": "value1"
    });

    app.mock_get("/api/v1/settings", 200, Some(response_body))
        .await;

    let params = json!({
        "action": "GetSettings"
    });

    let result = app
        .mcp
        .handle_method("system_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result.get("setting1").unwrap(), "value1");
}
