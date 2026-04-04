use changedetection_mcp_rs::api::Client;
use changedetection_mcp_rs::mcp::McpServer;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[allow(dead_code)]
pub struct MockApp {
    pub server: MockServer,
    pub client: Client,
    pub mcp: McpServer,
}

#[allow(dead_code)]
impl MockApp {
    pub async fn new() -> Self {
        Self::new_with_timeout(std::time::Duration::from_secs(10)).await
    }

    pub async fn new_with_timeout(timeout: std::time::Duration) -> Self {
        let server = MockServer::start().await;
        // Use a unique cache directory for each test instance to avoid cross-test interference
        let cache_dir = format!("/tmp/changedetection-mcp-test-{}", uuid::Uuid::new_v4());
        let client = changedetection_mcp_rs::api::Client::new_full(
            server.uri(),
            "test_api_key".to_string(),
            timeout,
            Some(cache_dir),
        );
        let mcp = McpServer::new(client.clone());

        Self {
            server,
            client,
            mcp,
        }
    }

    pub async fn mock_get(&self, path_str: &str, status: u16, body: Option<serde_json::Value>) {
        let mut response = ResponseTemplate::new(status);
        if let Some(b) = body {
            response = response.set_body_json(b);
        }
        Mock::given(method("GET"))
            .and(path(path_str))
            .respond_with(response)
            .mount(&self.server)
            .await;
    }

    pub async fn mock_get_with_query(
        &self,
        path_str: &str,
        query_key: &str,
        query_val: &str,
        status: u16,
        body: Option<serde_json::Value>,
    ) {
        let mut response = ResponseTemplate::new(status);
        if let Some(b) = body {
            response = response.set_body_json(b);
        }
        Mock::given(method("GET"))
            .and(path(path_str))
            .and(query_param(query_key, query_val))
            .respond_with(response)
            .mount(&self.server)
            .await;
    }

    pub async fn mock_get_text(&self, path_str: &str, status: u16, body: &str) {
        Mock::given(method("GET"))
            .and(path(path_str))
            .respond_with(ResponseTemplate::new(status).set_body_string(body))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_get_binary(&self, path_str: &str, status: u16, body: Vec<u8>) {
        Mock::given(method("GET"))
            .and(path(path_str))
            .respond_with(ResponseTemplate::new(status).set_body_raw(body, "image/png"))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_post(&self, path_str: &str, status: u16, body: Option<serde_json::Value>) {
        let mut response = ResponseTemplate::new(status);
        if let Some(b) = body {
            response = response.set_body_json(b);
        }
        Mock::given(method("POST"))
            .and(path(path_str))
            .respond_with(response)
            .mount(&self.server)
            .await;
    }

    pub async fn mock_post_with_query(
        &self,
        path_str: &str,
        query_key: &str,
        query_val: &str,
        status: u16,
        body: Option<serde_json::Value>,
    ) {
        let mut response = ResponseTemplate::new(status);
        if let Some(b) = body {
            response = response.set_body_json(b);
        }
        Mock::given(method("POST"))
            .and(path(path_str))
            .and(query_param(query_key, query_val))
            .respond_with(response)
            .mount(&self.server)
            .await;
    }

    pub async fn mock_post_text(
        &self,
        path_str: &str,
        status: u16,
        body: Option<serde_json::Value>,
    ) {
        let mut response = ResponseTemplate::new(status);
        if let Some(b) = body {
            response = response.set_body_json(b);
        }
        Mock::given(method("POST"))
            .and(path(path_str))
            .respond_with(response)
            .mount(&self.server)
            .await;
    }

    pub async fn mock_delete(&self, path_str: &str, status: u16, body: Option<serde_json::Value>) {
        let mut response = ResponseTemplate::new(status);
        if let Some(b) = body {
            response = response.set_body_json(b);
        }
        Mock::given(method("DELETE"))
            .and(path(path_str))
            .respond_with(response)
            .mount(&self.server)
            .await;
    }

    pub async fn mock_put(&self, path_str: &str, status: u16, body: Option<serde_json::Value>) {
        let mut response = ResponseTemplate::new(status);
        if let Some(b) = body {
            response = response.set_body_json(b);
        }
        Mock::given(method("PUT"))
            .and(path(path_str))
            .respond_with(response)
            .mount(&self.server)
            .await;
    }
}
