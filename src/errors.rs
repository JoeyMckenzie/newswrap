//! Various errors and their conversions for Hacker News API interactions.

use thiserror::Error;

/// Exported types for handling internal errors with the client.
#[derive(Debug, Error)]
pub enum HackerNewsClientError {
    /// Reports errors that occur when making HTTP requests to Hacker News.
    #[error("{0}")]
    RequestError(#[from] reqwest::Error),
    /// Represents an error that occurred while attempting to parse the response into an invalid Hacker News item subtype.
    #[error("The requested item was not a valid {0} type.")]
    InvalidTypeMapping(String),
}
