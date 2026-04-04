use changedetection_mcp_rs::api::Client;
use changedetection_mcp_rs::mcp::McpServer;
use mcp_sdk_rs::server::ServerHandler;
use std::env;
use uuid::Uuid;

fn wrap_action(action: &str, params: Option<serde_json::Value>) -> Option<serde_json::Value> {
    let mut p = params.unwrap_or_else(|| serde_json::json!({}));
    if let Some(obj) = p.as_object_mut() {
        obj.insert("action".to_string(), serde_json::json!(action));
    }
    Some(p)
}

#[tokio::test]
async fn test_live_tag_lifecycle() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Create a tag
    let tag_name = format!("test-tag-{}", Uuid::new_v4());
    let create_params = serde_json::json!({
        "title": tag_name
    });
    let create_result = mcp
        .handle_method("tag_ops", wrap_action("Create", Some(create_params)))
        .await
        .expect("Failed to create tag");

    // The API might return a UUID string or a JSON object with a uuid field
    let uuid = if create_result.is_string() {
        create_result.as_str().unwrap().to_string()
    } else {
        create_result
            .get("uuid")
            .and_then(|v| v.as_str())
            .expect("No uuid in create result")
            .to_string()
    };
    println!("Created tag: {} ({})", tag_name, uuid);

    // 2. Get details
    let details_params = serde_json::json!({ "uuid": uuid });
    let details = mcp
        .handle_method("tag_ops", wrap_action("Get", Some(details_params)))
        .await
        .expect("Failed to get tag details");
    // ChangeDetection.io API might return title or name depending on version/endpoint
    assert!(details.get("title").is_some() || details.get("name").is_some());
    println!("Tag details: {:?}", details);

    // 3. Update tag
    let update_params = serde_json::json!({
        "uuid": uuid,
        "title": format!("{}-updated", tag_name)
    });
    mcp.handle_method("tag_ops", wrap_action("Update", Some(update_params)))
        .await
        .expect("Failed to update tag");

    // Verify update
    let updated_details = mcp
        .handle_method(
            "tag_ops",
            wrap_action("Get", Some(serde_json::json!({ "uuid": uuid }))),
        )
        .await
        .expect("Failed to get updated tag details");
    let updated_title = updated_details
        .get("title")
        .or(updated_details.get("name"))
        .and_then(|v| v.as_str())
        .expect("Missing title/name in updated details");
    assert!(updated_title.contains("updated"));

    // 4. List tags
    let tags = mcp
        .handle_method("tag_ops", wrap_action("List", None))
        .await
        .expect("Failed to list tags");
    println!("Tags list: {:?}", tags);
    // tags is now a map where keys are UUIDs
    assert!(tags.is_object());
    let found = tags
        .get("tags")
        .and_then(|v| v.as_object())
        .unwrap_or_else(|| tags.as_object().unwrap())
        .contains_key(&uuid);
    assert!(found, "Created tag not found in list_tags map");

    // 5. Delete tag
    let delete_params = serde_json::json!({ "uuid": uuid });
    mcp.handle_method("tag_ops", wrap_action("Delete", Some(delete_params)))
        .await
        .expect("Failed to delete tag");
    println!("Deleted tag: {}", uuid);
}

#[tokio::test]
async fn test_live_system_info() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    let result = mcp
        .handle_method("system_ops", wrap_action("GetInfo", None))
        .await
        .expect("Failed to get system info");

    assert!(result.get("version").is_some());
    assert!(result.get("watch_count").is_some());
    println!("Live System Info: {:?}", result);
}

#[tokio::test]
async fn test_live_list_watches() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    let result = mcp
        .handle_method("watch_ops", wrap_action("List", None))
        .await
        .expect("Failed to list watches");

    assert!(result.is_object());
    println!("Live Watches count: {}", result.as_object().unwrap().len());
}

