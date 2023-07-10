//! User response models and various metadata about accounts and related objects.

pub mod client;

use serde::Deserialize;
use time::OffsetDateTime;

use crate::HackerNewsID;

/// Represents a Hacker News user and their associated metadata.
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HackerNewsUser {
    /// Username of the account.
    pub id: String,
    /// Creation date of the item, in Unix Time.
    #[serde(with = "time::serde::timestamp")]
    pub created: OffsetDateTime,
    /// The user's karma.
    pub karma: u32,
    /// The user's optional self-description. HTML.
    pub about: Option<String>,
    /// List of the user's stories, polls and comments.
    pub stories: Option<Vec<HackerNewsID>>,
}

impl HackerNewsUser {
    /// Determines if the user's about section is populated
    pub fn has_about_section(&self) -> bool {
        self.about.is_some()
    }

    /// Determines if the user has related Hacker News content.
    pub fn has_related_stories(&self) -> bool {
        self.stories.is_some()
    }
}
