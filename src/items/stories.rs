//! Stories listed on the Hacker News homepage and all associated data.

use serde::Serialize;
use time::OffsetDateTime;

use crate::{
    errors::HackerNewsClientError,
    items::{HackerNewsItem, HackerNewsItemType},
    HackerNewsID,
};

/// Represents a Hacker News story and all associated data to it including author, text, and child comments.
#[derive(Debug, Serialize)]
pub struct HackerNewsStory {
    /// The item's unique id.
    pub id: HackerNewsID,
    /// The total comment count.
    pub number_of_comments: u32,
    /// A list of associated child comment IDs.
    pub comments: Vec<HackerNewsID>,
    /// The story's total number of upvotes.
    pub score: u32,
    /// Creation date of the story.
    pub created_at: OffsetDateTime,
    /// Title of the story.
    pub title: String,
    /// URL of the story.
    pub url: String,
    /// Username of the story poster.
    pub by: String,
    /// Text associated to the story.
    pub text: String,
}

impl TryFrom<HackerNewsItem> for HackerNewsStory {
    type Error = HackerNewsClientError;

    fn try_from(item: HackerNewsItem) -> Result<Self, Self::Error> {
        if item.get_item_type() != HackerNewsItemType::Story {
            return Err(HackerNewsClientError::InvalidTypeMapping(
                item.get_item_type(),
            ));
        }

        Ok(Self {
            id: item.id,
            number_of_comments: item.descendants.unwrap_or(0),
            comments: item.kids.unwrap_or_default(),
            score: item.score.unwrap_or(0),
            created_at: item.created_at,
            title: item.title.unwrap_or_default(),
            url: item.url.unwrap_or_default(),
            by: item.by.unwrap_or_default(),
            text: item.text.unwrap_or_default(),
        })
    }
}
