//! A client for interacting with item types and endpoints.

use crate::{
    errors::{HackerNewsClientError, HackerNewsResult},
    http::InternalHttpClient,
    HackerNewsID,
};

use super::{
    comments::HackerNewsComment, jobs::HackerNewsJob, poll_options::HackerNewsPollOption,
    polls::HackerNewsPoll, stories::HackerNewsStory, HackerNewsItem,
};

const ITEM_ENDPOINT: &str = "item";

/// An internal items client for interacting with item endpoints.
#[derive(Debug)]
pub struct HackerNewsItemClient {
    internal_client: InternalHttpClient,
}

impl HackerNewsItemClient {
    /// Constructs a new item client for interacting with the item endpoints using the internal HTTP client.
    pub fn new(internal_client: InternalHttpClient) -> Self {
        Self { internal_client }
    }

    /// Retrieves item information based on the given ID.
    pub async fn get_item(&self, id: HackerNewsID) -> HackerNewsResult<HackerNewsItem> {
        let item = self
            .internal_client
            .get_item_with_id(ITEM_ENDPOINT, id)
            .await?;
        Ok(item)
    }

    /// Generic retrieval of various hacker news items assuming they are validly convertible to a subtype.
    async fn get_typed_item<T>(&self, id: HackerNewsID) -> HackerNewsResult<T>
    where
        HackerNewsClientError: From<<T as TryFrom<HackerNewsItem>>::Error>,
        T: TryFrom<HackerNewsItem>,
    {
        let item = self.get_item(id).await?;
        let typed_item = item.try_into()?;
        Ok(typed_item)
    }

    /// Retrieves a story from Hacker News, returning errors if the item was not a valid story type.
    pub async fn get_story(&self, id: HackerNewsID) -> HackerNewsResult<HackerNewsStory> {
        self.get_typed_item(id).await
    }

    /// Retrieves a story comment from Hacker News, returning errors if the item was not a valid comment type.
    pub async fn get_comment(&self, id: HackerNewsID) -> HackerNewsResult<HackerNewsComment> {
        self.get_typed_item(id).await
    }

    /// Retrieves a job posting from Hacker News, returning errors if the item was not a valid job posting type.
    pub async fn get_job(&self, id: HackerNewsID) -> HackerNewsResult<HackerNewsJob> {
        self.get_typed_item(id).await
    }

    /// Retrieves a poll from Hacker News, returning errors if the item was not a valid poll type.
    pub async fn get_poll(&self, id: HackerNewsID) -> HackerNewsResult<HackerNewsPoll> {
        self.get_typed_item(id).await
    }

    /// Retrieves a poll option from Hacker News, returning errors if the item was not a valid poll option type.
    pub async fn get_poll_option(
        &self,
        id: HackerNewsID,
    ) -> HackerNewsResult<HackerNewsPollOption> {
        self.get_typed_item(id).await
    }
}
