mod common;

use common::MockApp;
use serde_json::json;

#[tokio::test]
async fn test_list_fetchers() {
    let app = MockApp::new().await;

    let response_body = json!(["html_requests", "html_webdriver", "playwright"]);

    app.mock_get("/api/v1/fetchers", 200, Some(response_body))
        .await;

    let fetchers_val = app.client.list_fetchers().await.unwrap();
    let fetchers: Vec<String> = serde_json::from_value(fetchers_val).unwrap();
    assert_eq!(fetchers.len(), 3);
    assert!(fetchers.contains(&"html_requests".to_string()));
}

#[tokio::test]
async fn test_list_proxies() {
    let app = MockApp::new().await;

    let response_body = json!({
        "proxy1": "http://user:pass@1.2.3.4:8080",
        "proxy2": "socks5://5.6.7.8:1080"
    });

    app.mock_get("/api/v1/proxies", 200, Some(response_body))
        .await;

    let proxies = app.client.list_proxies().await.unwrap();
    assert_eq!(
        proxies.get("proxy1").unwrap(),
        "http://user:pass@1.2.3.4:8080"
    );
}

#[tokio::test]
async fn test_get_global_settings() {
    let app = MockApp::new().await;

    let response_body = json!({
        "default_check_interval": 3600,
        "global_notification_urls": ["mailto:admin@example.com"],
        "system_wide_headers": {
            "User-Agent": "ChangeDetection/1.0"
        }
    });

    app.mock_get("/api/v1/settings", 200, Some(response_body))
        .await;

    let settings = app.client.get_global_settings().await.unwrap();
    assert_eq!(
        settings
            .get("default_check_interval")
            .unwrap()
            .as_i64()
            .unwrap(),
        3600
    );
}