#[tokio::test]
async fn test_live_automation() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Create a watch
    let watch_url = format!("https://example.com/automation-test-{}", Uuid::new_v4());
    let create_params = serde_json::json!({
        "url": watch_url,
        "title": "Automation Test"
    });
    let create_result = mcp
        .handle_method("watch_ops", wrap_action("Create", Some(create_params)))
        .await
        .expect("Failed to create watch");

    let uuid = if create_result.is_string() {
        create_result.as_str().unwrap().to_string()
    } else if let Some(u) = create_result.get("uuid").and_then(|v| v.as_str()) {
        u.to_string()
    } else {
        // Search for it to get UUID
        let search_params = serde_json::json!({ "query": watch_url });
        let search_result = mcp
            .handle_method("watch_ops", wrap_action("Search", Some(search_params)))
            .await
            .expect("Failed to search for created watch");
        let watches = search_result
            .get("watches")
            .and_then(|v| v.as_object())
            .unwrap();
        watches
            .keys()
            .next()
            .expect("No watch found after creation")
            .clone()
    };
    println!("Created watch for automation test: {}", uuid);

    // 2. Set browser steps
    let steps = serde_json::json!([
        {
            "operation": "wait",
            "selector": "",
            "optional_value": "2"
        },
        {
            "operation": "click",
            "selector": "body",
            "optional_value": ""
        }
    ]);
    let step_params = serde_json::json!({
        "uuid": uuid,
        "browser_steps": steps
    });
    mcp.handle_method(
        "watch_ops",
        wrap_action("SetBrowserSteps", Some(step_params)),
    )
    .await
    .expect("Failed to set browser steps");

    // 3. Verify steps (using Get)
    let get_params = serde_json::json!({ "uuid": uuid });
    let details = mcp
        .handle_method("watch_ops", wrap_action("Get", Some(get_params)))
        .await
        .expect("Failed to get watch details");

    let browser_steps = details
        .get("browser_steps")
        .and_then(|v| v.as_array())
        .expect("Missing browser_steps in details");
    assert!(browser_steps.len() >= 2); // Might have default steps
    let has_wait = browser_steps
        .iter()
        .any(|s| s["operation"] == "wait" && s["optional_value"] == "2");
    let has_click = browser_steps
        .iter()
        .any(|s| s["operation"] == "click" && s["selector"] == "body");
    assert!(has_wait);
    assert!(has_click);

    // 4. Cleanup
    let delete_params = serde_json::json!({ "uuid": uuid });
    mcp.handle_method("watch_ops", wrap_action("Delete", Some(delete_params)))
        .await
        .expect("Failed to delete watch");
}

#[tokio::test]
async fn test_live_conditions() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Create a watch
    let watch_url = format!("https://example.com/conditions-test-{}", Uuid::new_v4());
    let create_params = serde_json::json!({
        "url": watch_url,
        "title": "Conditions Test"
    });
    let create_result = mcp
        .handle_method("watch_ops", wrap_action("Create", Some(create_params)))
        .await
        .expect("Failed to create watch");

    let uuid = if create_result.is_string() {
        create_result.as_str().unwrap().to_string()
    } else if let Some(u) = create_result.get("uuid").and_then(|v| v.as_str()) {
        u.to_string()
    } else {
        // Search for it to get UUID
        let search_params = serde_json::json!({ "query": watch_url });
        let search_result = mcp
            .handle_method("watch_ops", wrap_action("Search", Some(search_params)))
            .await
            .expect("Failed to search for created watch");
        let watches = search_result
            .get("watches")
            .and_then(|v| v.as_object())
            .unwrap();
        watches
            .keys()
            .next()
            .expect("No watch found after creation")
            .clone()
    };
    println!("Created watch for conditions test: {}", uuid);

    // 2. Set conditions
    let conditions = serde_json::json!([
        {
            "field": "page_title",
            "operator": "contains",
            "value": "Example"
        }
    ]);
    let cond_params = serde_json::json!({
        "uuid": uuid,
        "conditions": conditions,
        "conditions_match_logic": "ANY"
    });
    mcp.handle_method("watch_ops", wrap_action("SetConditions", Some(cond_params)))
        .await
        .expect("Failed to set conditions");

    // 3. Verify conditions (using Get)
    let get_params = serde_json::json!({ "uuid": uuid });
    let details = mcp
        .handle_method("watch_ops", wrap_action("Get", Some(get_params)))
        .await
        .expect("Failed to get watch details");

    let conds = details
        .get("conditions")
        .and_then(|v| v.as_array())
        .expect("Missing conditions in details");
    assert!(conds.iter().any(|c| c["field"] == "page_title"
        && c["operator"] == "contains"
        && c["value"] == "Example"));
    assert_eq!(details["conditions_match_logic"], "ANY");

    // 4. Cleanup
    let delete_params = serde_json::json!({ "uuid": uuid });
    mcp.handle_method("watch_ops", wrap_action("Delete", Some(delete_params)))
        .await
        .expect("Failed to delete watch");
}

#[tokio::test]
async fn test_live_trigger_all() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Trigger all
    let result = mcp
        .handle_method("watch_ops", wrap_action("TriggerAll", None))
        .await
        .expect("Failed to trigger all watches");

    assert!(result.is_object());
    let status = result["status"].as_str().unwrap();
    assert!(status == "success" || status.contains("OK"));
}

#[tokio::test]
async fn test_live_mark_as_viewed() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Find any watch
    let watches_result = mcp
        .handle_method("watch_ops", wrap_action("List", None))
        .await
        .expect("Failed to list watches");
    let uuid = watches_result
        .get("watches")
        .and_then(|v| v.as_object())
        .unwrap_or_else(|| watches_result.as_object().unwrap())
        .keys()
        .next()
        .expect("No watches found for live test")
        .clone();

    // 2. Mark as viewed
    let params = serde_json::json!({ "uuid": uuid });
    let result = mcp
        .handle_method("watch_ops", wrap_action("MarkAsViewed", Some(params)))
        .await
        .expect("Failed to mark watch as viewed");

    println!("Mark as viewed result: {:?}", result);
    assert!(result.is_object());
    assert_eq!(result["status"], "success");
}

