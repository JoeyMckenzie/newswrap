use newswrap::{client::HackerNewsClient, errors::HackerNewsClientError};

#[tokio::main]
async fn main() -> Result<(), HackerNewsClientError> {
    // Build your client at the start of your application process
    let client = HackerNewsClient::new();

    // Check for the latest item ID
    let latest_id = client.realtime.get_latest_item_id().await?;
    dbg!(latest_id);

    Ok(())
}
