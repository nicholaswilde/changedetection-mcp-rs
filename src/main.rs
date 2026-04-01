use changedetection_mcp_rs::api::Client;
use changedetection_mcp_rs::cli::Args;
use changedetection_mcp_rs::mcp::McpServer;
use clap::Parser;
use std::env;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(
            EnvFilter::from_default_env()
                .add_directive(args.log_level.parse().unwrap_or(tracing::Level::INFO).into()),
        )
        .init();

    tracing::debug!("Arguments parsed: {:?}", args);

    let api_key = args
        .api_key
        .or_else(|| env::var("CHANGEDETECTION_API_KEY").ok())
        .expect("CHANGEDETECTION_API_KEY not set (via --api-key or env)");

    let base_url = env::var("CHANGEDETECTION_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:5000".to_string());

    let client = Client::new(base_url, api_key);
    let server = McpServer::new(client);

    server.run(args.transport, &args.host, args.port).await?;

    Ok(())
}
