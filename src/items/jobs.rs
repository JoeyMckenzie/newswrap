//! Jobs listed on the Hacker News and all associated data.

use time::OffsetDateTime;

use crate::{
    errors::HackerNewsClientError,
    items::{HackerNewsItem, HackerNewsItemType},
    HackerNewsID,
};

/// Represents a Hacker News story and all associated data to it including author, text, and child comments.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HackerNewsJob {
    /// The item's unique id.
    pub id: HackerNewsID,
    /// The number of upvotes on the job posting.
    pub score: u32,
    /// Creation date of the job posting.
    pub created_at: OffsetDateTime,
    /// The job listing title.
    pub title: String,
    /// The job listing description.
    pub text: String,
    /// Username of the job poster.
    pub by: String,
    /// URL of the job posting.
    pub url: String,
}

impl TryFrom<HackerNewsItem> for HackerNewsJob {
    type Error = HackerNewsClientError;

    fn try_from(item: HackerNewsItem) -> Result<Self, Self::Error> {
        if item.get_item_type() != HackerNewsItemType::Comment {
            return Err(HackerNewsClientError::InvalidTypeMapping(
                item.get_item_type(),
            ));
        }

        Ok(Self {
            id: item.id,
            created_at: item.created_at,
            text: item.text.unwrap_or_default(),
            by: item.by.unwrap_or_default(),
            score: item.score.unwrap_or(0),
            title: item.title.unwrap_or_default(),
            url: item.url.unwrap_or_default(),
        })
    }
}
