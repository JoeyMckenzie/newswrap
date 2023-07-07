use std::rc::Rc;

use crate::{errors::HackerNewsResult, HackerNewsID};

const MAX_ITEM_ENDPOINT: &str = "maxitem";

/// an internal client for interacting with the realtime data endpoints.
#[derive(Debug)]
pub struct HackerNewsRealtimeClient {
    ref_client: Rc<reqwest::Client>,
    base_url: &'static str,
}

impl HackerNewsRealtimeClient {
    /// Constructs a new instance of the realtime client from the root HTTP client.
    pub fn new(ref_client: Rc<reqwest::Client>, base_url: &'static str) -> Self {
        Self {
            ref_client,
            base_url,
        }
    }

    pub async fn get_latest_item_id(&self) -> HackerNewsResult<HackerNewsID> {
        let item_id = self
            .ref_client
            .get(format!("{}/{}.json", self.base_url, MAX_ITEM_ENDPOINT))
            .send()
            .await?
            .text()
            .await?
            .parse::<u32>()?;

        Ok(item_id)
    }
}
