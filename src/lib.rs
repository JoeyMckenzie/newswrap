//! [Hacker News](https://news.ycombinator.com/) API bindings for Rust.
//!
//! ```
//! use newswrap::{client::HackerNewsClient, errors::HackerNewsClientError};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), HackerNewsClientError> {
//!     // Build your client at the start of your application process
//!     let client = HackerNewsClient::new();
//!
//!     // Optionally build your client with a configured request timeout
//!     let _client_with_timeout = HackerNewsClient::new_with_timeout(2);
//!
//!     // Call various endpoints with your client instance
//!     let unknown_item = client.get_item(69).await?;
//!     dbg!(&unknown_item);
//!
//!     // Determine what the item type is
//!     let item_type = unknown_item.get_item_type();
//!     dbg!(item_type);
//!
//!     // Check if the item is job
//!     assert!(unknown_item.is_story());
//!
//!     // Conveniently request typed items for known IDs
//!     let story_item = client.get_story(69).await?;
//!     dbg!(&story_item);
//!
//!     // Get child comments associated to the story
//!     let comment = client
//!         .get_item(*story_item.comments.first().unwrap())
//!         .await?;
//!     dbg!(comment);
//!
//!     // Retrieve user information
//!     let user = client.get_user("joeymckenzie").await?;
//!     dbg!(user);
//!
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code, dead_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts,
    clippy::single_char_pattern
)]

pub mod client;
pub mod errors;
pub mod items;
pub mod users;

/// The ID associated to all Hacker News items and users.
type HackerNewsID = u32;

/// Current URL of the API.
pub const API_BASE_URL: &str = "https://hacker-news.firebaseio.com";

/// Item endpoint for stories, polls, news items, etc.
pub const ITEM_ENDPOINT: &str = "item";

/// User endpoint for user data and related items.
pub const USERS_ENDPOINT: &str = "user";

/// Default timeout for requests the API.
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 10;
