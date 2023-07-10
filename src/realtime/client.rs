//! Realtime client API for interacting with live data endpoints of Hacker News.

use crate::{errors::HackerNewsResult, http::InternalHttpClient, HackerNewsID};

use super::{HackerNewsItemList, HackerNewsUpdatedItems};

/// An internal client for interacting with the realtime data endpoints.
#[derive(Debug)]
pub struct HackerNewsRealtimeClient {
    internal_client: InternalHttpClient,
}

/// Endpoint for the latest item ID.
const MAX_ITEM_ENDPOINT: &str = "maxitem";

/// Endpoints for stories.
const TOP_STORIES_ENDPOINT: &str = "topstories";
const NEW_STORIES_ENDPOINT: &str = "newstories";
const BEST_STORIES_ENDPOINT: &str = "beststories";

/// Endpoints for jobs, polls, and asks.
const ASK_STORIES_ENDPOINT: &str = "askstories";
const SHOW_STORIES_ENDPOINT: &str = "showstories";
const JOB_STORIES_ENDPOINT: &str = "jobstories";

/// Endpoint for updated items and profiles.
const UPDATES_ENDPOINT: &str = "updates";

impl HackerNewsRealtimeClient {
    /// Constructs a new instance of the realtime client from the root HTTP client.
    pub fn new(internal_client: InternalHttpClient) -> Self {
        Self { internal_client }
    }

    async fn get_realtime_story_data(
        &self,
        endpoint: &str,
    ) -> HackerNewsResult<HackerNewsItemList> {
        let stories = self.internal_client.get_item(endpoint).await?;
        Ok(stories)
    }

    /// Retrieves the latest item ID to be created, referred to by Hacker News as the max item ID.
    pub async fn get_latest_item_id(&self) -> HackerNewsResult<HackerNewsID> {
        let item_id = self.internal_client.get_item(MAX_ITEM_ENDPOINT).await?;
        Ok(item_id)
    }

    /// Retrieves the top 500 stories and jobs.
    pub async fn get_top_stories(&self) -> HackerNewsResult<HackerNewsItemList> {
        self.get_realtime_story_data(TOP_STORIES_ENDPOINT).await
    }

    /// Retrieves the latest 500 stories.
    pub async fn get_latest_stories(&self) -> HackerNewsResult<HackerNewsItemList> {
        self.get_realtime_story_data(NEW_STORIES_ENDPOINT).await
    }

    /// Retrieves the best 500 stories.
    pub async fn get_best_stories(&self) -> HackerNewsResult<HackerNewsItemList> {
        self.get_realtime_story_data(BEST_STORIES_ENDPOINT).await
    }

    /// Retrieves up to 200 of the latest Ask Hacker News stories.
    pub async fn get_ask_hacker_news_stories(&self) -> HackerNewsResult<HackerNewsItemList> {
        self.get_realtime_story_data(ASK_STORIES_ENDPOINT).await
    }

    /// Retrieves up to 200 of the latest Show Hacker News stories.
    pub async fn get_show_hacker_news_stories(&self) -> HackerNewsResult<HackerNewsItemList> {
        self.get_realtime_story_data(SHOW_STORIES_ENDPOINT).await
    }

    /// Retrieves up to 200 of the latest job stories.
    pub async fn get_job_hacker_news_stories(&self) -> HackerNewsResult<HackerNewsItemList> {
        self.get_realtime_story_data(JOB_STORIES_ENDPOINT).await
    }

    /// Retrieves the most recently updated items.
    pub async fn get_recently_updated_items(&self) -> HackerNewsResult<HackerNewsItemList> {
        let updated_items: HackerNewsUpdatedItems =
            self.internal_client.get_item(UPDATES_ENDPOINT).await?;
        Ok(updated_items.items)
    }

    /// Retrieves the most recently updated profiles>.
    pub async fn get_recently_updated_profiles(&self) -> HackerNewsResult<HackerNewsItemList> {
        let updated_items: HackerNewsUpdatedItems =
            self.internal_client.get_item(UPDATES_ENDPOINT).await?;
        Ok(updated_items.profiles)
    }
}
