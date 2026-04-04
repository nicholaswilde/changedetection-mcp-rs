mod common;
use changedetection_mcp_rs::observability::init_tracing;
use common::MockApp;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;
use std::fs;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[test]
fn test_observability_init() {
    // Test normal init
    let _guard = init_tracing("info", None, false);

    // Test JSON init
    let _guard_json = init_tracing("debug", None, true);

    // Test with log file
    let log_file = "test_log.log";
    {
        let _guard_file = init_tracing("info", Some(log_file), false);
    }
    if fs::metadata(log_file).is_ok() {
        let _ = fs::remove_file(log_file);
    }

    // Test with log file and JSON
    let log_file_json = "test_log_json.log";
    {
        let _guard_file_json = init_tracing("info", Some(log_file_json), true);
    }
    if fs::metadata(log_file_json).is_ok() {
        let _ = fs::remove_file(log_file_json);
    }
}

#[tokio::test]
async fn test_mcp_watch_ops_create_delete() {
    let app = MockApp::new().await;
    let url = "https://example.com";
    let uuid = "test-uuid";

    // Create
    Mock::given(method("POST"))
        .and(path("/api/v1/watch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "uuid": uuid,
            "url": url
        })))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "Create",
        "url": url
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["uuid"], uuid);

    // Delete
    Mock::given(method("DELETE"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "Delete",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["status"], "success");
}

#[tokio::test]
async fn test_mcp_watch_ops_state_and_trigger() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";

    // Trigger
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}/recheck", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "Trigger",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["status"], "success");

    // Pause
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .and(query_param("paused", "paused"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "Pause",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["status"], "success");

    // Unpause
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .and(query_param("paused", "unpaused"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "Unpause",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["status"], "success");
}

#[tokio::test]
async fn test_mcp_watch_ops_mute_unmute() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";

    // Mute
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .and(query_param("muted", "muted"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "Mute",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["status"], "success");

    // Unmute
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .and(query_param("muted", "unmuted"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "Unmute",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["status"], "success");
}

#[tokio::test]
async fn test_mcp_watch_ops_trigger_all_mark_viewed() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";

    // TriggerAll
    Mock::given(method("GET"))
        .and(path("/api/v1/watch"))
        .and(query_param("recheck_all", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_string("OK"))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "TriggerAll"
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["status"], "success");

    // MarkAsViewed
    // First GET details
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "url": "https://example.com",
            "last_changed": 1000
        })))
        .mount(&app.server)
        .await;

    // Then PUT update
    Mock::given(method("PUT"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "MarkAsViewed",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["status"], "success");
}

#[tokio::test]
async fn test_mcp_system_ops_more() {
    let app = MockApp::new().await;

    // GetSpec
    Mock::given(method("GET"))
        .and(path("/api/v1/full-spec"))
        .respond_with(ResponseTemplate::new(200).set_body_string("openapi: 3.0.0"))
        .mount(&app.server)
        .await;

    let params = json!({ "action": "GetSpec" });
    let result = app
        .mcp
        .handle_method("system_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result, "openapi: 3.0.0");

    // ListFetchers
    Mock::given(method("GET"))
        .and(path("/api/v1/fetchers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!(["basic", "playwright"])))
        .mount(&app.server)
        .await;

    let params = json!({ "action": "ListFetchers" });
    let result = app
        .mcp
        .handle_method("system_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result, json!(["basic", "playwright"]));

    // ListProxies
    Mock::given(method("GET"))
        .and(path("/api/v1/proxies"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"p1": "url1", "p2": "url2"})))
        .mount(&app.server)
        .await;

    let params = json!({ "action": "ListProxies" });
    let result = app
        .mcp
        .handle_method("system_ops", Some(params))
        .await
        .unwrap();
    assert!(result["proxies"].is_object());
    assert_eq!(result["total"], 2);
}

#[tokio::test]
async fn test_mcp_system_ops_even_more() {
    let app = MockApp::new().await;

    // GetSettings
    Mock::given(method("GET"))
        .and(path("/api/v1/settings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"some": "setting"})))
        .mount(&app.server)
        .await;

    let params = json!({ "action": "GetSettings" });
    let result = app
        .mcp
        .handle_method("system_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["some"], "setting");

    // ListProcessors
    Mock::given(method("GET"))
        .and(path("/api/v1/processors"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!(["p1", "p2"])))
        .mount(&app.server)
        .await;

    let params = json!({ "action": "ListProcessors" });
    let result = app
        .mcp
        .handle_method("system_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result, json!(["p1", "p2"]));

    // AuditProxies
    Mock::given(method("GET"))
        .and(path("/api/v1/proxies"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"p1": "url1"})))
        .mount(&app.server)
        .await;

    let params = json!({ "action": "AuditProxies" });
    let result = app
        .mcp
        .handle_method("system_ops", Some(params))
        .await
        .unwrap();
    assert!(result["p1"].is_object());
}

#[tokio::test]
async fn test_mcp_history_ops_more() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";

    // GetHistory
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}/history", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"123": "/path"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "GetHistory",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("history_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["123"], "/path");

    // GetDiff
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}/difference/123/456", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_string("diff content"))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "GetDiff",
        "uuid": uuid,
        "from_timestamp": "123",
        "to_timestamp": "456"
    });
    let result = app
        .mcp
        .handle_method("history_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result, "diff content");
}