#[tokio::test]
async fn test_live_bulk_retention() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Create two watches with a unique tag
    let unique_tag = format!("bulk-retention-test-{}", Uuid::new_v4());
    let watch1_url = format!("https://example.com/bulk-1-{}", Uuid::new_v4());
    let watch2_url = format!("https://example.com/bulk-2-{}", Uuid::new_v4());

    let _ = mcp
        .handle_method(
            "watch_ops",
            wrap_action(
                "Create",
                Some(serde_json::json!({
                    "url": watch1_url,
                    "tag": unique_tag
                })),
            ),
        )
        .await
        .expect("Failed to create watch 1");

    let _ = mcp
        .handle_method(
            "watch_ops",
            wrap_action(
                "Create",
                Some(serde_json::json!({
                    "url": watch2_url,
                    "tag": unique_tag
                })),
            ),
        )
        .await
        .expect("Failed to create watch 2");

    // 2. Set bulk limit
    let params = serde_json::json!({
        "tag": unique_tag,
        "limit": 50
    });
    let result = mcp
        .handle_method("history_ops", wrap_action("SetBulkLimit", Some(params)))
        .await
        .expect("Failed to set bulk history limit");

    assert!(result.is_object());
    assert!(result.as_object().unwrap().len() >= 2);
    for (_, res) in result.as_object().unwrap() {
        assert_eq!(res["status"], "success");
    }

    // 3. Cleanup
    let watches_result = mcp
        .handle_method(
            "watch_ops",
            wrap_action("List", Some(serde_json::json!({ "tag": unique_tag }))),
        )
        .await
        .expect("Failed to list watches by tag");

    let watches = watches_result
        .get("watches")
        .and_then(|v| v.as_object())
        .unwrap_or_else(|| watches_result.as_object().unwrap());
    for uuid in watches.keys() {
        let _ = mcp
            .handle_method(
                "watch_ops",
                wrap_action("Delete", Some(serde_json::json!({ "uuid": uuid }))),
            )
            .await;
    }
}

#[tokio::test]
async fn test_live_request_config() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Create a watch
    let watch_url = format!("https://example.com/config-test-{}", Uuid::new_v4());
    let create_params = serde_json::json!({
        "url": watch_url,
        "title": "Config Test"
    });
    let create_result = mcp
        .handle_method("watch_ops", wrap_action("Create", Some(create_params)))
        .await
        .expect("Failed to create watch");

    let uuid = if create_result.is_string() {
        create_result.as_str().unwrap().to_string()
    } else if let Some(u) = create_result.get("uuid").and_then(|v| v.as_str()) {
        u.to_string()
    } else {
        // Search for it to get UUID
        let search_params = serde_json::json!({ "query": watch_url });
        let search_result = mcp
            .handle_method("watch_ops", wrap_action("Search", Some(search_params)))
            .await
            .expect("Failed to search for created watch");
        let watches = search_result
            .get("watches")
            .and_then(|v| v.as_object())
            .unwrap();
        watches
            .keys()
            .next()
            .expect("No watch found after creation")
            .clone()
    };
    println!("Created watch for config test: {}", uuid);

    // 2. Set request config
    let headers = serde_json::json!({
        "X-Test-Header": "test-value"
    });
    let body = "test-body";
    let config_params = serde_json::json!({
        "uuid": uuid,
        "headers": headers,
        "body": body
    });
    mcp.handle_method(
        "watch_ops",
        wrap_action("SetRequestConfig", Some(config_params)),
    )
    .await
    .expect("Failed to set request config");

    // 3. Verify config (using Get)
    let get_params = serde_json::json!({ "uuid": uuid });
    let details = mcp
        .handle_method("watch_ops", wrap_action("Get", Some(get_params)))
        .await
        .expect("Failed to get watch details");

    assert_eq!(details["headers"]["X-Test-Header"], "test-value");
    assert_eq!(details["body"], "test-body");

    // 4. Cleanup
    let delete_params = serde_json::json!({ "uuid": uuid });
    mcp.handle_method("watch_ops", wrap_action("Delete", Some(delete_params)))
        .await
        .expect("Failed to delete watch");
}

