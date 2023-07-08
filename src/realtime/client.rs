use std::rc::Rc;

use crate::{errors::HackerNewsResult, http::InternalHttpClient, HackerNewsID};

const MAX_ITEM_ENDPOINT: &str = "maxitem";

/// an internal client for interacting with the realtime data endpoints.
#[derive(Debug)]
pub struct HackerNewsRealtimeClient {
    internal_client: Rc<InternalHttpClient>,
}

impl HackerNewsRealtimeClient {
    /// Constructs a new instance of the realtime client from the root HTTP client.
    pub fn new(internal_client: Rc<InternalHttpClient>) -> Self {
        Self { internal_client }
    }

    /// Retrieves the latest item ID to be created, referred to by Hacker News as the max item ID.
    pub async fn get_latest_item_id(&self) -> HackerNewsResult<HackerNewsID> {
        let item_id = self
            .internal_client
            .get_client()
            .get(format!(
                "{}/{}.json",
                self.internal_client.api_base_url(),
                MAX_ITEM_ENDPOINT
            ))
            .send()
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        Ok(item_id)
    }
}
