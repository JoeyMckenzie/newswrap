use crate::client::{ApiVersion, HackerNewsClient};

/// A fluent builder for configuring a client with custom options.
#[derive(Debug)]
pub struct HackerNewsClientBuilder {
    /// The internal client to be exposed once options have been determined.
    client: HackerNewsClient,
}

impl HackerNewsClientBuilder {
    /// Constructs a default instance of the Hacker News client targeting the latest version of the API.
    pub fn new() -> Self {
        Self {
            client: HackerNewsClient::new(),
        }
    }

    /// Sets the version to be used for the Hacker News client.
    pub fn with_version(&self, version: ApiVersion) -> Self {
        Self {
            client: HackerNewsClient {
                version,
                http_client: self.client.clone_client(),
            },
        }
    }

    /// Consumes the builder in place of the configured Hacker News client.
    pub fn build(self) -> HackerNewsClient {
        self.client
    }
}
