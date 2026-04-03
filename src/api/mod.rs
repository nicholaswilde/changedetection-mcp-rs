use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
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
    pub paused: Option<bool>,
    pub last_error: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub watch_count: usize,
    pub queue_size: usize,
    pub overdue_watches: Vec<String>,
    pub uptime: f64,
    pub version: String,
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

    pub async fn get_system_info(&self) -> Result<SystemInfo, ApiError> {
        let url = format!("{}/api/v1/systeminfo", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let info = response.json::<SystemInfo>().await?;
        Ok(info)
    }

    pub async fn list_watches(
        &self,
        tag: Option<&str>,
    ) -> Result<HashMap<String, Watch>, ApiError> {
        let mut url = format!("{}/api/v1/watch", self.base_url);
        if let Some(tag) = tag {
            url.push_str(&format!("?tag={}", tag));
        }
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let watches = response.json::<HashMap<String, Watch>>().await?;
        Ok(watches)
    }

    pub async fn search_watches(&self, query: &str) -> Result<HashMap<String, Watch>, ApiError> {
        let url = format!("{}/api/v1/search?q={}&partial=1", self.base_url, query);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let watches = response.json::<HashMap<String, Watch>>().await?;
        Ok(watches)
    }

    pub async fn get_watch_details(&self, uuid: &str) -> Result<Watch, ApiError> {
        let url = format!("{}/api/v1/watch/{}", self.base_url, uuid);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
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

        let response = self
            .http_client
            .post(&endpoint)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;
        let result = response.json::<HashMap<String, String>>().await?;
        Ok(result)
    }

    pub async fn update_watch(
        &self,
        uuid: &str,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/watch/{}", self.base_url, uuid);
        let response = self
            .http_client
            .put(&url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        let text = response.text().await?;
        if text.trim().is_empty() {
            return Ok(serde_json::json!({"status": "success"}));
        }

        let result =
            serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({"status": text}));

        Ok(result)
    }

    pub async fn delete_watch(&self, uuid: &str) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/watch/{}", self.base_url, uuid);
        let response = self
            .http_client
            .delete(&url)
            .send()
            .await?
            .error_for_status()?;

        let text = response.text().await?;
        if text.trim().is_empty() {
            return Ok(serde_json::json!({"status": "success"}));
        }

        let result =
            serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({"status": text}));
        Ok(result)
    }

    pub async fn trigger_check(&self, uuid: &str) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/watch/{}?recheck=1", self.base_url, uuid);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let text = response.text().await?;
        if text.trim().is_empty() {
            return Ok(serde_json::json!({"status": "success"}));
        }

        let result =
            serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({"status": text}));
        Ok(result)
    }

    pub async fn get_watch_history(&self, uuid: &str) -> Result<HashMap<String, String>, ApiError> {
        let url = format!("{}/api/v1/watch/{}/history", self.base_url, uuid);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let history = response.json::<HashMap<String, String>>().await?;
        Ok(history)
    }

    pub async fn get_watch_diff(
        &self,
        uuid: &str,
        from: &str,
        to: &str,
        format_type: Option<&str>,
    ) -> Result<String, ApiError> {
        let mut url = format!(
            "{}/api/v1/watch/{}/difference/{}/{}",
            self.base_url, uuid, from, to
        );
        if let Some(fmt) = format_type {
            url.push_str(&format!("?format={}", fmt));
        }
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let diff = response.text().await?;
        Ok(diff)
    }

    pub async fn list_tags(&self) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/tags", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let text = response.text().await?;
        let tags = serde_json::from_str(&text)?;
        Ok(tags)
    }

    pub async fn create_tag(&self, title: &str) -> Result<String, ApiError> {
        let url = format!("{}/api/v1/tag", self.base_url);
        let mut body = HashMap::new();
        body.insert("title", title.to_string());

        let response = self
            .http_client
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;
        let text = response.text().await?;

        // The API might return a UUID string or a JSON object with a uuid field
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(uuid) = val.get("uuid").and_then(|v| v.as_str()) {
                return Ok(uuid.to_string());
            }
        }

        let result = text.trim_matches('"').trim().to_string();
        Ok(result)
    }

    pub async fn get_tag_details(&self, uuid: &str) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/tag/{}", self.base_url, uuid);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let text = response.text().await?;
        let tag = serde_json::from_str(&text)?;
        Ok(tag)
    }

    pub async fn update_tag(
        &self,
        uuid: &str,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/tag/{}", self.base_url, uuid);
        let response = self
            .http_client
            .put(&url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        let text = response.text().await?;
        if text.trim().is_empty() {
            return Ok(serde_json::json!({"status": "success"}));
        }
        let result =
            serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({"status": text}));
        Ok(result)
    }

    pub async fn delete_tag(&self, uuid: &str) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/tag/{}", self.base_url, uuid);
        let response = self
            .http_client
            .delete(&url)
            .send()
            .await?
            .error_for_status()?;
        let text = response.text().await?;
        if text.trim().is_empty() {
            return Ok(serde_json::json!({"status": "success"}));
        }
        let result =
            serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({"status": text}));
        Ok(result)
    }

    pub async fn get_full_spec(&self) -> Result<String, ApiError> {
        let url = format!("{}/api/v1/full-spec", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let spec = response.text().await?;
        Ok(spec)
    }

    pub async fn list_notifications(&self) -> Result<Vec<String>, ApiError> {
        let url = format!("{}/api/v1/notifications", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let res = response.json::<serde_json::Value>().await?;
        let urls = res
            .get("notification_urls")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        Ok(urls)
    }

    pub async fn add_notification(
        &self,
        notification_url: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/notifications", self.base_url);
        let mut body = HashMap::new();
        body.insert("notification_urls", vec![notification_url.to_string()]);

        let response = self
            .http_client
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;
        let result = response.json::<serde_json::Value>().await?;
        Ok(result)
    }

    pub async fn update_notifications(
        &self,
        notification_urls: Vec<String>,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/notifications", self.base_url);
        let mut body = HashMap::new();
        body.insert("notification_urls", notification_urls);

        let response = self
            .http_client
            .put(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let result = response.json::<serde_json::Value>().await?;
        Ok(result)
    }

    pub async fn delete_notification(
        &self,
        notification_url: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/notifications", self.base_url);
        let mut body = HashMap::new();
        body.insert("notification_urls", vec![notification_url.to_string()]);

        let response = self
            .http_client
            .delete(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        if response.status() == StatusCode::NO_CONTENT {
            return Ok(serde_json::json!({"status": "success"}));
        }

        let text = response.text().await?;
        if text.trim().is_empty() {
            return Ok(serde_json::json!({"status": "success"}));
        }
        let result =
            serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({"status": text}));
        Ok(result)
    }

    pub async fn get_snapshot_content(
        &self,
        uuid: &str,
        timestamp: &str,
    ) -> Result<String, ApiError> {
        let url = format!(
            "{}/api/v1/watch/{}/history/{}",
            self.base_url, uuid, timestamp
        );
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let content = response.text().await?;
        Ok(content)
    }

    pub async fn import_watches(
        &self,
        urls: Vec<String>,
        tag: Option<&str>,
    ) -> Result<Vec<String>, ApiError> {
        let mut url = format!("{}/api/v1/import", self.base_url);
        if let Some(tag) = tag {
            url.push_str(&format!("?tag={}", tag));
        }
        let body = urls.join("\n");

        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "text/plain")
            .body(body)
            .send()
            .await?
            .error_for_status()?;
        let uuids = response.json::<Vec<String>>().await?;
        Ok(uuids)
    }

    pub async fn set_watch_state(
        &self,
        uuid: &str,
        key: &str,
        value: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/watch/{}?{}={}", self.base_url, uuid, key, value);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let text = response.text().await?;
        let trimmed = text.trim().trim_matches('"');
        if trimmed.is_empty() || trimmed == "OK" {
            return Ok(serde_json::json!({"status": "success"}));
        }

        let result =
            serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({"status": text}));

        Ok(result)
    }

    pub async fn get_watch_screenshot(&self, uuid: &str) -> Result<Vec<u8>, ApiError> {
        let url = format!("{}/api/v1/watch/{}/screenshot", self.base_url, uuid);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}
