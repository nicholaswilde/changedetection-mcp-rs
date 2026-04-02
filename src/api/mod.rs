use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Middleware error: {0}")]
    Middleware(#[from] reqwest_middleware::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid URL: {0}")]
    Url(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Watch {
    pub url: String,
    pub title: Option<String>,
}

#[derive(Clone)]
pub struct Client {
    base_url: String,
    http_client: ClientWithMiddleware,
}

impl Client {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self::new_with_timeout(base_url, api_key, Duration::from_secs(10))
    }

    pub fn new_with_timeout(base_url: String, api_key: String, timeout: Duration) -> Self {
        let mut headers = HeaderMap::new();
        if let Ok(val) = HeaderValue::from_str(&api_key) {
            headers.insert("x-api-key", val);
        }

        let reqwest_client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(timeout)
            .build()
            .expect("Failed to build HTTP client");

        // Retry strategy: exponential backoff with 3 retries
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);

        let http_client = ClientBuilder::new(reqwest_client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::new("/tmp/changedetection-mcp-cache".into(), true),
                options: HttpCacheOptions::default(),
            }))
            .build();

        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http_client,
        }
    }

    pub async fn list_watches(&self, tag: Option<&str>) -> Result<HashMap<String, Watch>, ApiError> {
        let mut url = format!("{}/api/v1/watch", self.base_url);
        if let Some(tag) = tag {
            url.push_str(&format!("?tag={}", tag));
        }
        let response = self.http_client.get(&url).send().await?.error_for_status()?;
        let watches = response.json::<HashMap<String, Watch>>().await?;
        Ok(watches)
    }

    pub async fn get_watch_details(&self, uuid: &str) -> Result<Watch, ApiError> {
        let url = format!("{}/api/v1/watch/{}", self.base_url, uuid);
        let response = self.http_client.get(&url).send().await?.error_for_status()?;
        let watch = response.json::<Watch>().await?;
        Ok(watch)
    }

    pub async fn create_watch(
        &self,
        url: &str,
        tag: Option<&str>,
    ) -> Result<HashMap<String, String>, ApiError> {
        let endpoint = format!("{}/api/v1/watch", self.base_url);
        let mut body = HashMap::new();
        body.insert("url", url.to_string());
        if let Some(tag) = tag {
            body.insert("tag", tag.to_string());
        }

        let response = self.http_client.post(&endpoint).json(&body).send().await?.error_for_status()?;
        let result = response.json::<HashMap<String, String>>().await?;
        Ok(result)
    }

    pub async fn delete_watch(&self, uuid: &str) -> Result<HashMap<String, String>, ApiError> {
        let url = format!("{}/api/v1/watch/{}", self.base_url, uuid);
        let response = self.http_client.delete(&url).send().await?.error_for_status()?;
        let result = response.json::<HashMap<String, String>>().await?;
        Ok(result)
    }

    pub async fn update_watch(
        &self,
        uuid: &str,
        payload: serde_json::Value,
    ) -> Result<HashMap<String, String>, ApiError> {
        let url = format!("{}/api/v1/watch/{}", self.base_url, uuid);
        let response = self
            .http_client
            .put(&url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        
        // ChangeDetection.io PUT might return an empty body or success message.
        // We attempt to decode it, but return an empty map if it's empty or invalid JSON.
        let text = response.text().await?;
        if text.trim().is_empty() {
            return Ok(HashMap::new());
        }
        
        let result = serde_json::from_str(&text).unwrap_or_else(|_| {
            let mut map = HashMap::new();
            map.insert("status".to_string(), "success".to_string());
            map
        });
        
        Ok(result)
    }

    pub async fn trigger_check(&self, uuid: &str) -> Result<HashMap<String, String>, ApiError> {
        let url = format!("{}/api/v1/watch/{}/recheck", self.base_url, uuid);
        let response = self.http_client.get(&url).send().await?.error_for_status()?;
        let result = response.json::<HashMap<String, String>>().await?;
        Ok(result)
    }

    pub async fn get_watch_history(&self, uuid: &str) -> Result<HashMap<String, String>, ApiError> {
        let url = format!("{}/api/v1/watch/{}/history", self.base_url, uuid);
        let response = self.http_client.get(&url).send().await?.error_for_status()?;
        let history = response.json::<HashMap<String, String>>().await?;
        Ok(history)
    }

    pub async fn get_watch_diff(&self, uuid: &str, from: &str, to: &str) -> Result<String, ApiError> {
        let url = format!(
            "{}/api/v1/watch/{}/difference/{}/{}",
            self.base_url, uuid, from, to
        );
        let response = self.http_client.get(&url).send().await?.error_for_status()?;
        let diff = response.text().await?;
        Ok(diff)
    }
}