#[tokio::test]
async fn test_live_watch_lifecycle() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Create a watch
    let create_params = serde_json::json!({
        "url": "https://example.com/test-live-test",
        "tag": "test-live"
    });
    let create_result = mcp
        .handle_method("watch_ops", wrap_action("Create", Some(create_params)))
        .await
        .expect("Failed to create watch");
    let uuid = create_result
        .get("uuid")
        .expect("No uuid in create result")
        .as_str()
        .expect("uuid not a string")
        .to_string();
    println!("Created watch: {}", uuid);

    // 2. Get details
    let details_params = serde_json::json!({ "uuid": uuid });
    let details = mcp
        .handle_method("watch_ops", wrap_action("Get", Some(details_params)))
        .await
        .expect("Failed to get watch details");
    assert_eq!(
        details.get("url").unwrap().as_str().unwrap(),
        "https://example.com/test-live-test"
    );

    // 3. Trigger check
    let trigger_params = serde_json::json!({ "uuid": uuid });
    mcp.handle_method("watch_ops", wrap_action("Trigger", Some(trigger_params)))
        .await
        .expect("Failed to trigger check");

    // 4. Delete watch
    let delete_params = serde_json::json!({ "uuid": uuid });
    mcp.handle_method("watch_ops", wrap_action("Delete", Some(delete_params)))
        .await
        .expect("Failed to delete watch");
    println!("Deleted watch: {}", uuid);
}

#[tokio::test]
async fn test_live_search_filtering() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Setup: Create a watch with a unique tag
    let unique_tag = format!("search-test-{}", Uuid::new_v4());
    let watch_url = "https://example.com/search-test";
    let watch_title = format!("Search Test Watch {}", Uuid::new_v4());

    let create_params = serde_json::json!({
        "url": watch_url,
        "tag": unique_tag
    });
    let create_result = mcp
        .handle_method("watch_ops", wrap_action("Create", Some(create_params)))
        .await
        .expect("Failed to create watch");
    let uuid = create_result
        .get("uuid")
        .expect("No uuid in create result")
        .as_str()
        .expect("uuid not a string")
        .to_string();

    // Update title so we can search for it (create_watch doesn't support title in current impl)
    let update_params = serde_json::json!({
        "uuid": uuid,
        "title": watch_title
    });
    mcp.handle_method("watch_ops", wrap_action("Update", Some(update_params)))
        .await
        .expect("Failed to update watch title");

    // 2. Test search_watches
    let search_params = serde_json::json!({ "query": watch_title });
    let search_results = mcp
        .handle_method("watch_ops", wrap_action("Search", Some(search_params)))
        .await
        .expect("Failed to search watches");
    assert!(search_results.is_object());
    assert!(
        search_results.as_object().unwrap().contains_key(&uuid),
        "Search result should contain the created watch"
    );

    // 3. Test list_watches with tag filtering
    let list_params = serde_json::json!({ "tag": unique_tag });
    let list_results = mcp
        .handle_method("watch_ops", wrap_action("List", Some(list_params)))
        .await
        .expect("Failed to list watches with tag");
    assert!(list_results.is_object());
    let list_results_obj = list_results
        .get("watches")
        .and_then(|v| v.as_object())
        .unwrap_or_else(|| list_results.as_object().unwrap());
    assert_eq!(
        list_results_obj.len(),
        1,
        "Should find exactly one watch with the unique tag"
    );
    assert!(list_results_obj.contains_key(&uuid));

    // 4. Test search with no results
    let search_params_empty = serde_json::json!({ "query": "NonExistentWatchTitle123456789" });
    let search_results_empty = mcp
        .handle_method(
            "watch_ops",
            wrap_action("Search", Some(search_params_empty)),
        )
        .await
        .expect("Failed to search watches");
    assert!(search_results_empty.is_object());
    assert_eq!(search_results_empty.as_object().unwrap().len(), 0);

    // 5. Cleanup
    let delete_params = serde_json::json!({ "uuid": uuid });
    mcp.handle_method("watch_ops", wrap_action("Delete", Some(delete_params)))
        .await
        .expect("Failed to delete watch");

    // Cleanup tag (optional but good)
    let tags = mcp
        .handle_method("tag_ops", wrap_action("List", None))
        .await
        .expect("Failed to list tags");
    if let Some(tag_uuid) = tags.as_object().and_then(|obj| {
        obj.iter()
            .find(|(_, v)| v.get("title").and_then(|t| t.as_str()) == Some(&unique_tag))
            .map(|(k, _)| k.clone())
    }) {
        let delete_tag_params = serde_json::json!({ "uuid": tag_uuid });
        let _ = mcp
            .handle_method("tag_ops", wrap_action("Delete", Some(delete_tag_params)))
            .await;
    }
}

#[tokio::test]
async fn test_live_list_processors() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    let result = mcp
        .handle_method("system_ops", wrap_action("ListProcessors", None))
        .await
        .expect("Failed to list processors");

    assert!(result.is_array() || result.is_object());
    if let Some(arr) = result.as_array() {
        assert!(!arr.is_empty());
    } else if let Some(obj) = result.as_object() {
        assert!(!obj.is_empty());
    }
    println!("Available processors: {:?}", result);
}

#[tokio::test]
async fn test_live_list_fetchers() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    let result = mcp
        .handle_method("system_ops", wrap_action("ListFetchers", None))
        .await
        .expect("Failed to list fetchers");

    assert!(result.is_array() || result.is_object());
    println!("Available fetchers: {:?}", result);
}

