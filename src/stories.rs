//! Stories listed on the Hacker News homepage and all associated data.

use serde::Deserialize;
use time::OffsetDateTime;

use crate::{items::HackerNewsItem, HackerNewsID};

/// Represents a Hacker News story and all associated data to it including author, text, and child comments.
///
// {
//   "by" : "dhouston",
//   "descendants" : 71,
//   "id" : 8863,
//   "kids" : [ 8952, 9224, 8917, 8884, 8887, 8943, 8869, 8958, 9005, 9671, 8940, 9067, 8908, 9055, 8865, 8881, 8872, 8873, 8955, 10403, 8903, 8928, 9125, 8998, 8901, 8902, 8907, 8894, 8878, 8870, 8980, 8934, 8876 ],
//   "score" : 111,
//   "time" : 1175714200,
//   "title" : "My YC app: Dropbox - Throw away your USB drive",
//   "type" : "story",
//   "url" : "http://www.getdropbox.com/u/2/screencast.html"
// }
#[derive(Debug, Deserialize)]
pub struct HackerNewsStory {
    /// The item's unique id.
    pub id: HackerNewsID,
    /// The total comment count.
    #[serde(rename = "descendants")]
    pub number_of_comments: Option<u32>,
    /// A list of associated child comment IDs.
    #[serde(rename = "kids")]
    pub comments: Vec<HackerNewsID>,
    /// The story's total number of upvotes
    pub score: u32,
    /// Creation date of the story.
    #[serde(with = "time::serde::timestamp", rename = "time")]
    pub created_at: OffsetDateTime,
    /// Title of the story.
    pub title: String,
    /// URL of the story.
    pub url: String,
}

impl From<HackerNewsItem> for HackerNewsStory {
    fn from(item: HackerNewsItem) -> Self {
        Self {
            id: item.id,
            number_of_comments: item.descendants,
            comments: item.kids.unwrap_or_default(),
            score: item.score.unwrap_or_default(),
            created_at: item.created_at,
            title: item.title.unwrap_or_default(),
            url: item.url.unwrap_or_default(),
        }
    }
}
