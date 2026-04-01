use changedetection_mcp_rs::api::Client;
use changedetection_mcp_rs::cli::Args;
use changedetection_mcp_rs::mcp::McpServer;
use changedetection_mcp_rs::observability::init_tracing;
use clap::Parser;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let _guard = init_tracing(
        &args.log_level,
        args.log_file.as_deref(),
        args.log_format == changedetection_mcp_rs::cli::LogFormat::Json,
    );

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
