use changedetection_mcp_rs::api::Client;
use std::env;
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").unwrap();
    let api_key = env::var("CHANGEDETECTION_API_KEY").unwrap();
    let client = Client::new(base_url, api_key);
    let watches = client.list_watches(None).await.unwrap();
    let uuid = watches.keys().next().unwrap();
    let history = client.get_watch_history(uuid).await.unwrap();
    println!("{:?}", history);
}
