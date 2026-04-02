use changedetection_mcp_rs::api::Client;
use changedetection_mcp_rs::mcp::McpServer;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub struct MockApp {
    pub server: MockServer,
    pub client: Client,
    pub mcp: McpServer,
}

impl MockApp {
    pub async fn new() -> Self {
        Self::new_with_timeout(std::time::Duration::from_secs(10)).await
    }

    pub async fn new_with_timeout(timeout: std::time::Duration) -> Self {
        let server = MockServer::start().await;
        let client = Client::new_with_timeout(server.uri(), "test_api_key".to_string(), timeout);
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

    pub async fn mock_get_text(&self, path_str: &str, status: u16, body: &str) {
        Mock::given(method("GET"))
            .and(path(path_str))
            .respond_with(ResponseTemplate::new(status).set_body_string(body))
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
