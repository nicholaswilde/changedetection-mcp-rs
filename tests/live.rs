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
