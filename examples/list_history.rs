use changedetection_mcp_rs::api::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let base_url = env::var("CHANGEDETECTION_BASE_URL").expect("CHANGEDETECTION_BASE_URL not set");
    let api_key = env::var("CHANGEDETECTION_API_KEY").expect("CHANGEDETECTION_API_KEY not set");

    let client = Client::new(base_url, api_key);
    let watches = client
        .list_watches(None)
        .await
        .expect("Failed to list watches");

    for (uuid, watch) in watches {
        let history = client
            .get_watch_history(&uuid)
            .await
            .expect("Failed to get history");
        println!(
            "Watch {}: {} - history count: {}",
            uuid,
            watch.url,
            history.len()
        );
    }
}