#[tokio::test]
async fn test_live_audit_proxies() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    let result = mcp
        .handle_method("system_ops", wrap_action("AuditProxies", None))
        .await
        .expect("Failed to audit proxies");

    assert!(result.is_object());
    println!("Proxy audit result: {:?}", result);
}

#[tokio::test]
async fn test_live_history_diffs() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Find a watch with at least 2 history points
    let watches_result = mcp
        .handle_method("watch_ops", wrap_action("List", None))
        .await
        .expect("Failed to list watches");
    let watches = watches_result
        .get("watches")
        .and_then(|v| v.as_object())
        .unwrap_or_else(|| watches_result.as_object().unwrap());

    let mut target_uuid = None;
    for (uuid, _) in watches {
        let history_params = serde_json::json!({ "uuid": uuid });
        if let Ok(history) = mcp
            .handle_method(
                "history_ops",
                wrap_action("GetHistory", Some(history_params)),
            )
            .await
        {
            if history
                .as_object()
                .map(|obj| obj.len() >= 2)
                .unwrap_or(false)
            {
                target_uuid = Some(uuid.clone());
                break;
            }
        }
    }

    let uuid =
        target_uuid.expect("Could not find a watch with at least 2 history points for live test");
    println!("Testing history/diff on watch: {}", uuid);

    // 2. Get history
    let history_params = serde_json::json!({ "uuid": uuid });
    let history = mcp
        .handle_method(
            "history_ops",
            wrap_action("GetHistory", Some(history_params)),
        )
        .await
        .expect("Failed to get history");
    assert!(history.is_object());
    assert!(history.as_object().unwrap().len() >= 2);
    println!("History points: {}", history.as_object().unwrap().len());

    // 3. Get diff using "latest" and "previous"
    let diff_params = serde_json::json!({
        "uuid": uuid,
        "from_timestamp": "previous",
        "to_timestamp": "latest"
    });
    let diff = mcp
        .handle_method("history_ops", wrap_action("GetDiff", Some(diff_params)))
        .await
        .expect("Failed to get diff");
    assert!(diff.is_string());
    println!("Diff length (default): {}", diff.as_str().unwrap().len());

    // 4. Get diff with explicit timestamps from history
    let mut timestamps: Vec<String> = history.as_object().unwrap().keys().cloned().collect();
    timestamps.sort();
    let t1 = &timestamps[timestamps.len() - 2];
    let t2 = &timestamps[timestamps.len() - 1];

    let diff_params_explicit = serde_json::json!({
        "uuid": uuid,
        "from_timestamp": t1,
        "to_timestamp": t2
    });
    let diff_explicit = mcp
        .handle_method(
            "history_ops",
            wrap_action("GetDiff", Some(diff_params_explicit)),
        )
        .await
        .expect("Failed to get explicit diff");
    assert!(diff_explicit.is_string());
    assert_eq!(
        diff.as_str().unwrap().len(),
        diff_explicit.as_str().unwrap().len()
    );

    // 5. Get diff with markdown format
    let diff_params_md = serde_json::json!({
        "uuid": uuid,
        "from_timestamp": "previous",
        "to_timestamp": "latest",
        "format": "markdown"
    });
    let diff_md = mcp
        .handle_method("history_ops", wrap_action("GetDiff", Some(diff_params_md)))
        .await
        .expect("Failed to get markdown diff");
    assert!(diff_md.is_string());
    println!(
        "Diff length (markdown): {}",
        diff_md.as_str().unwrap().len()
    );

    // 6. Get diff with advanced parameters
    let diff_params_adv = serde_json::json!({
        "uuid": uuid,
        "from_timestamp": "previous",
        "to_timestamp": "latest",
        "word_diff": "on",
        "changes_only": "on",
        "ignore_whitespace": "on"
    });
    let diff_adv = mcp
        .handle_method("history_ops", wrap_action("GetDiff", Some(diff_params_adv)))
        .await
        .expect("Failed to get advanced diff");
    assert!(diff_adv.is_string());
    println!(
        "Diff length (advanced): {}",
        diff_adv.as_str().unwrap().len()
    );
}

#[tokio::test]
async fn test_live_get_full_spec() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    let result = mcp
        .handle_method("system_ops", wrap_action("GetSpec", None))
        .await
        .expect("Failed to get full spec");

    assert!(result.is_string());
    let spec = result.as_str().unwrap();
    assert!(spec.contains("openapi:"));
    assert!(spec.contains("info:"));
    println!("Live Full Spec length: {}", spec.len());
}

