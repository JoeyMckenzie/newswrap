use newswrap::{client::HackerNewsClient, errors::HackerNewsClientError};

#[tokio::main]
async fn main() -> Result<(), HackerNewsClientError> {
    // Build your client at the start of your application process
    let client = HackerNewsClient::new();

    // Check for the latest item ID
    let latest_id = client.realtime.get_latest_item_id().await?;
    dbg!(latest_id);

    // Get the latest stories IDs
    let latest_stories = client.realtime.get_latest_stories().await?;
    dbg!(latest_stories);

    Ok(())
}
