use std::fmt::Display;

use serde::Deserialize;

use crate::errors::HackerNewsResult;

/// An internal reqwest-based HTTP client for interacting with Hacker News.
#[derive(Debug)]
pub struct InternalHttpClient {
    reqwest: reqwest::Client,
    base_url: &'static str,
}

impl InternalHttpClient {
    pub fn new(reqwest: reqwest::Client, base_url: &'static str) -> Self {
        Self { reqwest, base_url }
    }

    // TODO: Don't really want to do this, will clean up later
    pub fn get_client(&self) -> reqwest::Client {
        self.reqwest.clone()
    }

    pub fn api_base_url(&self) -> &'static str {
        self.base_url
    }

    pub async fn get_item_with_id<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        id: impl Display,
    ) -> HackerNewsResult<T> {
        let item = self
            .reqwest
            .get(format!("{}/{}/{}.json", self.base_url, endpoint, id))
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(item)
    }
}
