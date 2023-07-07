//! Various errors and their conversions for Hacker News API interactions.

use std::{convert::Infallible, num::ParseIntError};

use thiserror::Error;

use crate::{
    items::{HackerNewsItem, HackerNewsItemType},
    HackerNewsID,
};

/// A generic result type mapping downstream errors to their library equivalents.
pub type HackerNewsResult<T> = Result<T, HackerNewsClientError>;

/// Exported types for handling internal errors with the client.
#[derive(Debug, Error)]
pub enum HackerNewsClientError {
    /// Reports errors that occur when making HTTP requests to Hacker News.
    #[error("{0}")]
    RequestError(#[from] reqwest::Error),
    /// Represents an error that occurred while attempting to parse the response into an invalid Hacker News item subtype.
    #[error("The requested item was not a valid {0} type.")]
    InvalidTypeMapping(HackerNewsItemType),
    /// Represents an error on an item missing a valid parent item.
    #[error("An associated parent was not found for item {0}")]
    AssociatedParentNotFound(HackerNewsID),
    /// Represents an item that failed to identify as one of the known Hacker News item types.
    #[error("An implicit conversion could not be performed for the item {0}.")]
    ImplicitConversionError(HackerNewsItemType),
    /// Represents a request to an endpoint returning an ID that failed to parse.
    #[error("Could not convert the response into a valid ID.")]
    InvalidIdentifier(#[from] ParseIntError),
    /// Represents a seemingly infallible operation that has occurred.
    #[error(
        "An infallible operation has occurred. If you're seeing this, please report an issue!"
    )]
    InfallibleOperation,
}

/// Conveniently allows for the result to convert between Hacker News items and their subtypes for client requests.
impl<T> From<HackerNewsItem> for HackerNewsResult<T> {
    fn from(value: HackerNewsItem) -> Self {
        Self::Err(HackerNewsClientError::ImplicitConversionError(
            value.get_item_type(),
        ))
    }
}

/// Allows for conversion between the generic result type and the client errors.
impl From<Infallible> for HackerNewsClientError {
    fn from(_: Infallible) -> Self {
        Self::InfallibleOperation
    }
}
