//! Hacker News API client bindings and various methods for interacting.

use std::cell::OnceCell;

use crate::{errors::ClientError, items::HackerNewsItem};

const API_BASE_URL: &str = "https://hacker-news.firebaseio.com";
const ITEM_ENDPOINT: &str = "item";

/// Version information for the Hacker News API containing the base URLs.
#[derive(Debug, Clone, Copy)]
pub enum ApiVersion {
    /// Represents version 0 of the Hacker News API.
    V0,
}

/// A wrapping HTTP client for Hacker News Firebase API and real-time data.
/// A client instance should only be instantiated once in an application's
/// lifecycle, seeking to reuse it where possible.
#[derive(Debug)]
pub struct HackerNewsClient {
    /// An internal HTTP client to be used for connections to the Hacker News API.
    http_client: OnceCell<reqwest::Client>,
    /// The internal version of the Hacker News API your client will target.
    version: ApiVersion,
}

impl Default for HackerNewsClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HackerNewsClient {
    /// Constructs a new implementation of the client pointing to the latest Hacker News API version.
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        let celled_client = OnceCell::<reqwest::Client>::new();
        celled_client.get_or_init(|| client);

        Self {
            http_client: celled_client,
            version: ApiVersion::V0,
        }
    }

    /// Constructs the base URL including the version.
    fn versioned_api_base_url(&self) -> String {
        let version = match self.version {
            ApiVersion::V0 => "v0",
        };

        format!("{}/{}", API_BASE_URL, version)
    }

    /// Returns a cloned instance of the celled HTTP client.
    pub fn clone_client(&self) -> OnceCell<reqwest::Client> {
        self.http_client.clone()
    }

    /// Retrieves item information based on the given ID.
    pub async fn get_item(&self, id: u32) -> Result<HackerNewsItem, ClientError> {
        let item = self
            .http_client
            .get()
            .unwrap()
            .get(format!(
                "{}/{}/{}.json",
                self.versioned_api_base_url(),
                ITEM_ENDPOINT,
                id
            ))
            .send()
            .await?
            .json::<HackerNewsItem>()
            .await?;

        Ok(item)
    }
}
