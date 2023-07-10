//! Polls listed on Hacker News and all associated data.

use time::OffsetDateTime;

use crate::{
    errors::HackerNewsClientError,
    items::{HackerNewsItem, HackerNewsItemType},
    HackerNewsID,
};

/// Represents a Hacker News poll and all associated data including comments on the poll, poll options, etc.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HackerNewsPoll {
    /// The item's unique id.
    pub id: HackerNewsID,
    /// The number of users participating in the poll.
    pub participants: u32,
    /// The associated comments on the poll.
    pub comments: Vec<HackerNewsID>,
    /// The associated poll choices.
    pub poll_options: Vec<HackerNewsID>,
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
}

impl TryFrom<HackerNewsItem> for HackerNewsPoll {
    type Error = HackerNewsClientError;

    fn try_from(item: HackerNewsItem) -> Result<Self, Self::Error> {
        if item.get_item_type() != HackerNewsItemType::Poll {
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
            participants: item.descendants.unwrap_or(0),
            comments: item.kids.unwrap_or_default(),
            poll_options: item.parts.unwrap_or_default(),
        })
    }
}
