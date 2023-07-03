//! Various errors and their conversions for Hacker News API interactions.

use thiserror::Error;

/// Exported types for handling internal errors with the client.
#[derive(Debug, Error)]
pub enum HackerNewsClientError {
    /// Reports errors that occur when making HTTP requests to Hacker News.
    #[error("{0}")]
    RequestError(#[from] reqwest::Error),
}
