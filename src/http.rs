use std::fmt::Display;

use serde::Deserialize;

use crate::errors::HackerNewsResult;

/// An internal reqwest-based HTTP client for interacting with Hacker News.
#[derive(Debug)]
pub struct InternalHttpClient {
    http: reqwest::Client,
    base_url: &'static str,
}

impl InternalHttpClient {
    /// Constructs a new internal client with the base URL of the Hacker News API and the configured HTTP client.
    pub fn new(http: reqwest::Client, base_url: &'static str) -> Self {
        Self { http, base_url }
    }

    /// Exposes the internal client for configuring requests manually, should only be used as an escape hatch.
    pub fn get_client(&self) -> reqwest::Client {
        self.http.clone()
    }

    /// Exposes the base URL, should be used in conjunction with the exposed client for manual requests.
    pub fn api_base_url(&self) -> &'static str {
        self.base_url
    }

    /// Retrieves an item from Hacker News generic over the endpoint being called.
    pub async fn get_item_with_id<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        id: impl Display,
    ) -> HackerNewsResult<T> {
        let item = self
            .http
            .get(format!("{}/{}/{}.json", self.base_url, endpoint, id))
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(item)
    }
}
