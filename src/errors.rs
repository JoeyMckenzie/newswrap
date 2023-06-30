//! Various errors and their conversions for Hacker News API interactions.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("{0}")]
    RequestError(#[from] reqwest::Error),
}
