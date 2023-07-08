//! User interactions, models, and endpoints for the Hacker News API.

use std::rc::Rc;

use crate::{errors::HackerNewsResult, http::InternalHttpClient};

use super::HackerNewsUser;

const USER_ENDPOINT: &str = "user";

/// An internal users client for interacting with user endpoints.
#[derive(Debug)]
pub struct HackerNewsUserClient {
    internal_client: Rc<InternalHttpClient>,
}

impl HackerNewsUserClient {
    /// Constructs a new instance of the user client from the root HTTP client.
    pub fn new(internal_client: Rc<InternalHttpClient>) -> Self {
        Self { internal_client }
    }

    /// Retrieves a user from the user endpoint based on the provided username.
    pub async fn get_user(&self, username: &str) -> HackerNewsResult<HackerNewsUser> {
        let user: HackerNewsUser = self
            .internal_client
            .get_item_with_id(USER_ENDPOINT, username)
            .await?;

        Ok(user)
    }
}
