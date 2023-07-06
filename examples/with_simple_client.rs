use hacker_rs::{client::HackerNewsClient, errors::HackerNewsClientError};

#[tokio::main]
async fn main() -> Result<(), HackerNewsClientError> {
    // Build your client at the start of your application process
    let client = HackerNewsClient::new();

    // Optionally build your client with a configured request timeout
    let _client_with_timeout = HackerNewsClient::new_with_timeout(2);

    // Call various endpoints with your client instance
    let first_item = client.get_item(69).await?;
    dbg!(&first_item);

    // Determine what the item type is
    let item_type = first_item.get_item_type();
    dbg!(item_type);

    // Check if the item is job
    assert!(first_item.is_story());

    // Retrieve user information
    let user = client.get_user("joeymckenzie").await?;
    dbg!(user);

    Ok(())
}