#[tokio::test]
async fn test_live_notification_lifecycle() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Add a notification
    let test_url = format!("mailto://test-{}@example.com", Uuid::new_v4());
    let add_params = serde_json::json!({
        "notification_url": test_url
    });
    let add_result = mcp
        .handle_method("notification_ops", wrap_action("Add", Some(add_params)))
        .await
        .expect("Failed to add notification");

    println!("Created notification result: {:?}", add_result);

    // 2. List notifications
    let list_result = mcp
        .handle_method("notification_ops", wrap_action("List", None))
        .await
        .expect("Failed to list notifications");

    let urls: Vec<String> = serde_json::from_value(
        list_result
            .get("notifications")
            .unwrap_or(&list_result)
            .clone(),
    )
    .unwrap();
    assert!(urls.contains(&test_url));

    // 3. Update notifications (replace all)
    let mut new_urls = urls.clone();
    let updated_url = format!("mailto://test-updated-{}@example.com", Uuid::new_v4());
    // Replace our test url with the updated one
    if let Some(pos) = new_urls.iter().position(|x| x == &test_url) {
        new_urls[pos] = updated_url.clone();
    } else {
        new_urls.push(updated_url.clone());
    }

    let update_params = serde_json::json!({
        "notification_urls": new_urls
    });
    mcp.handle_method(
        "notification_ops",
        wrap_action("Update", Some(update_params)),
    )
    .await
    .expect("Failed to update notifications");

    // Verify update
    let list_result_updated = mcp
        .handle_method("notification_ops", wrap_action("List", None))
        .await
        .expect("Failed to list notifications after update");
    let urls_updated: Vec<String> = serde_json::from_value(
        list_result_updated
            .get("notifications")
            .unwrap_or(&list_result_updated)
            .clone(),
    )
    .unwrap();
    assert!(urls_updated.contains(&updated_url));
    assert!(!urls_updated.contains(&test_url));

    // 4. Delete notification
    let delete_params = serde_json::json!({ "notification_url": updated_url });
    mcp.handle_method(
        "notification_ops",
        wrap_action("Delete", Some(delete_params)),
    )
    .await
    .expect("Failed to delete notification");
    println!("Deleted notification: {}", updated_url);

    // Verify deletion
    let list_result_final = mcp
        .handle_method("notification_ops", wrap_action("List", None))
        .await
        .expect("Failed to list notifications after delete");
    let urls_final: Vec<String> = serde_json::from_value(
        list_result_final
            .get("notifications")
            .unwrap_or(&list_result_final)
            .clone(),
    )
    .unwrap();
    assert!(!urls_final.contains(&updated_url));
}

#[tokio::test]
async fn test_live_snapshot_content() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Find a watch with history
    let watches_result = mcp
        .handle_method("watch_ops", wrap_action("List", None))
        .await
        .expect("Failed to list watches");
    let watches = watches_result
        .get("watches")
        .and_then(|v| v.as_object())
        .unwrap_or_else(|| watches_result.as_object().unwrap());

    let mut target_uuid = None;
    let mut target_timestamp = None;

    for (uuid, _) in watches {
        let history_params = serde_json::json!({ "uuid": uuid });
        if let Ok(history) = mcp
            .handle_method(
                "history_ops",
                wrap_action("GetHistory", Some(history_params)),
            )
            .await
        {
            if let Some(obj) = history.as_object() {
                if !obj.is_empty() {
                    // Get the latest timestamp
                    let mut timestamps: Vec<String> = obj.keys().cloned().collect();
                    timestamps.sort();
                    target_uuid = Some(uuid.clone());
                    target_timestamp = Some(timestamps.last().unwrap().clone());
                    break;
                }
            }
        }
    }

    let uuid = target_uuid.expect("Could not find a watch with history for live test");
    let timestamp = target_timestamp.expect("Could not find a timestamp for live test");
    println!(
        "Testing snapshot content on watch: {} at timestamp: {}",
        uuid, timestamp
    );

    // 2. Get snapshot content
    let content_params = serde_json::json!({
        "uuid": uuid,
        "timestamp": timestamp
    });
    let content = mcp
        .handle_method(
            "history_ops",
            wrap_action("GetContent", Some(content_params)),
        )
        .await
        .expect("Failed to get snapshot content");

    assert!(content.is_string());
    assert!(!content.as_str().unwrap().is_empty());
    println!(
        "Retrieved content length: {}",
        content.as_str().unwrap().len()
    );

    // 3. Get snapshot metadata (GetInfo)
    let info_params = serde_json::json!({
        "uuid": uuid,
        "timestamp": timestamp
    });
    let info = mcp
        .handle_method("history_ops", wrap_action("GetInfo", Some(info_params)))
        .await
        .expect("Failed to get snapshot info");

    assert!(info.is_object());
    assert!(info.get("content_length").is_some());
    assert!(info.get("content_type").is_some());
    println!("Snapshot info: {:?}", info);
}

