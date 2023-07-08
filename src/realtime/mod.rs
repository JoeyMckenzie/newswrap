//! Data associated to the live data API endpoints pertaining to top stories, latest items, recently updated users, etc.
//! Live data endpoints will vary in terms of their content and provide realtime insights into data captured by Hacker News.

use serde::Deserialize;

use crate::HackerNewsID;

pub mod client;

/// A list of IDs returned from the live data endpoints.
type HackerNewsItemList = Vec<HackerNewsID>;

/// Recently changed items and profiles from the live data endpoints.
#[derive(Debug, Deserialize)]
pub struct HackerNewsUpdatedItems {
    /// Represents recently updated item IDs.
    pub items: HackerNewsItemList,
    /// Represents recently updated profile IDs.
    pub profiles: HackerNewsItemList,
}
