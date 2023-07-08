//! Hacker News API client bindings and various methods for interacting. The client is an async-first HTTP client and
//! application authors should expect to bring an async runtime of their choosing. There are currently no plans to provide
//! a blocking API, though this may change in future releases.

use std::{rc::Rc, time::Duration};

use crate::{
    http::InternalHttpClient, items::client::HackerNewsItemClient,
    realtime::client::HackerNewsRealtimeClient, users::client::HackerNewsUserClient,
};

/// Version information for the Hacker News API containing the base URLs.
#[derive(Debug, Clone, Copy)]
pub enum ApiVersion {
    /// Represents version 0 of the Hacker News API.
    V0,
}

/// All outgoing requests will have a user-agent associated to newswrap for request visibility.
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Current URL of the API.
const API_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

/// Default timeout for requests the API.
const DEFAULT_TIMEOUT_SECONDS: u64 = 10;

/// A wrapping HTTP client for Hacker News Firebase API and real-time data.
/// A client instance should only be instantiated once in an application's
/// lifecycle, seeking to reuse it where possible. Clients can be configured
/// with requests timeouts in seconds, defaulting to 10 seconds.
///
/// ```
/// use newswrap::client::HackerNewsClient;
///
/// // Default client with 10 second timeout
/// let default_timeout_client = HackerNewsClient::new();
///
/// // Or, with an optional timeout
/// let custom_client = HackerNewsClient::new_with_timeout_secs(10);
/// let custom_client_with_duration = HackerNewsClient::new_with_timeout_duration(std::time::Duration::from_millis(400));
/// ```
#[derive(Debug)]
pub struct HackerNewsClient {
    /// An internal item client for interacting with items.
    pub items: HackerNewsItemClient,
    /// An internal user client for interacting with users.
    pub users: HackerNewsUserClient,
    /// An internal realtime client for interacting with live data.
    pub realtime: HackerNewsRealtimeClient,
    /// The internal version of the Hacker News API your client will target.
    pub version: ApiVersion,
}

impl Default for HackerNewsClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HackerNewsClient {
    /// Internally constructs the client allowing for flexibility in configuring the timeout.
    fn new_client(timeout: std::time::Duration) -> Self {
        let client = reqwest::ClientBuilder::new()
            .timeout(timeout)
            .user_agent(USER_AGENT)
            .build()
            // TODO: probably move this to a builder of sorts, if this panics we have much bigger problems
            .unwrap();
        let internal_client = InternalHttpClient::new(client, API_BASE_URL);
        let ref_client = Rc::new(internal_client);
        let item_client = HackerNewsItemClient::new(ref_client.clone());
        let user_client: HackerNewsUserClient = HackerNewsUserClient::new(ref_client.clone());
        let realtime_client = HackerNewsRealtimeClient::new(ref_client);

        Self {
            items: item_client,
            users: user_client,
            realtime: realtime_client,
            version: ApiVersion::V0,
        }
    }

    /// Constructs a new client pointing to the latest Hacker News API version.
    pub fn new() -> Self {
        let duration = std::time::Duration::from_secs(DEFAULT_TIMEOUT_SECONDS);
        Self::new_client(duration)
    }

    /// Constructs a new client pointing to the latest Hacker News API version with the configured request timeout in seconds.
    pub fn new_with_timeout_secs(timeout: u64) -> Self {
        let duration = std::time::Duration::from_secs(timeout);
        Self::new_client(duration)
    }

    /// Constructs a new client pointing to the latest Hacker News API version with the configured request timeout duration.
    pub fn new_with_timeout_duration(duration: Duration) -> Self {
        Self::new_client(duration)
    }
}
