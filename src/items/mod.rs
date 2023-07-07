//! Item response types associated to various Hacker News posts, comments, users, etc.
//! Items are top level items in the Hacker News API and represent all stories, comments,
//! polls, jobs, and pollopts. Items contain a `type` identifying which entity they represent.

pub mod comments;
pub mod jobs;
pub mod poll;
pub mod poll_option;
pub mod stories;

use std::fmt::Display;

use serde::Deserialize;
use time::OffsetDateTime;

use crate::HackerNewsID;

const ITEM_TYPE_COMMENT: &str = "comment";
const ITEM_TYPE_JOB: &str = "job";
const ITEM_TYPE_POLL: &str = "poll";
const ITEM_TYPE_POLLOPT: &str = "pollopt";
const ITEM_TYPE_STORY: &str = "story";

/// Hacker News response type included on each item retrieval.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum HackerNewsItemType {
    /// The comment type, representing comments on articles and users.
    Comment,
    /// The Job type, representing jobs.
    Job,
    /// The Job type, representing jobs.
    Poll,
    /// The Job type, representing jobs.
    PollOpt,
    /// The Job type, representing jobs.
    Story,
    /// An unknown type in the case a match is not found for the item type
    Unknown,
}

impl Display for HackerNewsItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents a Hacker News item returned from the item endpoint.
#[derive(Debug, Deserialize)]
pub struct HackerNewsItem {
    /// The item's unique id.
    pub id: HackerNewsID,
    /// Flag representing item state, true if the item is deleted.
    pub deleted: Option<bool>,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    #[serde(rename = "type")]
    pub response_type: Option<String>,
    /// The username of the item's author.
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    #[serde(with = "time::serde::timestamp", rename = "time")]
    pub created_at: OffsetDateTime,
    /// Flag representing active state, true if the item is dead.
    pub dead: Option<bool>,
    /// The comment's parent: either another comment or the relevant story.
    pub parent: Option<HackerNewsID>,
    /// The pollopt's associated poll.
    pub poll: Option<HackerNewsID>,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<HackerNewsID>>,
    /// The URL of the story.
    pub url: Option<String>,
    /// The story's score, or the votes for a pollopt.
    pub score: Option<u32>,
    /// The title of the story, poll or job. HTML.
    pub title: Option<String>,
    /// The comment, story or poll text. HTML.
    pub text: Option<String>,
    /// A list of related pollopts, in display order.
    pub parts: Option<Vec<HackerNewsID>>,
    /// In the case of stories or polls, the total comment count.
    pub descendants: Option<u32>,
}

impl HackerNewsItem {
    fn parse_item_type(&self, item_type: &str) -> HackerNewsItemType {
        match item_type {
            ITEM_TYPE_COMMENT => HackerNewsItemType::Comment,
            ITEM_TYPE_JOB => HackerNewsItemType::Job,
            ITEM_TYPE_POLL => HackerNewsItemType::Poll,
            ITEM_TYPE_POLLOPT => HackerNewsItemType::PollOpt,
            ITEM_TYPE_STORY => HackerNewsItemType::Story,
            _ => HackerNewsItemType::Unknown,
        }
    }

    fn is_item_type(&self, item_type: HackerNewsItemType) -> bool {
        self.get_item_type() == item_type
    }

    /// Returns a typed variant of the item type based on the response item.
    pub fn get_item_type(&self) -> HackerNewsItemType {
        match &self.response_type {
            Some(item_type) => self.parse_item_type(&item_type.to_lowercase()),
            None => HackerNewsItemType::Unknown,
        }
    }

    /// Determines if the item type is a comment.
    pub fn is_comment(&self) -> bool {
        self.is_item_type(HackerNewsItemType::Comment)
    }

    /// Determines if the item type is a job.
    pub fn is_job(&self) -> bool {
        self.is_item_type(HackerNewsItemType::Job)
    }

    /// Determines if the item type is a poll.
    pub fn is_poll(&self) -> bool {
        self.is_item_type(HackerNewsItemType::Poll)
    }

    /// Determines if the item type is a poll option.
    pub fn is_pollopt(&self) -> bool {
        self.is_item_type(HackerNewsItemType::PollOpt)
    }

    /// Determines if the item type is a story.
    pub fn is_story(&self) -> bool {
        self.is_item_type(HackerNewsItemType::Story)
    }
}
