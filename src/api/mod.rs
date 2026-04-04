use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use schemars::JsonSchema;
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
    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Condition {
    /// Field to check (e.g., 'page_filtered_text', 'page_title').
    pub field: String,
    /// Comparison operator (e.g., 'contains_regex', 'equals', 'not_equals').
    pub operator: String,
    /// Value to compare against.
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct BrowserStep {
    /// The operation to perform (e.g., 'click', 'wait', 'input').
    pub operation: Option<String>,
    /// The CSS/XPath selector to target.
    pub selector: Option<String>,
    /// An optional value for the operation (e.g., text to input).
    pub optional_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Watch {
    pub url: String,
    pub title: Option<String>,
    pub paused: Option<bool>,
    pub last_error: Option<serde_json::Value>,
    pub processor: Option<String>,
    pub last_viewed: Option<u64>,
    pub last_changed: Option<u64>,
    #[serde(default)]
    pub browser_steps: Option<Vec<BrowserStep>>,
    #[serde(default)]
    pub notification_muted: Option<bool>,
    #[serde(default)]
    pub notification_urls: Option<Vec<String>>,
    #[serde(default)]
    pub notification_title: Option<String>,
    #[serde(default)]
    pub notification_body: Option<String>,
    #[serde(default)]
    pub include_filters: Option<Vec<String>>,
    #[serde(default)]
    pub subtractive_selectors: Option<Vec<String>>,
    #[serde(default)]
    pub fetch_backend: Option<String>,
    #[serde(default)]
    pub conditions: Option<serde_json::Value>,
    #[serde(default)]
    pub conditions_match_logic: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
    #[serde(default)]
    pub previous_md5: Option<serde_json::Value>,
    #[serde(default)]
    pub previous_md5_before_filters: Option<serde_json::Value>,
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

    pub async fn list_fetchers(&self) -> Result<Vec<String>, ApiError> {
        let url = format!("{}/api/v1/fetchers", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let fetchers = response.json::<Vec<String>>().await?;
        Ok(fetchers)
    }

    pub async fn list_proxies(&self) -> Result<HashMap<String, String>, ApiError> {
        let url = format!("{}/api/v1/proxies", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let proxies = response.json::<HashMap<String, String>>().await?;
        Ok(proxies)
    }

    pub async fn get_global_settings(&self) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/settings", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let settings = response.json::<serde_json::Value>().await?;
        Ok(settings)
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

    pub async fn find_watches_by_error(&self) -> Result<HashMap<String, Watch>, ApiError> {
        let watches = self.list_watches(None).await?;
        let filtered = watches
            .into_iter()
            .filter(|(_, watch)| {
                if let Some(error) = &watch.last_error {
                    match error {
                        serde_json::Value::Bool(b) => *b,
                        serde_json::Value::String(s) => !s.is_empty(),
                        serde_json::Value::Null => false,
                        _ => true,
                    }
                } else {
                    false
                }
            })
            .collect();
        Ok(filtered)
    }

    pub async fn list_watches_by_processor(
        &self,
        processor: &str,
    ) -> Result<HashMap<String, Watch>, ApiError> {
        let watches = self.list_watches(None).await?;
        let filtered = watches
            .into_iter()
            .filter(|(_, watch)| watch.processor.as_deref() == Some(processor))
            .collect();
        Ok(filtered)
    }

    pub async fn get_watch_details(&self, uuid: &str) -> Result<Watch, ApiError> {
        let url = format!("{}/api/v1/watch/{}", self.base_url, uuid);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?;
        
        let status = response.status();
        let text = response.text().await?;
        println!("Get watch details status: {}, body: {}", status, text);

        if status.is_success() {
            let watch = serde_json::from_str::<Watch>(&text)?;
            Ok(watch)
        } else {
            Err(ApiError::Internal(format!("HTTP error {}: {}", status, text)))
        }
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
            .await?;
        
        let status = response.status();
        let text = response.text().await?;
        println!("Update watch status: {}, body: {}", status, text);

        if status.is_success() {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                return Ok(serde_json::json!({"status": "success"}));
            }
            if trimmed == "OK" || trimmed == "\"OK\"" {
                return Ok(serde_json::json!({"status": "success"}));
            }
            if trimmed.starts_with('{') || trimmed.starts_with('[') {
                match serde_json::from_str::<serde_json::Value>(trimmed) {
                    Ok(json) => return Ok(json),
                    Err(_) => {}
                }
            }
            Ok(serde_json::json!({ "status": trimmed.trim_matches('"') }))
        } else {
            Err(ApiError::Internal(format!("HTTP error {}: {}", status, text)))
        }
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
        let url = format!("{}/api/v1/watch/{}/recheck", self.base_url, uuid);
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
        Ok(serde_json::json!({"status": text}))
    }

    pub async fn trigger_recheck_all(&self, tag: Option<&str>) -> Result<serde_json::Value, ApiError> {
        let url = if let Some(t) = tag {
            // Find tag UUID first if it's a name, or use it directly if it looks like a UUID
            // Actually, the API for /tag/{uuid} recheck=true uses UUID.
            // Let's assume the user provides the tag UUID or we search for it.
            // For now, we'll try to find the tag by name if it's not a UUID.
            let tag_uuid = if uuid::Uuid::parse_str(t).is_ok() {
                t.to_string()
            } else {
                let tags = self.list_tags().await?;
                let mut found_uuid = t.to_string();
                if let Some(obj) = tags.as_object() {
                    for (uuid, tag_val) in obj {
                        if tag_val.get("title").and_then(|v| v.as_str()) == Some(t) {
                            found_uuid = uuid.clone();
                            break;
                        }
                    }
                }
                found_uuid
            };
            format!("{}/api/v1/tag/{}?recheck=true", self.base_url, tag_uuid)
        } else {
            format!("{}/api/v1/watch?recheck_all=1", self.base_url)
        };

        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let text = response.text().await?;
        let trimmed = text.trim();
        if trimmed.is_empty() || trimmed == "OK" {
            return Ok(serde_json::json!({"status": "success"}));
        }
        Ok(serde_json::json!({"status": trimmed}))
    }

    pub async fn mark_as_viewed(&self, uuid: &str) -> Result<serde_json::Value, ApiError> {
        let watch = self.get_watch_details(uuid).await?;
        let last_changed = watch.last_changed.unwrap_or(0);
        let mut payload = HashMap::new();
        payload.insert("last_viewed", serde_json::json!(last_changed + 1));

        self.update_watch(uuid, serde_json::to_value(payload)?)
            .await
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
        word_diff: Option<&str>,
        changes_only: Option<&str>,
        ignore_whitespace: Option<&str>,
    ) -> Result<String, ApiError> {
        let mut url = format!(
            "{}/api/v1/watch/{}/difference/{}/{}",
            self.base_url, uuid, from, to
        );
        let mut params = Vec::new();
        if let Some(fmt) = format_type {
            params.push(format!("format={}", fmt));
        }
        if let Some(wd) = word_diff {
            params.push(format!("word_diff={}", wd));
        }
        if let Some(co) = changes_only {
            params.push(format!("changesOnly={}", co));
        }
        if let Some(iw) = ignore_whitespace {
            params.push(format!("ignoreWhitespace={}", iw));
        }

        if !params.is_empty() {
            url.push_str("?");
            url.push_str(&params.join("&"));
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

    pub async fn get_watch_favicon(&self, uuid: &str) -> Result<Vec<u8>, ApiError> {
        let url = format!("{}/api/v1/watch/{}/favicon", self.base_url, uuid);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub async fn set_watch_selectors(
        &self,
        uuid: &str,
        css_filter: Option<&str>,
        xpath_filter: Option<&str>,
        json_filter: Option<&str>,
    ) -> Result<serde_json::Value, ApiError> {
        let mut filters = Vec::new();
        if let Some(css) = css_filter {
            filters.push(css.to_string());
        }
        if let Some(xpath) = xpath_filter {
            filters.push(xpath.to_string());
        }
        if let Some(json) = json_filter {
            filters.push(json.to_string());
        }

        let mut payload = HashMap::new();
        payload.insert("include_filters", serde_json::json!(filters));

        self.update_watch(uuid, serde_json::to_value(payload)?)
            .await
    }

    pub async fn set_browser_steps(
        &self,
        uuid: &str,
        steps: Vec<BrowserStep>,
    ) -> Result<serde_json::Value, ApiError> {
        let mut payload = HashMap::new();
        payload.insert("browser_steps", serde_json::json!(steps));

        self.update_watch(uuid, serde_json::to_value(payload)?)
            .await
    }

    pub async fn set_conditions(
        &self,
        uuid: &str,
        conditions: Vec<Condition>,
        match_logic: Option<&str>,
    ) -> Result<serde_json::Value, ApiError> {
        let mut payload = HashMap::new();
        payload.insert("conditions", serde_json::json!(conditions));
        if let Some(logic) = match_logic {
            payload.insert("conditions_match_logic", serde_json::json!(logic));
        }

        self.update_watch(uuid, serde_json::to_value(payload)?)
            .await
    }

    pub async fn set_request_config(
        &self,
        uuid: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<&str>,
    ) -> Result<serde_json::Value, ApiError> {
        let mut payload = HashMap::new();
        if let Some(h) = headers {
            payload.insert("headers", serde_json::json!(h));
        }
        if let Some(b) = body {
            payload.insert("body", serde_json::json!(b));
        }

        self.update_watch(uuid, serde_json::to_value(payload)?)
            .await
    }

    pub async fn set_watch_fetcher(
        &self,
        uuid: &str,
        fetcher: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let mut payload = HashMap::new();
        payload.insert("fetch_backend", serde_json::json!(fetcher));

        self.update_watch(uuid, serde_json::to_value(payload)?)
            .await
    }

    pub async fn configure_watch_notifications(
        &self,
        uuid: &str,
        notification_urls: Vec<String>,
        notification_title: Option<&str>,
        notification_body: Option<&str>,
    ) -> Result<serde_json::Value, ApiError> {
        let mut payload = HashMap::new();
        payload.insert("notification_urls", serde_json::json!(notification_urls));
        if let Some(title) = notification_title {
            payload.insert("notification_title", serde_json::json!(title));
        }
        if let Some(body) = notification_body {
            payload.insert("notification_body", serde_json::json!(body));
        }

        self.update_watch(uuid, serde_json::to_value(payload)?)
            .await
    }

    pub async fn list_processors(&self) -> Result<Vec<String>, ApiError> {
        let spec = self.get_full_spec().await?;
        let yaml: serde_yaml::Value = serde_yaml::from_str(&spec)
            .map_err(|e| ApiError::Internal(format!("Failed to parse OpenAPI spec: {}", e)))?;

        let schemas = yaml.get("components").and_then(|v| v.get("schemas"));

        if let Some(schemas) = schemas {
            // Try different possible locations for the processor enum
            for schema_name in &["WatchBase", "Watch", "CreateWatch", "UpdateWatch"] {
                if let Some(schema) = schemas.get(*schema_name) {
                    let enum_vals = schema
                        .get("properties")
                        .and_then(|v| v.get("processor"))
                        .and_then(|v| v.get("enum"))
                        .and_then(|v| v.as_sequence());

                    if let Some(seq) = enum_vals {
                        let processors: Vec<String> = seq
                            .iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect();
                        if !processors.is_empty() {
                            return Ok(processors);
                        }
                    }

                    // Also check allOf if present
                    if let Some(all_of) = schema.get("allOf").and_then(|v| v.as_sequence()) {
                        for sub_schema in all_of {
                            let enum_vals = sub_schema
                                .get("properties")
                                .and_then(|v| v.get("processor"))
                                .and_then(|v| v.get("enum"))
                                .and_then(|v| v.as_sequence());

                            if let Some(seq) = enum_vals {
                                let processors: Vec<String> = seq
                                    .iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect();
                                if !processors.is_empty() {
                                    return Ok(processors);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(vec![])
    }

    pub async fn list_all_history(
        &self,
        tag: Option<&str>,
    ) -> Result<HashMap<String, HashMap<String, String>>, ApiError> {
        let watches = self.list_watches(tag).await?;
        let mut all_history = HashMap::new();

        for uuid in watches.keys() {
            if let Ok(history) = self.get_watch_history(uuid).await {
                all_history.insert(uuid.clone(), history);
            }
        }

        Ok(all_history)
    }

    pub async fn set_history_limit(
        &self,
        uuid: &str,
        limit: i32,
    ) -> Result<serde_json::Value, ApiError> {
        let mut payload = HashMap::new();
        payload.insert("history_snapshot_max_length", serde_json::json!(limit));

        self.update_watch(uuid, serde_json::to_value(payload)?)
            .await
    }

    pub async fn set_bulk_history_limit(
        &self,
        tag: Option<&str>,
        limit: i32,
    ) -> Result<serde_json::Value, ApiError> {
        let watches = if let Some(t) = tag {
            self.list_watches(Some(t)).await?
        } else {
            self.list_watches(None).await?
        };

        let mut results = HashMap::new();
        for (uuid, _) in watches {
            if let Ok(res) = self.set_history_limit(&uuid, limit).await {
                results.insert(uuid, res);
            }
        }

        Ok(serde_json::to_value(results)?)
    }

    pub async fn get_snapshot_info(
        &self,
        uuid: &str,
        timestamp: &str,
    ) -> Result<serde_json::Value, ApiError> {
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

        let headers = response.headers();
        let content_length = headers
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());
        let content_type = headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let last_modified = headers
            .get("last-modified")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Ok(serde_json::json!({
            "uuid": uuid,
            "timestamp": timestamp,
            "content_length": content_length,
            "content_type": content_type,
            "last_modified": last_modified,
        }))
    }

    pub async fn trigger_backup(&self) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}/api/v1/backup", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .send()
            .await?
            .error_for_status()?;
        let result = response.json::<serde_json::Value>().await?;
        Ok(result)
    }

    pub async fn export_watches_to_json(
        &self,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        let url = format!("{}/api/v1/watch", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        let watches = response
            .json::<HashMap<String, serde_json::Value>>()
            .await?;

        // We iterate and fetch full details for each watch to ensure a "Full JSON export"
        let mut full_export = HashMap::new();
        for (uuid, _) in watches {
            let details_url = format!("{}/api/v1/watch/{}", self.base_url, uuid);
            if let Ok(details_response) = self.http_client.get(&details_url).send().await {
                if let Ok(details) = details_response.json::<serde_json::Value>().await {
                    full_export.insert(uuid, details);
                }
            }
        }

        Ok(full_export)
    }
}
