mod common;

use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_mcp_list_watches_with_tag() {
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

    let params = json!({ "tag": "test" });
    let result = app
        .mcp
        .handle_method("list_watches", Some(params))
        .await
        .unwrap();

    let watches: serde_json::Value = serde_json::from_value(result).unwrap();
    assert!(watches.get("watch_id_1").is_some());
    assert_eq!(watches["watch_id_1"]["url"], "https://example.com");
}

#[tokio::test]
async fn test_mcp_list_watches() {
    let app = MockApp::new().await;

    let response_body = json!({
        "watch_id_1": {
            "url": "https://example.com",
            "title": "Example"
        }
    });

    app.mock_get("/api/v1/watch", 200, Some(response_body))
        .await;

    let result = app.mcp.handle_method("list_watches", None).await.unwrap();

    let watches: serde_json::Value = serde_json::from_value(result).unwrap();
    assert!(watches.get("watch_id_1").is_some());
    assert_eq!(watches["watch_id_1"]["url"], "https://example.com");
}

#[tokio::test]
async fn test_mcp_tools_list() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("tools/list", None).await.unwrap();

    let tools = result.get("tools").unwrap().as_array().unwrap();
    assert_eq!(tools.len(), 20);

    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&"get_full_spec"));
    assert!(tool_names.contains(&"list_watches"));
    assert!(tool_names.contains(&"get_watch_history"));
    assert!(tool_names.contains(&"get_watch_diff"));
    assert!(tool_names.contains(&"update_watch"));
    assert!(tool_names.contains(&"search_watches"));
    assert!(tool_names.contains(&"list_tags"));
    assert!(tool_names.contains(&"create_tag"));
    assert!(tool_names.contains(&"get_tag_details"));
    assert!(tool_names.contains(&"update_tag"));
    assert!(tool_names.contains(&"delete_tag"));
    assert!(tool_names.contains(&"get_system_info"));
}