#[tokio::test]
async fn test_mcp_history_ops_even_more() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";

    // GetScreenshot
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}/screenshot", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_raw(vec![1, 2, 3], "image/png"))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "GetScreenshot",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("history_ops", Some(params))
        .await
        .unwrap();
    assert!(result.is_string()); // Base64

    // SetBulkLimit
    // 1. List watches with tag
    Mock::given(method("GET"))
        .and(path("/api/v1/watch"))
        .and(query_param("tag", "test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            uuid: {"url": "https://example.com"}
        })))
        .mount(&app.server)
        .await;

    // 2. Set limit for the watch
    Mock::given(method("PUT"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "SetBulkLimit",
        "tag": "test",
        "limit": 10
    });
    let result = app
        .mcp
        .handle_method("history_ops", Some(params))
        .await
        .unwrap();
    assert!(result.is_object());
    assert!(result.get(uuid).is_some());
}

#[tokio::test]
async fn test_mcp_watch_ops_list_filtering() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";

    // Mock list_watches
    Mock::given(method("GET"))
        .and(path("/api/v1/watch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            uuid: {"url": "https://example.com"}
        })))
        .mount(&app.server)
        .await;

    // Mock get_watch_details for state filtering
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "url": "https://example.com",
            "paused": true,
            "last_error": "Some error"
        })))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "List",
        "state": "paused"
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["total"], 1);

    let params_unpaused = json!({
        "action": "List",
        "state": "unpaused"
    });
    let result_unpaused = app
        .mcp
        .handle_method("watch_ops", Some(params_unpaused))
        .await
        .unwrap();
    assert_eq!(result_unpaused["total"], 0);

    let params_error = json!({
        "action": "List",
        "state": "error"
    });
    let result_error = app
        .mcp
        .handle_method("watch_ops", Some(params_error))
        .await
        .unwrap();
    assert_eq!(result_error["total"], 1);
}

#[tokio::test]
async fn test_mcp_watch_ops_more_config() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";

    // SetBrowserSteps
    Mock::given(method("PUT"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "SetBrowserSteps",
        "uuid": uuid,
        "browser_steps": [{"operation": "wait", "selector": "body"}]
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();
    assert!(result.is_object());

    // SetConditions
    let params_cond = json!({
        "action": "SetConditions",
        "uuid": uuid,
        "conditions": [{"field": "price", "operator": "less", "value": "100"}],
        "conditions_match_logic": "ALL"
    });
    let result_cond = app
        .mcp
        .handle_method("watch_ops", Some(params_cond))
        .await
        .unwrap();
    assert!(result_cond.is_object());

    // SetRequestConfig
    let params_req = json!({
        "action": "SetRequestConfig",
        "uuid": uuid,
        "headers": {"User-Agent": "Test"},
        "body": "post body"
    });
    let result_req = app
        .mcp
        .handle_method("watch_ops", Some(params_req))
        .await
        .unwrap();
    assert!(result_req.is_object());

    // GetFavicon
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}/favicon", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_raw(vec![1, 2, 3], "image/x-icon"))
        .mount(&app.server)
        .await;

    let params_favicon = json!({
        "action": "GetFavicon",
        "uuid": uuid
    });
    let result_favicon = app
        .mcp
        .handle_method("watch_ops", Some(params_favicon))
        .await
        .unwrap();
    assert!(result_favicon.is_string()); // Base64
}

#[tokio::test]
async fn test_mcp_watch_ops_search_and_get() {
    let app = MockApp::new().await;
    let uuid = "test-uuid";

    // Get
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/watch/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "url": "https://example.com"
        })))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "Get",
        "uuid": uuid
    });
    let result = app
        .mcp
        .handle_method("watch_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["url"], "https://example.com");
}

