use changedetection_mcp_rs::api::Client;
use changedetection_mcp_rs::mcp::McpServer;
use mcp_sdk_rs::server::ServerHandler;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mcp_list_watches() {
    let mock_server = MockServer::start().await;
    let client = Client::new(mock_server.uri(), "test_api_key".to_string());
    let server = McpServer::new(client);

    let response_body = json!({
        "watch_id_1": {
            "url": "https://example.com",
            "title": "Example"
        }
    });

    Mock::given(method("GET"))
        .and(path("/api/v1/watch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let result = server.handle_method("list_watches", None).await.unwrap();
    
    let watches: serde_json::Value = serde_json::from_value(result).unwrap();
    assert!(watches.get("watch_id_1").is_some());
    assert_eq!(watches["watch_id_1"]["url"], "https://example.com");
}

#[tokio::test]
async fn test_mcp_tools_list() {
    let mock_server = MockServer::start().await;
    let client = Client::new(mock_server.uri(), "test_api_key".to_string());
    let server = McpServer::new(client);

    let result = server.handle_method("tools/list", None).await.unwrap();
    
    let tools = result.get("tools").unwrap().as_array().unwrap();
    assert_eq!(tools.len(), 5);
    
    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&"list_watches"));
    assert!(tool_names.contains(&"get_watch_details"));
    assert!(tool_names.contains(&"create_watch"));
    assert!(tool_names.contains(&"delete_watch"));
    assert!(tool_names.contains(&"trigger_check"));
}
