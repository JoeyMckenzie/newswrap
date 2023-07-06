//! Hacker News API client bindings and various methods for interacting.

use crate::{
    comments::HackerNewsComment, errors::HackerNewsResult, items::HackerNewsItem,
    stories::HackerNewsStory, users::HackerNewsUser, HackerNewsID, API_BASE_URL,
    DEFAULT_TIMEOUT_SECONDS, ITEM_ENDPOINT, USERS_ENDPOINT,
};

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
    http_client: reqwest::Client,
    /// The internal version of the Hacker News API your client will target.
    version: ApiVersion,
}

impl Default for HackerNewsClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HackerNewsClient {
    /// Internally constructs the client allowing for flexibility in configuring the timeout.
    fn new_client(timeout: u64) -> Self {
        let client = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(timeout))
            .build()
            .unwrap();

        Self {
            http_client: client,
            version: ApiVersion::V0,
        }
    }

    /// Constructs a new client pointing to the latest Hacker News API version.
    pub fn new() -> Self {
        Self::new_client(DEFAULT_TIMEOUT_SECONDS)
    }

    /// Constructs a new client pointing to the latest Hacker News API version with the configured request timeout.
    pub fn new_with_timeout(timeout: u64) -> Self {
        Self::new_client(timeout)
    }

    /// Constructs the base URL including the version.
    fn versioned_api_base_url(&self) -> String {
        let version = match self.version {
            ApiVersion::V0 => "v0",
        };

        format!("{}/{}", API_BASE_URL, version)
    }

    /// Retrieves item information based on the given ID.
    pub async fn get_item(&self, id: HackerNewsID) -> HackerNewsResult<HackerNewsItem> {
        let item = self
            .http_client
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

    /// Retrieves a user from the user endpoint based on the provided username.
    pub async fn get_user(&self, username: &str) -> HackerNewsResult<HackerNewsUser> {
        let user = self
            .http_client
            .get(format!(
                "{}/{}/{}.json",
                self.versioned_api_base_url(),
                USERS_ENDPOINT,
                username
            ))
            .send()
            .await?
            .json::<HackerNewsUser>()
            .await?;

        Ok(user)
    }

    /// Retrieves a story from Hacker News, returning errors if the item was not a valid story type.
    pub async fn get_story(&self, id: HackerNewsID) -> HackerNewsResult<HackerNewsStory> {
        let item = self.get_item(id).await?;
        let story = item.try_into()?;
        Ok(story)
    }

    /// Retrieves a story comment from Hacker News, returning errors if the item was not a valid comment type.
    pub async fn get_comment(&self, id: HackerNewsID) -> HackerNewsResult<HackerNewsComment> {
        let item = self.get_item(id).await?;
        let comment = item.try_into()?;
        Ok(comment)
    }
}