#[tokio::test]
async fn test_mcp_tag_ops_more() {
    let app = MockApp::new().await;
    let uuid = "tag-uuid";

    // Create Tag
    Mock::given(method("POST"))
        .and(path("/api/v1/tag"))
        .respond_with(ResponseTemplate::new(200).set_body_string(uuid))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "Create",
        "title": "New Tag"
    });
    let result = app
        .mcp
        .handle_method("tag_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result, uuid);

    // Get Tag
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/tag/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "title": "New Tag"
        })))
        .mount(&app.server)
        .await;

    let params_get = json!({
        "action": "Get",
        "uuid": uuid
    });
    let result_get = app
        .mcp
        .handle_method("tag_ops", Some(params_get))
        .await
        .unwrap();
    assert_eq!(result_get["title"], "New Tag");

    // Update Tag
    Mock::given(method("PUT"))
        .and(path(format!("/api/v1/tag/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params_update = json!({
        "action": "Update",
        "uuid": uuid,
        "title": "Updated Tag"
    });
    let result_update = app
        .mcp
        .handle_method("tag_ops", Some(params_update))
        .await
        .unwrap();
    assert_eq!(result_update["status"], "success");

    // Delete Tag
    Mock::given(method("DELETE"))
        .and(path(format!("/api/v1/tag/{}", uuid)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
        .mount(&app.server)
        .await;

    let params_delete = json!({
        "action": "Delete",
        "uuid": uuid
    });
    let result_delete = app
        .mcp
        .handle_method("tag_ops", Some(params_delete))
        .await
        .unwrap();
    assert_eq!(result_delete["status"], "success");
}

#[tokio::test]
async fn test_mcp_error_paths() {
    let app = MockApp::new().await;

    // Unknown method
    let result = app.mcp.handle_method("unknown_method", None).await;
    assert!(result.is_err());

    // WatchAction::Search without query
    let params = json!({ "action": "Search" });
    let result = app.mcp.handle_method("watch_ops", Some(params)).await;
    assert!(result.is_err());

    // TagAction::Get without uuid
    let params = json!({ "action": "Get" });
    let result = app.mcp.handle_method("tag_ops", Some(params)).await;
    assert!(result.is_err());

    // NotificationAction::Add without url
    let params = json!({ "action": "Add" });
    let result = app
        .mcp
        .handle_method("notification_ops", Some(params))
        .await;
    assert!(result.is_err());

    // HistoryAction::GetDiff without timestamps
    let params = json!({ "action": "GetDiff", "uuid": "uuid" });
    let result = app.mcp.handle_method("history_ops", Some(params)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_mcp_resources_unknown() {
    let app = MockApp::new().await;

    let params = json!({ "uri": "unknown://resource" });
    let result = app.mcp.handle_method("resources/read", Some(params)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_mcp_pagination_more() {
    let app = MockApp::new().await;

    // Tag pagination
    Mock::given(method("GET"))
        .and(path("/api/v1/tags"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "t1": {"title": "Tag 1"},
            "t2": {"title": "Tag 2"},
            "t3": {"title": "Tag 3"}
        })))
        .mount(&app.server)
        .await;

    let params = json!({
        "action": "List",
        "pagination": {"page": 1, "per_page": 2}
    });
    let result = app
        .mcp
        .handle_method("tag_ops", Some(params))
        .await
        .unwrap();
    assert_eq!(result["tags"].as_object().unwrap().len(), 2);
    assert_eq!(result["total"], 3);

    // Notification pagination
    Mock::given(method("GET"))
        .and(path("/api/v1/notifications"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "notification_urls": ["n1", "n2", "n3"]
        })))
        .mount(&app.server)
        .await;

    let params_n = json!({
        "action": "List",
        "pagination": {"page": 2, "per_page": 2}
    });
    let result_n = app
        .mcp
        .handle_method("notification_ops", Some(params_n))
        .await
        .unwrap();
    assert_eq!(result_n["notifications"].as_array().unwrap().len(), 1);
    assert_eq!(result_n["total"], 3);

    // History pagination
    Mock::given(method("GET"))
        .and(path("/api/v1/watch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "w1": {"url": "url1"}
        })))
        .mount(&app.server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/watch/w1/history"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ts1": "/p1", "ts2": "/p2", "ts3": "/p3"
        })))
        .mount(&app.server)
        .await;

    let params_h = json!({
        "action": "ListAll",
        "pagination": {"page": 1, "per_page": 2}
    });
    let result_h = app
        .mcp
        .handle_method("history_ops", Some(params_h))
        .await
        .unwrap();
    assert_eq!(result_h["history"].as_array().unwrap().len(), 2);
    assert_eq!(result_h["total"], 3);
}
