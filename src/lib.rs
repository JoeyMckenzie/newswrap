//! [Hacker News](https://news.ycombinator.com/) API bindings for Rust with full async support.
//!
//! ```
//! use newswrap::{client::HackerNewsClient, errors::HackerNewsClientError};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), HackerNewsClientError> {
//!     // Build your client at the start of your application process
//!     let client = HackerNewsClient::new();
//!
//!     // Call various endpoints with your client instance
//!     let generic_item = client.items.get_item(69).await?;
//!     dbg!(&generic_item);
//!
//!     // Determine what the item type is
//!     let item_type = generic_item.get_item_type();
//!     dbg!(item_type);
//!
//!     // Check if the item is job
//!     assert!(generic_item.is_story());
//!
//!     // Retrieve user information
//!     let user = client.users.get_user("joeymckenzie").await?;
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
mod http;
pub mod items;
pub mod realtime;
pub mod users;

/// The ID associated to all Hacker News items and users.
pub type HackerNewsID = u32;
