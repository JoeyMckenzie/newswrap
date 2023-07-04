//! [Hacker News](https://news.ycombinator.com/) API bindings for Rust.
//!
//! ```no_run
//! use hacker_rs::client::HackerNewsClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Build your client at the start of your application process
//!     let client = HackerNewsClient::new();
//!
//!     // Call various endpoints with your client instance
//!     let first_item = client.get_item(69).await?;
//!     dbg!(&first_item);
//!
//!     // Determine what the item type is
//!     let item_type = first_item.get_item_type();
//!     dbg!(item_type);
//!
//!     // Check if the item is job
//!     assert!(first_item.is_comment());
//!
//!     // Retrieve user information
//!     let user = client.get_user("joeymckenzie").await;
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
