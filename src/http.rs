use std::fmt::Display;

use serde::Deserialize;

use crate::errors::HackerNewsResult;

/// An internal reqwest-based HTTP client for interacting with Hacker News.
#[derive(Debug, Clone)]
pub struct InternalHttpClient {
    http: reqwest::Client,
    base_url: &'static str,
}

impl InternalHttpClient {
    /// Constructs a new internal client with the base URL of the Hacker News API and the configured HTTP client.
    pub fn new(http: reqwest::Client, base_url: &'static str) -> Self {
        Self { http, base_url }
    }

    /// Retrieves an item from Hacker News generic over the endpoint being called.
    pub async fn get_item<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
    ) -> HackerNewsResult<T> {
        let item_result = self
            .http
            .get(format!("{}/{}.json", self.base_url, endpoint))
            .send()
            .await;

        match item_result {
            Ok(item) => {
                let parsed_item = item.json::<T>().await?;
                Ok(parsed_item)
            }
            Err(_) => Err(crate::errors::HackerNewsClientError::ItemOrUserNotFound(
                endpoint.to_string(),
            )),
        }
    }

    /// Retrieves an item from Hacker News generic over the endpoint being called.
    pub async fn get_item_with_id<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        id: impl Display,
    ) -> HackerNewsResult<T> {
        let item_result = self
            .http
            .get(format!("{}/{}/{}.json", self.base_url, endpoint, id))
            .send()
            .await;

        match item_result {
            Ok(item) => {
                let parsed_item = item.json::<T>().await?;
                Ok(parsed_item)
            }
            Err(_) => Err(crate::errors::HackerNewsClientError::ItemOrUserNotFound(
                id.to_string(),
            )),
        }
    }
}
