use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
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

pub struct Client {
    base_url: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(base_url: String, api_key: String) -> Self {
        let mut headers = HeaderMap::new();
        if let Ok(val) = HeaderValue::from_str(&api_key) {
            headers.insert("x-api-key", val);
        }

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http_client,
        }
    }

    pub async fn list_watches(&self) -> Result<HashMap<String, Watch>, ApiError> {
        let url = format!("{}/api/v1/watch", self.base_url);
        let response = self.http_client.get(&url).send().await?;
        let watches = response.json::<HashMap<String, Watch>>().await?;
        Ok(watches)
    }

    pub async fn get_watch_details(&self, uuid: &str) -> Result<Watch, ApiError> {
        let url = format!("{}/api/v1/watch/{}", self.base_url, uuid);
        let response = self.http_client.get(&url).send().await?;
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

        let response = self.http_client.post(&endpoint).json(&body).send().await?;
        let result = response.json::<HashMap<String, String>>().await?;
        Ok(result)
    }

    pub async fn delete_watch(&self, uuid: &str) -> Result<HashMap<String, String>, ApiError> {
        let url = format!("{}/api/v1/watch/{}", self.base_url, uuid);
        let response = self.http_client.delete(&url).send().await?;
        let result = response.json::<HashMap<String, String>>().await?;
        Ok(result)
    }

    pub async fn trigger_check(&self, uuid: &str) -> Result<HashMap<String, String>, ApiError> {
        let url = format!("{}/api/v1/watch/{}/recheck", self.base_url, uuid);
        let response = self.http_client.get(&url).send().await?;
        let result = response.json::<HashMap<String, String>>().await?;
        Ok(result)
    }
}
