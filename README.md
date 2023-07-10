# Newswrap

[![CI](https://github.com/JoeyMckenzie/newswrap/actions/workflows/build-ci.yml/badge.svg)](https://github.com/JoeyMckenzie/newswrap/actions/workflows/build-ci.yml) [![crates.io](https://github.com/JoeyMckenzie/newswrap/actions/workflows/publish-crate.yml/badge.svg?branch=main)](https://github.com/JoeyMckenzie/newswrap/actions/workflows/publish-crate.yml)

[Hacker News](https://news.ycombinator.com/) API bindings for Rust.

```rust
use newswrap::client::HackerNewsClient;

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

## What is newswrap?

Newswrap provides a convenient Rust interface for the Hacker News API. [Hacker News](https://news.ycombinator.com/) is a
community-driven website targeted at software developers and technology professionals. While Hacker News offers up its
[API](https://github.com/HackerNews/API) for the general public, there are no official language-based libraries for connecting
to it. This project aims to provide an easy-to-use Rust-based client for retrieving data from Hacker News in an idiomatic fashion
with first-class async support.

## Using newswrap

To get started with newswrap within your Rust application, simply add the package,

```bash
cargo add newswrap
```

and spin up the client at the start of your application process:

```rust
use newswrap::{client::HackerNewsClient, errors::HackerNewsClientError};

#[tokio::main]
async fn main() -> Result<(), HackerNewsClientError> {
    // Build your client at the start of your application process
    let client = HackerNewsClient::new();

    // Check for the latest item ID
    let latest_id = client.realtime.get_latest_item_id().await?;

    // Get the latest stories IDs
    let story = client.items.get_story(69).await?;
    println!("{}... nice.", story.title);

    // Retrieve user profiles and information
    let my_profile = client.users.get_user("joeymckenzie").await?;

    if let Some(about_section) = my_profile.about {
        println!("{}", about_section);
    }

    Ok(())
}

```

Under the hood, newswrap relies on [reqwest](https://docs.rs/reqwest/latest/reqwest/) for collecting information from the Hacker News API via HTTP. It's advised for consumers of the newswrap client to instantiate a single instance at the start of your application process. [Examples](https://github.com/JoeyMckenzie/newswrap/tree/main/examples) are available for using clients in binary applications and web applications (with axum).
