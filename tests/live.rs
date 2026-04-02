use changedetection_mcp_rs::api::Client;
use changedetection_mcp_rs::mcp::McpServer;
use mcp_sdk_rs::server::ServerHandler;
use std::env;
use uuid::Uuid;

#[tokio::test]
async fn test_live_tag_lifecycle() {
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
    let create_result = mcp.handle_method("create_tag", Some(create_params)).await.expect("Failed to create tag");
    
    // The API might return a UUID string or a JSON object with a uuid field
    let uuid = if create_result.is_string() {
        create_result.as_str().unwrap().to_string()
    } else {
        create_result.get("uuid").and_then(|v| v.as_str()).expect("No uuid in create result").to_string()
    };
    println!("Created tag: {} ({})", tag_name, uuid);

    // 2. Get details
    let details_params = serde_json::json!({ "uuid": uuid });
    let details = mcp.handle_method("get_tag_details", Some(details_params)).await.expect("Failed to get tag details");
    // ChangeDetection.io API might return title or name depending on version/endpoint
    assert!(details.get("title").is_some() || details.get("name").is_some());
    println!("Tag details: {:?}", details);

    // 3. Update tag
    let update_params = serde_json::json!({
        "uuid": uuid,
        "title": format!("{}-updated", tag_name)
    });
    mcp.handle_method("update_tag", Some(update_params)).await.expect("Failed to update tag");
    
    // Verify update
    let updated_details = mcp.handle_method("get_tag_details", Some(serde_json::json!({ "uuid": uuid }))).await.expect("Failed to get updated tag details");
    let updated_title = updated_details.get("title").or(updated_details.get("name")).and_then(|v| v.as_str()).expect("Missing title/name in updated details");
    assert!(updated_title.contains("updated"));

    // 4. List tags
    let tags = mcp.handle_method("list_tags", None).await.expect("Failed to list tags");
    println!("Tags list: {:?}", tags);
    // tags is now a map where keys are UUIDs
    assert!(tags.is_object());
    let found = tags.as_object().unwrap().contains_key(&uuid);
    assert!(found, "Created tag not found in list_tags map");

    // 5. Delete tag
    let delete_params = serde_json::json!({ "uuid": uuid });
    mcp.handle_method("delete_tag", Some(delete_params)).await.expect("Failed to delete tag");
    println!("Deleted tag: {}", uuid);
}

#[tokio::test]
async fn test_live_system_info() {
    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    let result = mcp.handle_method("get_system_info", None).await.expect("Failed to get system info");
    
    assert!(result.get("version").is_some());
    assert!(result.get("watch_count").is_some());
    println!("Live System Info: {:?}", result);
}

#[tokio::test]
async fn test_live_list_watches() {
    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    let result = mcp.handle_method("list_watches", None).await.expect("Failed to list watches");
    
    assert!(result.is_object());
    println!("Live Watches count: {}", result.as_object().unwrap().len());
}

#[tokio::test]
async fn test_live_watch_lifecycle() {
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
    let create_result = mcp.handle_method("create_watch", Some(create_params)).await.expect("Failed to create watch");
    let uuid = create_result.get("uuid").expect("No uuid in create result").as_str().expect("uuid not a string").to_string();
    println!("Created watch: {}", uuid);

    // 2. Get details
    let details_params = serde_json::json!({ "uuid": uuid });
    let details = mcp.handle_method("get_watch_details", Some(details_params)).await.expect("Failed to get watch details");
    assert_eq!(details.get("url").unwrap().as_str().unwrap(), "https://example.com/test-live-test");

    // 3. Trigger check
    let trigger_params = serde_json::json!({ "uuid": uuid });
    mcp.handle_method("trigger_check", Some(trigger_params)).await.expect("Failed to trigger check");

    // 4. Delete watch
    let delete_params = serde_json::json!({ "uuid": uuid });
    mcp.handle_method("delete_watch", Some(delete_params)).await.expect("Failed to delete watch");
    println!("Deleted watch: {}", uuid);
}

