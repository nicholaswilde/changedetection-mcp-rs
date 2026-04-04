mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_mcp_watch_ops_list_with_tag() {
    let app = MockApp::new().await;

    let response_body = json!({
        "watch_id_1": {
            "url": "https://example.com",
            "title": "Example"
        }
    });

    Mock::given(method("GET"))
        .and(path("/api/v1/watch"))
        .and(query_param("tag", "test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&app.server)
        .await;

    let params = json!({ "action": "List", "tag": "test" });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    let watches = result.get("watches").unwrap().as_object().unwrap();
    assert!(watches.get("watch_id_1").is_some());
    assert_eq!(watches["watch_id_1"]["url"], "https://example.com");
}

#[tokio::test]
async fn test_mcp_watch_ops_search() {
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
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&app.server)
        .await;

    let params = json!({ "action": "Search", "query": query });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    let watches: serde_json::Value = serde_json::from_value(result).unwrap();
    assert!(watches.get("watch_id_1").is_some());
}

#[tokio::test]
async fn test_mcp_watch_ops_update() {
    let app = MockApp::new().await;

    let uuid = "watch_id_1";
    let params = json!({
        "action": "Update",
        "uuid": uuid,
        "url": "https://new-example.com",
        "title": "New Example"
    });
    let response_body = json!({
        "status": "success"
    });

    app.mock_put(
        &format!("/api/v1/watch/{}", uuid),
        200,
        Some(response_body.clone()),
    )
    .await;

    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_tag_ops_list() {
    let app = MockApp::new().await;

    let response_body = json!({
        "tag_id_1": {
            "uuid": "tag_id_1",
            "title": "Tag 1"
        }
    });

    app.mock_get("/api/v1/tags", 200, Some(response_body.clone()))
        .await;

    let params = json!({ "action": "List" });
    let result = app
        .mcp
        .handle_method("tag_ops", Some(params))
        .await
        .unwrap();

    let tags = result.get("tags").unwrap().as_object().unwrap();
    assert!(tags.get("tag_id_1").is_some());
}

#[tokio::test]
async fn test_mcp_system_ops_get_info() {
    let app = MockApp::new().await;

    let response_body = json!({
        "watch_count": 10,
        "queue_size": 2,
        "overdue_watches": ["watch-1"],
        "uptime": 3600.0,
        "version": "0.45.2"
    });

    app.mock_get("/api/v1/systeminfo", 200, Some(response_body.clone()))
        .await;

    let params = json!({ "action": "GetInfo" });
    let result = app
        .mcp
        .handle_method("system_ops", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_tools_list_consolidated() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("tools/list", None).await.unwrap();

    let tools = result.get("tools").unwrap().as_array().unwrap();

    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&"watch_ops"));
    assert!(tool_names.contains(&"tag_ops"));
    assert!(tool_names.contains(&"notification_ops"));
    assert!(tool_names.contains(&"history_ops"));
    assert!(tool_names.contains(&"system_ops"));
}

#[tokio::test]
async fn test_mcp_watch_ops_get_details_missing_params() {
    let app = MockApp::new().await;

    let params = json!({ "action": "Get" }); // Missing uuid
    let result = app.mcp.handle_method("watch_ops", Some(params)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_mcp_api_error_consolidated() {
    let app = MockApp::new().await;

    app.mock_get("/api/v1/watch", 500, None).await;

    let params = json!({ "action": "List" });
    let result = app.mcp.handle_method("watch_ops", Some(params)).await;
    assert!(result.is_err());
}
