use hacker_rs::client::{ApiVersion, HackerNewsClient};

fn main() {
    // Build your client at the start of your application process
    let client = HackerNewsClient::new();

    // We can also specify the HN client with a specific version
    let versioned_client = HackerNewsClient::new().with_version(ApiVersion::V0);

    println!("Hello, world!");
}
