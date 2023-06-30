//! Item response types associated to various Hacker News posts, comments, users, etc.

use serde::Deserialize;

/// Hacker News response type included on each item retrieval.
#[derive(Debug, Clone, Copy)]
pub enum HackerNewsMediaType {
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
}

/// Represents a Hacker News item returned from the item endpoint.
#[derive(Debug, Deserialize)]
pub struct HackerNewsItem {
    id: u32,
    deleted: Option<bool>,
    #[serde(rename = "type")]
    response_type: String,
}
