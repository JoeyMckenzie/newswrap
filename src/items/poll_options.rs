//! Poll options associated to a poll.

use time::OffsetDateTime;

use crate::{
    errors::HackerNewsClientError,
    items::{HackerNewsItem, HackerNewsItemType},
    HackerNewsID,
};

/// Represents a Hacker News poll option and all associated data to it including the parent poll and author.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HackerNewsPollOption {
    /// The item's unique id.
    pub id: HackerNewsID,
    /// The parent poll ID of the poll option.
    pub poll: u32,
    /// The number of upvotes on the job posting.
    pub score: u32,
    /// Creation date of the job posting.
    pub created_at: OffsetDateTime,
    /// The job listing description.
    pub text: String,
    /// Username of the job poster.
    pub by: String,
}

impl TryFrom<HackerNewsItem> for HackerNewsPollOption {
    type Error = HackerNewsClientError;

    fn try_from(item: HackerNewsItem) -> Result<Self, Self::Error> {
        if item.get_item_type() != HackerNewsItemType::PollOption {
            return Err(HackerNewsClientError::InvalidTypeMapping(
                item.get_item_type(),
            ));
        }

        if item.poll.is_none() {
            return Err(HackerNewsClientError::AssociatedParentNotFound(item.id));
        }

        Ok(Self {
            id: item.id,
            created_at: item.created_at,
            text: item.text.unwrap_or_default(),
            by: item.by.unwrap_or_default(),
            score: item.score.unwrap_or(0),
            poll: item.poll.unwrap(),
        })
    }
}