#[tokio::test]
async fn test_live_import_watches() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    let tag = format!("live-import-test-{}", Uuid::new_v4());
    let params = serde_json::json!({
        "urls": ["https://example.com/live-1", "https://example.com/live-2"],
        "tag": tag
    });

    // 1. Import watches
    let result = mcp
        .handle_method("watch_ops", wrap_action("Import", Some(params)))
        .await
        .expect("Failed to import watches");

    assert!(result.is_array());
    let uuids = result.as_array().unwrap();
    assert_eq!(uuids.len(), 2);
    println!("Imported UUIDs: {:?}", uuids);

    // 2. Cleanup (delete the imported watches)
    for uuid_val in uuids {
        let uuid = uuid_val.as_str().unwrap();
        let delete_params = serde_json::json!({ "uuid": uuid });
        mcp.handle_method("watch_ops", wrap_action("Delete", Some(delete_params)))
            .await
            .expect("Failed to delete watch after import test");
    }
    println!("Cleaned up imported watches.");
}

#[tokio::test]
async fn test_live_state_management() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Get a watch UUID
    let watches_result = mcp
        .handle_method("watch_ops", wrap_action("List", None))
        .await
        .expect("Failed to list watches");
    let uuid = watches_result
        .get("watches")
        .and_then(|v| v.as_object())
        .unwrap_or_else(|| watches_result.as_object().unwrap())
        .keys()
        .next()
        .expect("No watches found for live test")
        .clone();

    println!("Testing state management on watch: {}", uuid);

    // 2. Pause watch
    let params = serde_json::json!({ "uuid": uuid });
    let result = mcp
        .handle_method("watch_ops", wrap_action("Pause", Some(params.clone())))
        .await
        .expect("Failed to pause watch");
    assert_eq!(
        result.get("status").and_then(|v| v.as_str()),
        Some("success")
    );

    // 3. Unpause watch
    let result = mcp
        .handle_method("watch_ops", wrap_action("Unpause", Some(params.clone())))
        .await
        .expect("Failed to unpause watch");
    assert_eq!(
        result.get("status").and_then(|v| v.as_str()),
        Some("success")
    );

    // 4. Mute notifications
    let result = mcp
        .handle_method("watch_ops", wrap_action("Mute", Some(params.clone())))
        .await
        .expect("Failed to mute notifications");
    assert_eq!(
        result.get("status").and_then(|v| v.as_str()),
        Some("success")
    );

    // 5. Unmute notifications
    let result = mcp
        .handle_method("watch_ops", wrap_action("Unmute", Some(params.clone())))
        .await
        .expect("Failed to unmute notifications");
    assert_eq!(
        result.get("status").and_then(|v| v.as_str()),
        Some("success")
    );

    println!("State management tests completed for watch: {}", uuid);
}

#[tokio::test]
async fn test_live_watch_filtering() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Filter by unpaused (most watches should be unpaused)
    let params = serde_json::json!({ "state": "unpaused" });
    let result = mcp
        .handle_method("watch_ops", wrap_action("List", Some(params)))
        .await
        .expect("Failed to list unpaused watches");
    assert!(result.is_object());
    println!(
        "Unpaused watches found: {}",
        result.as_object().unwrap().len()
    );

    // 2. Filter by paused
    let params = serde_json::json!({ "state": "paused" });
    let result = mcp
        .handle_method("watch_ops", wrap_action("List", Some(params)))
        .await
        .expect("Failed to list paused watches");
    assert!(result.is_object());
    println!(
        "Paused watches found: {}",
        result.as_object().unwrap().len()
    );

    // 3. Filter by error
    let params = serde_json::json!({ "state": "error" });
    let result = mcp
        .handle_method("watch_ops", wrap_action("List", Some(params)))
        .await
        .expect("Failed to list watches with errors");
    assert!(result.is_object());
    println!(
        "Watches with errors found: {}",
        result.as_object().unwrap().len()
    );
}

#[tokio::test]
async fn test_live_watch_screenshot() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Find any watch (we'll try to get a screenshot even if it 404s, to verify the tool handles it)
    let watches_result = mcp
        .handle_method("watch_ops", wrap_action("List", None))
        .await
        .expect("Failed to list watches");
    let uuid = watches_result
        .get("watches")
        .and_then(|v| v.as_object())
        .unwrap_or_else(|| watches_result.as_object().unwrap())
        .keys()
        .next()
        .expect("No watches found for live test")
        .clone();

    println!("Testing screenshot on watch: {}", uuid);

    // 2. Get screenshot
    let params = serde_json::json!({ "uuid": uuid });
    let result = mcp
        .handle_method("history_ops", wrap_action("GetScreenshot", Some(params)))
        .await;

    match result {
        Ok(b64) => {
            assert!(b64.is_string());
            println!(
                "Retrieved screenshot (base64 length: {})",
                b64.as_str().unwrap().len()
            );
        }
        Err(e) => {
            // If it's a 404, that's acceptable for a live test if no watches have screenshots
            let msg = e.to_string();
            if msg.contains("404") {
                println!("Screenshot not found (404) for watch {}, which is expected if not using a browser fetcher.", uuid);
            } else {
                panic!("Failed to get watch screenshot: {}", e);
            }
        }
    }
}

