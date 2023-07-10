//! Story comments associated to Hacker News stories  and all associated data.

use serde::Serialize;
use time::OffsetDateTime;

use crate::{
    errors::HackerNewsClientError,
    items::{HackerNewsItem, HackerNewsItemType},
    HackerNewsID,
};

/// Represents a Hacker News story comment and all associated data to it including author and child comments.
#[derive(Debug, Serialize)]
pub struct HackerNewsComment {
    /// The item's unique id.
    pub id: HackerNewsID,
    /// A list of associated child comment IDs.
    pub sub_comments: Vec<HackerNewsID>,
    /// Creation date of the comment.
    pub created_at: OffsetDateTime,
    /// The ID of the parent story.
    pub parent_story: HackerNewsID,
    /// Content of the comment.
    pub text: String,
    /// Username of the comment poster.
    pub by: String,
}

impl TryFrom<HackerNewsItem> for HackerNewsComment {
    type Error = HackerNewsClientError;

    fn try_from(item: HackerNewsItem) -> Result<Self, Self::Error> {
        if item.get_item_type() != HackerNewsItemType::Comment {
            return Err(HackerNewsClientError::InvalidTypeMapping(
                item.get_item_type(),
            ));
        }

        Ok(Self {
            id: item.id,
            sub_comments: item.kids.unwrap_or_default(),
            created_at: item.created_at,
            parent_story: item.parent.unwrap_or_default(),
            text: item.text.unwrap_or_default(),
            by: item.by.unwrap_or_default(),
        })
    }
}
