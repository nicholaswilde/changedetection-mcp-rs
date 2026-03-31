use changedetection_mcp_rs::api::Client;
use changedetection_mcp_rs::mcp::McpServer;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");
    let base_url = env::var("CHANGEDETECTION_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:5000".to_string());

    let client = Client::new(base_url, api_key);
    let server = McpServer::new(client);

    server.run().await?;

    Ok(())
}
