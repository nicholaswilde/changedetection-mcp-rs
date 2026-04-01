use changedetection_mcp_rs::api::Client;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");
    let base_url = env::var("CHANGEDETECTION_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:5000".to_string());

    let client = Client::new(base_url, api_key);

    println!("Listing watches...");
    let watches = client.list_watches(None).await?;
    println!("Found {} watches.", watches.len());
    for (uuid, watch) in watches {
        let title = watch.title.unwrap_or_default();
        println!("- {}: {} ({})", uuid, title, watch.url);
    }

    Ok(())
}