#[tokio::test]
async fn test_mcp_get_system_info() {
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

    let result = app
        .mcp
        .handle_method("get_system_info", None)
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_list_tags() {
    let app = MockApp::new().await;

    let response_body = json!({
        "tag_id_1": {
            "uuid": "tag_id_1",
            "title": "Tag 1"
        }
    });

    app.mock_get("/api/v1/tags", 200, Some(response_body.clone()))
        .await;

    let result = app.mcp.handle_method("list_tags", None).await.unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_create_tag() {
    let app = MockApp::new().await;

    let response_body = json!("tag_id_1");

    app.mock_post("/api/v1/tag", 201, Some(response_body.clone()))
        .await;

    let params = json!({ "title": "New Tag" });
    let result = app
        .mcp
        .handle_method("create_tag", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_get_tag_details() {
    let app = MockApp::new().await;

    let uuid = "tag_id_1";
    let response_body = json!({
        "uuid": uuid,
        "title": "Tag 1"
    });

    app.mock_get(
        &format!("/api/v1/tag/{}", uuid),
        200,
        Some(response_body.clone()),
    )
    .await;

    let params = json!({ "uuid": uuid });
    let result = app
        .mcp
        .handle_method("get_tag_details", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_update_tag() {
    let app = MockApp::new().await;

    let uuid = "tag_id_1";
    let params = json!({
        "uuid": uuid,
        "title": "Updated Tag"
    });
    let response_body = json!({
        "status": "success"
    });

    app.mock_put(
        &format!("/api/v1/tag/{}", uuid),
        200,
        Some(response_body.clone()),
    )
    .await;

    let result = app
        .mcp
        .handle_method("update_tag", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_delete_tag() {
    let app = MockApp::new().await;

    let uuid = "tag_id_1";
    let response_body = json!({
        "status": "success"
    });

    app.mock_delete(
        &format!("/api/v1/tag/{}", uuid),
        200,
        Some(response_body.clone()),
    )
    .await;

    let params = json!({ "uuid": uuid });
    let result = app
        .mcp
        .handle_method("delete_tag", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_search_watches() {
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

    let params = json!({ "query": query });
    let result = app
        .mcp
        .handle_method("search_watches", Some(params))
        .await
        .unwrap();

    let watches: serde_json::Value = serde_json::from_value(result).unwrap();
    assert!(watches.get("watch_id_1").is_some());
}

#[tokio::test]
async fn test_mcp_update_watch() {
    let app = MockApp::new().await;

    let uuid = "watch_id_1";
    let params = json!({
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
        .handle_method("update_watch", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_get_watch_details() {
    let app = MockApp::new().await;

    let uuid = "watch_id_1";
    let response_body = json!({
        "url": "https://example.com",
        "title": "Example"
    });

    app.mock_get(&format!("/api/v1/watch/{}", uuid), 200, Some(response_body))
        .await;

    let params = json!({ "uuid": uuid });
    let result = app
        .mcp
        .handle_method("get_watch_details", Some(params))
        .await
        .unwrap();

    let watch: serde_json::Value = serde_json::from_value(result).unwrap();
    assert_eq!(watch["url"], "https://example.com");
}

#[tokio::test]
async fn test_mcp_create_watch() {
    let app = MockApp::new().await;

    let response_body = json!({
        "status": "success",
        "uuid": "watch_id_1"
    });

    app.mock_post("/api/v1/watch", 201, Some(response_body))
        .await;

    let params = json!({ "url": "https://example.com" });
    let result = app
        .mcp
        .handle_method("create_watch", Some(params))
        .await
        .unwrap();

    let res: serde_json::Value = serde_json::from_value(result).unwrap();
    assert_eq!(res["status"], "success");
    assert_eq!(res["uuid"], "watch_id_1");
}

#[tokio::test]
async fn test_mcp_delete_watch() {
    let app = MockApp::new().await;

    let uuid = "watch_id_1";
    let response_body = json!({
        "status": "success"
    });

    app.mock_delete(&format!("/api/v1/watch/{}", uuid), 200, Some(response_body))
        .await;

    let params = json!({ "uuid": uuid });
    let result = app
        .mcp
        .handle_method("delete_watch", Some(params))
        .await
        .unwrap();

    let res: serde_json::Value = serde_json::from_value(result).unwrap();
    assert_eq!(res["status"], "success");
}

#[tokio::test]
async fn test_mcp_trigger_check() {
    let app = MockApp::new().await;

    let uuid = "watch_id_1";
    let response_body = json!({
        "status": "success"
    });

    app.mock_get_with_query(
        &format!("/api/v1/watch/{}", uuid),
        "recheck",
        "1",
        200,
        Some(response_body),
    )
    .await;

    let params = json!({ "uuid": uuid });
    let result = app
        .mcp
        .handle_method("trigger_check", Some(params))
        .await
        .unwrap();

    let res: serde_json::Value = serde_json::from_value(result).unwrap();
    assert_eq!(res["status"], "success");
}

#[tokio::test]
async fn test_mcp_get_watch_history() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let response_body = json!({
        "1234567890": "Snapshot 1",
        "1234567891": "Snapshot 2"
    });

    app.mock_get(
        &format!("/api/v1/watch/{}/history", uuid),
        200,
        Some(response_body.clone()),
    )
    .await;

    let params = json!({ "uuid": uuid });
    let result = app
        .mcp
        .handle_method("get_watch_history", Some(params))
        .await
        .unwrap();

    assert_eq!(result, response_body);
}

#[tokio::test]
async fn test_mcp_get_watch_diff() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";
    let from = "1234567890";
    let to = "1234567891";
    let response_body = "Difference between snapshots";

    app.mock_get_text(
        &format!("/api/v1/watch/{}/difference/{}/{}", uuid, from, to),
        200,
        response_body,
    )
    .await;

    let params = json!({
        "uuid": uuid,
        "from_timestamp": from,
        "to_timestamp": to
    });
    let result = app
        .mcp
        .handle_method("get_watch_diff", Some(params))
        .await
        .unwrap();

    assert_eq!(result, json!(response_body));
}

#[tokio::test]
async fn test_mcp_get_watch_details_missing_params() {
    let app = MockApp::new().await;

    let result = app.mcp.handle_method("get_watch_details", None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_mcp_api_error() {
    let app = MockApp::new().await;

    app.mock_get("/api/v1/watch", 500, None).await;

    let result = app.mcp.handle_method("list_watches", None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_mcp_get_full_spec() {
    let app = MockApp::new().await;

    let response_body = "openapi: 3.0.0";

    app.mock_get_text("/api/v1/full-spec", 200, response_body)
        .await;

    let result = app.mcp.handle_method("get_full_spec", None).await.unwrap();

    assert_eq!(result, json!(response_body));
}
