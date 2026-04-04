mod common;
use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_mcp_set_browser_steps() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";

    Mock::given(method("PUT"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "SetBrowserSteps",
        "uuid": uuid,
        "browser_steps": [
            {
                "operation": "click",
                "selector": "#login-button",
                "optional_value": ""
            }
        ]
    });
    let result = app.mcp.handle_method("watch_ops", Some(params)).await;

    assert!(result.is_ok());
}
