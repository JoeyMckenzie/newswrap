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

#[derive(Deserialize)]
pub struct ItemResponse {
    id: i32,
    deleted: bool,
    #[serde(rename = "type")]
    responseType: String,
}
