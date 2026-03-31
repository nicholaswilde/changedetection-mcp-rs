use changedetection_mcp_rs::api::Client;
use changedetection_mcp_rs::mcp::McpServer;
use std::env;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");
    let base_url = env::var("CHANGEDETECTION_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:5000".to_string());

    let client = Client::new(base_url, api_key);
    let server = McpServer::new(client);

    server.run().await?;

    Ok(())
}