#[tokio::test]
async fn test_live_watch_favicon() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Find any watch
    let watches_result = mcp
        .handle_method("watch_ops", wrap_action("List", None))
        .await
        .expect("Failed to list watches");
    let uuid = watches_result
        .get("watches")
        .and_then(|v| v.as_object())
        .unwrap_or_else(|| watches_result.as_object().unwrap())
        .keys()
        .next()
        .expect("No watches found for live test")
        .clone();

    println!("Testing favicon on watch: {}", uuid);

    // 2. Get favicon
    let params = serde_json::json!({ "uuid": uuid });
    let result = mcp
        .handle_method("watch_ops", wrap_action("GetFavicon", Some(params)))
        .await;

    match result {
        Ok(b64) => {
            assert!(b64.is_string());
            println!(
                "Retrieved favicon (base64 length: {})",
                b64.as_str().unwrap().len()
            );
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("404") {
                println!(
                    "Favicon not found (404) for watch {}, which is common.",
                    uuid
                );
            } else {
                panic!("Failed to get watch favicon: {}", e);
            }
        }
    }
}

#[tokio::test]
async fn test_live_advanced_filtering() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Test find_watches_by_error
    let error_result = mcp
        .handle_method("watch_ops", wrap_action("ListErrors", None))
        .await
        .expect("Failed to find watches by error");
    assert!(error_result.is_object());
    println!(
        "Watches with errors (live): {}",
        error_result.as_object().unwrap().len()
    );

    // 2. Test list_watches_by_processor
    let params = serde_json::json!({
        "processor": "text_json_diff"
    });
    let processor_result = mcp
        .handle_method("watch_ops", wrap_action("ListByProcessor", Some(params)))
        .await
        .expect("Failed to list watches by processor");
    assert!(processor_result.is_object());
    println!(
        "Watches with text_json_diff (live): {}",
        processor_result.as_object().unwrap().len()
    );
}

#[tokio::test]
async fn test_live_maintenance() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Test Backup
    println!("Testing maintenance_ops Backup...");
    let backup_result = mcp
        .handle_method("maintenance_ops", wrap_action("Backup", None))
        .await;

    match backup_result {
        Ok(result) => {
            println!("Backup result: {:?}", result);
            assert!(result.get("status").is_some());
        }
        Err(e) => {
            // If the endpoint doesn't exist on the server, it might 404
            println!("Backup failed (this is expected if /api/v1/backup is not supported by your version): {}", e);
        }
    }

    // 2. Test Export
    println!("Testing maintenance_ops Export...");
    let export_result = mcp
        .handle_method("maintenance_ops", wrap_action("Export", None))
        .await
        .expect("Failed to export watches");

    assert!(export_result.get("watches").is_some());
    let watches = export_result.get("watches").unwrap().as_object().unwrap();
    println!("Exported {} watches", watches.len());

    if !watches.is_empty() {
        let first_uuid = watches.keys().next().unwrap();
        let first_watch = watches.get(first_uuid).unwrap();
        assert!(first_watch.get("url").is_some());
    }
}

#[tokio::test]
async fn test_live_resources() {
    if std::env::var("RUN_LIVE_TESTS").is_err() {
        return;
    }

    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. List resources
    let list_result = mcp
        .handle_method("resources/list", None)
        .await
        .expect("Failed to list resources");
    let resources = list_result.get("resources").unwrap().as_array().unwrap();
    assert!(resources
        .iter()
        .any(|r| r["uri"] == "system://openapi-spec"));
    println!("Live Resources List: {:?}", resources);

    // 2. Read system spec
    let read_spec_params = serde_json::json!({ "uri": "system://openapi-spec" });
    let spec_result = mcp
        .handle_method("resources/read", Some(read_spec_params))
        .await
        .expect("Failed to read system spec");
    let spec_contents = spec_result.get("contents").unwrap().as_array().unwrap();
    assert!(spec_contents[0]["text"]
        .as_str()
        .unwrap()
        .contains("openapi:"));
    println!("Live Resource Read (system spec) Success");

    // 3. Read latest watch snapshot (if any watches exist)
    let watches_result = mcp
        .handle_method("watch_ops", wrap_action("List", None))
        .await
        .expect("Failed to list watches");

    // Check if it's the new consolidated format or old
    let watches = if let Some(w) = watches_result.get("watches") {
        w.as_object().unwrap()
    } else {
        watches_result.as_object().unwrap()
    };

    if let Some(uuid) = watches.keys().next() {
        let uri = format!("watches://{}/latest", uuid);
        let read_watch_params = serde_json::json!({ "uri": uri });
        let watch_result = mcp
            .handle_method("resources/read", Some(read_watch_params))
            .await;

        match watch_result {
            Ok(res) => {
                let contents = res.get("contents").unwrap().as_array().unwrap();
                assert_eq!(contents[0]["uri"], uri);
                println!("Live resource read success for {}", uri);
            }
            Err(e) => {
                println!(
                    "Live resource read failed for {} (expected if no history): {}",
                    uri, e
                );
            }
        }
    }
}