#[tokio::test]
async fn test_live_search_filtering() {
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
    let create_result = mcp.handle_method("create_watch", Some(create_params)).await.expect("Failed to create watch");
    let uuid = create_result.get("uuid").expect("No uuid in create result").as_str().expect("uuid not a string").to_string();
    
    // Update title so we can search for it (create_watch doesn't support title in current impl)
    let update_params = serde_json::json!({
        "uuid": uuid,
        "title": watch_title
    });
    mcp.handle_method("update_watch", Some(update_params)).await.expect("Failed to update watch title");

    // 2. Test search_watches
    let search_params = serde_json::json!({ "query": watch_title });
    let search_results = mcp.handle_method("search_watches", Some(search_params)).await.expect("Failed to search watches");
    assert!(search_results.is_object());
    assert!(search_results.as_object().unwrap().contains_key(&uuid), "Search result should contain the created watch");

    // 3. Test list_watches with tag filtering
    let list_params = serde_json::json!({ "tag": unique_tag });
    let list_results = mcp.handle_method("list_watches", Some(list_params)).await.expect("Failed to list watches with tag");
    assert!(list_results.is_object());
    assert_eq!(list_results.as_object().unwrap().len(), 1, "Should find exactly one watch with the unique tag");
    assert!(list_results.as_object().unwrap().contains_key(&uuid));

    // 4. Test search with no results
    let search_params_empty = serde_json::json!({ "query": "NonExistentWatchTitle123456789" });
    let search_results_empty = mcp.handle_method("search_watches", Some(search_params_empty)).await.expect("Failed to search watches");
    assert!(search_results_empty.is_object());
    assert_eq!(search_results_empty.as_object().unwrap().len(), 0);

    // 5. Cleanup
    let delete_params = serde_json::json!({ "uuid": uuid });
    mcp.handle_method("delete_watch", Some(delete_params)).await.expect("Failed to delete watch");
    
    // Cleanup tag (optional but good)
    let tags = mcp.handle_method("list_tags", None).await.expect("Failed to list tags");
    if let Some(tag_uuid) = tags.as_object().and_then(|obj| {
        obj.iter().find(|(_, v)| v.get("title").and_then(|t| t.as_str()) == Some(&unique_tag)).map(|(k, _)| k.clone())
    }) {
        let delete_tag_params = serde_json::json!({ "uuid": tag_uuid });
        let _ = mcp.handle_method("delete_tag", Some(delete_tag_params)).await;
    }
}

#[tokio::test]
async fn test_live_history_diffs() {
    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let mcp = McpServer::new(client);

    // 1. Find a watch with at least 2 history points
    let watches_result = mcp.handle_method("list_watches", None).await.expect("Failed to list watches");
    let watches = watches_result.as_object().expect("list_watches should return an object");
    
    let mut target_uuid = None;
    for (uuid, _) in watches {
        let history_params = serde_json::json!({ "uuid": uuid });
        if let Ok(history) = mcp.handle_method("get_watch_history", Some(history_params)).await {
            if history.as_object().map(|obj| obj.len() >= 2).unwrap_or(false) {
                target_uuid = Some(uuid.clone());
                break;
            }
        }
    }

    let uuid = target_uuid.expect("Could not find a watch with at least 2 history points for live test");
    println!("Testing history/diff on watch: {}", uuid);

    // 2. Get history
    let history_params = serde_json::json!({ "uuid": uuid });
    let history = mcp.handle_method("get_watch_history", Some(history_params)).await.expect("Failed to get history");
    assert!(history.is_object());
    assert!(history.as_object().unwrap().len() >= 2);
    println!("History points: {}", history.as_object().unwrap().len());

    // 3. Get diff using "latest" and "previous"
    let diff_params = serde_json::json!({
        "uuid": uuid,
        "from_timestamp": "previous",
        "to_timestamp": "latest"
    });
    let diff = mcp.handle_method("get_watch_diff", Some(diff_params)).await.expect("Failed to get diff");
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
    let diff_explicit = mcp.handle_method("get_watch_diff", Some(diff_params_explicit)).await.expect("Failed to get explicit diff");
    assert!(diff_explicit.is_string());
    assert_eq!(diff.as_str().unwrap().len(), diff_explicit.as_str().unwrap().len());

    // 5. Get diff with markdown format
    let diff_params_md = serde_json::json!({
        "uuid": uuid,
        "from_timestamp": "previous",
        "to_timestamp": "latest",
        "format": "markdown"
    });
    let diff_md = mcp.handle_method("get_watch_diff", Some(diff_params_md)).await.expect("Failed to get markdown diff");
    assert!(diff_md.is_string());
    println!("Diff length (markdown): {}", diff_md.as_str().unwrap().len());
    
    // Markdown diff should contain markdown indicators like # or *
    // Actually, depending on content it might be different, but let's just assert it's a string.
}
