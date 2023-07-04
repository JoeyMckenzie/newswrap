# hacker-rs

[Hacker News](https://news.ycombinator.com/) API bindings for Rust.

```rust
use hacker_rs::client::HackerNewsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build your client at the start of your application process
    let client = HackerNewsClient::new();

    // Call various endpoints with your client instance
    let first_item = client.get_item(69).await?;
    dbg!(&first_item);

    // Determine what the item type is
    let item_type = first_item.get_item_type();
    dbg!(item_type);

    // Check if the item is job
    assert!(first_item.is_comment());

    // Retrieve user information
    let user = client.get_user("joeymckenzie").await;
    dbg!(user);

    Ok(())
}
```
