//! User interactions, models, and endpoints for the Hacker News API.

use std::rc::Rc;

use crate::errors::HackerNewsResult;

use super::HackerNewsUser;

const USER_ENDPOINT: &str = "user";

/// An internal users client for interacting with user endpoints.
#[derive(Debug)]
pub struct HackerNewsUserClient {
    ref_client: Rc<reqwest::Client>,
    base_url: &'static str,
}

impl HackerNewsUserClient {
    /// Constructs a new instance of the user client from the root HTTP client.
    pub fn new(ref_client: Rc<reqwest::Client>, base_url: &'static str) -> Self {
        Self {
            ref_client,
            base_url,
        }
    }

    /// Retrieves a user from the user endpoint based on the provided username.
    pub async fn get_user(&self, username: &str) -> HackerNewsResult<HackerNewsUser> {
        let user: HackerNewsUser = self
            .ref_client
            .get(format!(
                "{}/{}/{}.json",
                self.base_url, USER_ENDPOINT, username
            ))
            .send()
            .await?
            .json::<HackerNewsUser>()
            .await?;

        Ok(user)
    }
}
