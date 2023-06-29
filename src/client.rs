//! Hacker News API client bindings and various methods for interacting.

const API_BASE_URL: &str = "https://hacker-news.firebaseio.com";

/// Version information for the Hacker News API containing the base URLs.
#[derive(Debug, Clone, Copy)]
pub enum ApiVersion {
    /// Represents version 0 of the Hacker News API.
    V0,
}

/// A wrapping HTTP client for Hacker News Firebase API and real-time data.
/// A client instance should only be instantiated once in an application's
/// lifecycle, seeking to reuse it where possible.
#[derive(Debug, Clone, Copy)]
pub struct HackerNewsClient {
    /// The internal version of the Hacker News API your client will target.
    version: ApiVersion,
}

impl Default for HackerNewsClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HackerNewsClient {
    /// Constructs a new implementation of the client.
    pub fn new() -> Self {
        Self {
            version: ApiVersion::V0,
        }
    }

    /// Sets the API version to be used.
    pub fn with_version(&self, version: ApiVersion) -> Self {
        Self { version }
    }

    fn versioned_api_base_url(&self) -> String {
        let version = match self.version {
            ApiVersion::V0 => "v0",
        };

        format!("{}/{}", API_BASE_URL, version)
    }
}
