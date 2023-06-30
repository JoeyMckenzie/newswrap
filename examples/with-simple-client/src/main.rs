use hacker_rs::client::HackerNewsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build your client at the start of your application process
    let client = HackerNewsClient::new();

    // Call various endpoints with your client instance
    let first_item = client.get_item(12).await?;

    Ok(())
}
