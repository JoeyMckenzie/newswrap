use serde::Deserialize;

/// Hacker News response type included on each item retrieval.
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

#[derive(Debug, Deserialize)]
pub struct HackerNewsItem {
    id: u32,
    deleted: Option<bool>,
    #[serde(rename = "type")]
    response_type: String,
}
